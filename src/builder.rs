use crate::{log, parse_file, MetaFile, Options, Src, Sub};
use color_eyre::{eyre::bail, Result};
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOutput};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub fn build_metafile(file: &MetaFile) -> Result<String> {
    let html = get_source_html(file, file.opts)?;

    let pattern = get_pattern("base", file)?;
    let mut base = parse_file(pattern, file.opts)?;

    base.merge(file);
    base.patterns.insert("SOURCE".to_string(), html);

    let output = metafile_to_string(&base)?;

    Ok(output)
}

pub fn write_file(path: &Path, html: String, opts: &Options) -> Result<()> {
    let dest = find_dest(path, opts)?;
    // want newline to end file
    fs::write(dest, html)?;
    Ok(())
}

fn metafile_to_string(file: &MetaFile) -> Result<String> {
    let mut output = String::default();
    let mut arrays = false;

    for section in file.source.iter() {
        match section {
            // concatenate any char sequences
            Src::Str(str) => {
                output.push_str(str);
            }
            // expand all variables and recursively expand patterns
            Src::Sub(sub) => {
                let expanded = match sub {
                    Sub::Var(key) => get_variable(key, file)?,
                    Sub::Pat(key) => get_pattern(key, file)?,
                    Sub::Arr(key) => {
                        arrays = true;
                        // comments have already been removed at this point,
                        // so we use them to mark keys for array substitution
                        format!("-{{{key}}}")
                    }
                };
                output.push_str(&expanded);
            }
        }
    }

    if arrays {
        log!(file.opts, "\t\t\texpanding arrays", 4);
        expand_arrays(output, file)
    } else {
        Ok(output)
    }
}

fn get_source_html(file: &MetaFile, opts: &Options) -> Result<String> {
    log!(opts, "\tbuilding source", 2);
    let file = metafile_to_string(file)?;

    if opts.no_pandoc {
        return Ok(file);
    }

    log!(opts, "\t\tcalling pandoc", 3);
    let mut pandoc = Pandoc::new();
    pandoc
        .set_input(InputKind::Pipe(file))
        .set_output(OutputKind::Pipe)
        .set_input_format(InputFormat::Markdown, vec![])
        .set_output_format(OutputFormat::Html, vec![]);

    if let Ok(PandocOutput::ToBuffer(html)) = pandoc.execute() {
        Ok(html)
    } else {
        bail!("pandoc could not write to buffer")
    }
}

fn get_pattern(key: &str, file: &MetaFile) -> Result<String> {
    // SOURCE is already expanded in the initial build_metafile() call
    // we just need to return that
    if key == "SOURCE" {
        log!(file.opts, "\t\t\treturning SOURCE", 4);
        if let Some(source) = file.patterns.get("SOURCE") {
            return Ok(source.to_string());
        }
    }

    log!(file.opts, format!("\t\tpattern: {}", key), 3);
    // anything not defined should have a default.meta file to fall back to
    let mut filename: String;
    if let Some(name) = file.get_pat(key) {
        filename = name.to_string();
    } else {
        filename = "default".to_string()
    }

    // if we're building from base pattern we need to wait on
    // parsing/expansion so we can build and convert source to html
    // we just want to return the string right now
    if key == "base" {
        log!(file.opts, "\t\t\treturning base", 4);
        let pattern_path = key.to_string() + "/" + &filename;
        let mut path = file.opts.pattern.join(pattern_path);
        path.set_extension("meta");

        let base = fs::read_to_string(&path)?;
        return Ok(base);
    }

    // BLANK returns nothing, so no more processing needs to be done
    if filename == "BLANK" {
        return Ok(String::from(""));
    };

    // DEFAULT override for patterns defined higher in chain
    if filename == "DEFAULT" {
        filename = "default".to_string();
    }

    let pattern_path = key.replace('.', "/") + "/" + &filename;
    let mut path = file.opts.pattern.join(pattern_path);
    path.set_extension("meta");

    let mut pattern = MetaFile::build(path, file.opts)?;

    // copy over maps for expanding contained variables
    pattern.merge(file);

    metafile_to_string(&pattern)
}

fn get_variable(key: &str, file: &MetaFile) -> Result<String> {
    let long_key = file.name()? + "." + key;
    if let Some(val) = file.get_var(&long_key) {
        Ok(val.clone())
    } else if let Some(val) = file.get_var(key) {
        Ok(val.clone())
    } else if file.opts.undefined {
        bail!("undefined variable: {}, {}", key, long_key)
    } else {
        Ok(String::new())
    }
}

fn find_dest(path: &Path, opts: &Options) -> Result<PathBuf> {
    let path = path.canonicalize()?;

    let mut path = opts.build.join(path.strip_prefix(&opts.source)?);
    path.set_extension("html");

    Ok(path)
}

fn expand_arrays(input: String, file: &MetaFile) -> Result<String> {
    let map: HashMap<String, &[String]> = file
        .source
        .iter()
        // filter out arrays from source vec
        .filter_map(|x| {
            if let Src::Sub(Sub::Arr(array)) = x {
                Some(array)
            } else {
                None
            }
        })
        // make a hash map of [keys in source] -> [defined arrays]
        .map(|key| {
            // concat array to pattern name to get key in HashMap
            let name = file.name().unwrap();
            let long_key = name + "." + key;

            let value: &[String];
            if let Some(val) = file.get_arr(&long_key) {
                value = val;
            } else if let Some(val) = file.get_arr(key) {
                value = val;
            } else if file.opts.undefined {
                panic!("undefined array called: {}, {}", key, long_key);
            } else {
                value = &[];
            }

            (key.to_string(), value)
        })
        .collect();

    // loop to duplicate the output template for each array member
    let mut expanded = String::new();
    for i in 0..get_max_size(&map) {
        // get a fresh copy of the file
        let mut str = input.clone();
        // replace each key in the file
        for (key, val) in map.iter() {
            if let Some(value) = val.get(i) {
                str = str.replace(&format!("-{{{key}}}"), value);
            }
        }
        // concatenate to final file
        expanded.push_str(&str);
    }

    Ok(expanded)
}

fn get_max_size(map: &HashMap<String, &[String]>) -> usize {
    let mut max = 0;
    for val in map.values() {
        if max < val.len() {
            max = val.len();
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    fn build_options() -> Result<Options> {
        let dir = PathBuf::from("files/site").canonicalize()?;

        let mut opts = Options::new();
        opts.root = dir.clone();
        opts.source = dir.join("source");
        opts.build = dir.join("build");
        opts.pattern = dir.join("pattern");
        opts.clean = true;

        Ok(opts)
    }

    #[test]
    fn test_find_dest() -> Result<()> {
        let opts = build_options()?;
        let path = opts.source.join("dir1/dir.meta");
        assert_eq!(find_dest(&path, &opts)?, opts.build.join("dir1/dir.html"));
        Ok(())
    }

    #[test]
    fn test_metafile_to_string() -> Result<()> {
        let opts = build_options()?;
        let path = opts.source.join("dir1/sub_dir1/deep2/deep.meta");
        let expanded = "<html><body>GOOD</body></html>";
        assert_eq!(build_metafile(&MetaFile::build(path, &opts)?)?, expanded);
        Ok(())
    }

    #[test]
    fn test_get_pattern() -> Result<()> {
        let opts = build_options()?;
        let file = MetaFile::new(&opts);
        let pat = get_pattern("header", &file)?;
        assert_eq!(pat, "<header>HEADER</header>");
        Ok(())
    }
}
