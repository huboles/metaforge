use crate::{log, parse_file, MetaFile, Options, Source, Substitution};
use color_eyre::{
    eyre::bail,
    eyre::{eyre, WrapErr},
    Result,
};
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOutput};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub fn build_metafile(file: &MetaFile, opts: &Options) -> Result<String> {
    let html = get_source_html(file, opts)
        .wrap_err_with(|| eyre!("failed converting to html: {}\n", file.path.display()))?;

    let pattern = get_pattern("base", &file, opts).wrap_err("failed to get base pattern\n")?;
    let mut base = parse_file(pattern).wrap_err("failed to parse base pattern\n")?;

    base.merge(&file);
    base.patterns.insert("SOURCE".to_string(), html);

    let output = metafile_to_string(&base, opts, Some("base"))
        .wrap_err_with(|| eyre!("failed to build: {}\n", file.path.display()))?;

    Ok(output)
}

pub fn write_file(path: &Path, html: String, opts: &Options) -> Result<()> {
    let dest = find_dest(path, opts)?;
    // want newline to end file
    fs::write(&dest, html + "\n")
        .wrap_err_with(|| eyre!("could not write to: {}\n", dest.display()))?;
    Ok(())
}

fn metafile_to_string(file: &MetaFile, opts: &Options, name: Option<&str>) -> Result<String> {
    let mut output = String::default();
    let mut arrays = false;

    for section in file.source.iter() {
        match section {
            // concatenate any char sequences
            Source::Str(str) => {
                output.push_str(str);
            }
            // expand all variables and recursively expand patterns
            Source::Sub(sub) => {
                let expanded = match sub {
                    Substitution::Variable(key) => file
                        .get_var(key)
                        // blank and default dont need to be processed
                        .filter(|val| *val != "BLANK" && *val != "DEFAULT")
                        .map(|val| val.to_string())
                        .unwrap_or_default(),
                    Substitution::Pattern(key) => get_pattern(key, file, opts)
                        .wrap_err_with(|| eyre!("could not find pattern for: {}\n", key))?,
                    // comments have already been removed at this point,
                    // so we use them to mark keys for array substitution
                    Substitution::Array(key) => {
                        arrays = true;
                        format!("-{{{key}}}")
                    }
                };
                output.push_str(&format!("\n{}\n", expanded));
            }
        }
    }

    if arrays {
        log!(opts, "\t\t\texpanding arrays", 4);
        expand_arrays(output, file, name)
    } else {
        Ok(output)
    }
}

fn get_source_html(file: &MetaFile, opts: &Options) -> Result<String> {
    log!(opts, "\tbuilding source", 2);
    let file = metafile_to_string(file, opts, Some("SOURCE")).wrap_err("failed building source")?;

    if opts.no_pandoc {
        return Ok(file);
    }

    log!(opts, "\t\tcalling pandoc", 3);
    log!(opts, "\t\t\tbuilding pandoc command", 4);
    let mut pandoc = Pandoc::new();
    pandoc
        .set_input(InputKind::Pipe(file))
        .set_output(OutputKind::Pipe)
        .set_input_format(InputFormat::Markdown, vec![])
        .set_output_format(OutputFormat::Html, vec![]);

    log!(opts, "\t\t\texecuting pandoc command", 4);
    if let Ok(PandocOutput::ToBuffer(html)) = pandoc.execute() {
        Ok(html)
    } else {
        bail!("pandoc could not write to buffer")
    }
}

fn get_pattern(key: &str, file: &MetaFile, opts: &Options) -> Result<String> {
    // SOURCE is already expanded in the initial build_metafile() call
    // we just need to return that
    if key == "SOURCE" {
        log!(opts, "\t\t\treturning SOURCE", 4);
        if let Some(source) = file.patterns.get("SOURCE") {
            return Ok(source.to_string());
        }
    }

    log!(opts, format!("\t\tpattern: {}", key), 3);
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
        log!(opts, "\t\t\treturning base", 4);
        let pattern_path = key.to_string() + "/" + &filename;
        let mut path = opts.pattern.join(pattern_path);
        path.set_extension("meta");

        let base = fs::read_to_string(&path)
            .wrap_err_with(|| eyre!("base pattern does not exist: {}\n", path.display()))?;
        return Ok(base);
    }

    // BLANK returns nothing, so no more processing needs to be done
    if filename == "BLANK" {
        log!(opts, format!("\t\t\treturning blank: {}", key), 4);
        return Ok(String::new());
    };

    // DEFAULT override for patterns defined higher in chain
    if filename == "DEFAULT" {
        log!(opts, "\t\t\tdefault pattern", 4);
        filename = "default".to_string();
    }

    log!(opts, "\t\t\tbuilding path from key", 4);
    let pattern_path = key.replace('.', "/") + "/" + &filename;
    let mut path = opts.pattern.join(pattern_path);
    path.set_extension("meta");

    log!(opts, "\t\t\tparsing file", 4);
    let mut pattern = MetaFile::build(path)?;

    // copy over maps for expanding contained variables
    pattern.merge(&file);

    log!(opts, "\t\t\tbuilding pattern", 4);
    metafile_to_string(&pattern, opts, Some(key))
}

fn find_dest(path: &Path, opts: &Options) -> Result<PathBuf> {
    let path = path
        .canonicalize()
        .wrap_err_with(|| eyre!("could not get absolute path: {}\n", path.display()))?;

    let path = opts.build.join(path.strip_prefix(&opts.source)?);
    let mut path = PathBuf::from(path);

    path.set_extension("html");

    Ok(path)
}

fn expand_arrays(output: String, file: &MetaFile, name: Option<&str>) -> Result<String> {
    let map: HashMap<String, &[String]> = file
        .source
        .iter()
        // filter out arrays from source vec
        .filter_map(|section| {
            if let Source::Sub(Substitution::Array(array)) = section {
                Some(array)
            } else {
                None
            }
        })
        // make a hash map of [keys in source] -> [defined arrays]
        .map(|array| {
            let key: String;
            // concat array to pattern name to get key in HashMap
            if let Some(name) = name {
                key = name.to_owned() + "." + array;
            } else {
                // keys for arrays in this file don't have a preceding pattern
                key = array.to_string();
            }
            let value = file.get_arr(&key).unwrap_or_default();
            (array.to_string(), value)
        })
        .collect();

    let mut expanded = String::new();
    // loop to duplicate the output template for each array member
    for i in 0..get_max_size(&map) {
        // get a fresh copy of the file
        let mut str = output.clone();
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
    fn test_build_metafile() -> Result<()> {
        let opts = build_options()?;
        let path = opts.source.join("dir1/sub_dir1/deep2/deep.meta");
        let expanded = "<html>\n<body>\nGOOD\n</body>\n</html>\n";
        build_metafile(&MetaFile::build(path)?, &opts)?;
        assert_eq!(
            std::fs::read_to_string(opts.build.join("dir1/sub_dir1/deep2/deep.html"))?,
            expanded
        );
        Ok(())
    }

    #[test]
    fn test_get_pattern() -> Result<()> {
        let opts = build_options()?;
        let file = MetaFile::new();
        let pat = get_pattern("header", &file, &opts)?;
        assert_eq!(pat, "<header>HEADER</header>");
        Ok(())
    }
}
