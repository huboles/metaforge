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
use walkdir::WalkDir;

pub fn build_site(opts: &Options) -> Result<()> {
    log!(opts, "finding files", 2);
    let files: Vec<PathBuf> = WalkDir::new(&opts.source)
        .into_iter()
        .filter_map(|file| {
            if file.as_ref().ok()?.file_type().is_dir() {
                // need to create directories in build dir
                let path = file.unwrap().into_path();
                let path = path.strip_prefix(&opts.source).ok()?;
                let path = opts.build.join(path);
                log!(opts, format!("\tcreating dir: {}", path.display()), 3);
                std::fs::create_dir(path).ok()?;
                // don't need them for any further operations so we filter them out
                None
            } else if let Ok(file) = file {
                log!(opts, format!("\tadding file: {}", file.path().display()), 3);
                Some(file.into_path())
            } else {
                None
            }
        })
        .collect();

    log!(opts, "building files", 2);
    for file in files.iter() {
        match build_metafile(file, opts) {
            Ok(_) => continue,
            Err(e) => {
                if opts.force {
                    // print a line to stderr about failure but continue with other files
                    eprintln!("{}: {}", file.display(), e);
                    continue;
                } else {
                    return Err(e.wrap_err(eyre!("{}:", file.display())));
                }
            }
        }
    }

    Ok(())
}

fn build_metafile(path: &Path, opts: &Options) -> Result<()> {
    log!(opts, format!("\t{}", path.display()), 1);
    let file =
        fs::read_to_string(path).wrap_err_with(|| eyre!("failed to read: {}\n", path.display()))?;

    log!(opts, "\tparsing", 2);
    let file =
        parse_file(&file).wrap_err_with(|| eyre!("failed to parse: {}\n", path.display()))?;

    let html = get_source_html(&file, opts)
        .wrap_err_with(|| eyre!("failed converting to html: {}\n", path.display()))?;

    let pattern = get_pattern("base", &file, opts).wrap_err("failed to get base pattern\n")?;
    let mut base = parse_file(&pattern).wrap_err("failed to parse base pattern\n")?;

    base.variables = file.variables;
    base.arrays = file.arrays;
    base.patterns = file.patterns;

    base.patterns.insert("SOURCE", &html);

    let output = metafile_to_string(&base, opts, Some("base"))
        .wrap_err_with(|| eyre!("failed to build: {}\n", path.display()))?;

    log!(opts, "\twriting", 2);
    let dest = find_dest(path, opts)
        .wrap_err_with(|| format!("could not find destination file: {}\n", path.display()))?;

    // want newline to end file
    fs::write(&dest, output + "\n")
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
    let mut pandoc = Pandoc::new();

    log!(opts, "\t\tsetting up pandoc", 3);
    pandoc
        .set_input(InputKind::Pipe(file))
        .set_output(OutputKind::Pipe)
        .set_input_format(InputFormat::Markdown, vec![])
        .set_output_format(OutputFormat::Html, vec![]);

    log!(opts, "\t\texecuting pandoc command", 3);
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
        let source = file.patterns.get("SOURCE").unwrap_or(&"");
        return Ok(source.to_string());
    }

    log!(opts, format!("\t\tpattern: {}", key), 3);
    // anything not defined should have a default.meta file to fall back to
    let mut filename = file.get_pat(key).unwrap_or("default");

    // if we're building from base pattern we need to wait on
    // parsing/expansion so we can build and convert source to html
    // we just want to return the string right now
    if key == "base" {
        log!(opts, "\t\t\treturning base", 4);
        let pattern_path = key.to_string() + "/" + filename;
        let mut path = opts.pattern.join(pattern_path);
        path.set_extension("meta");

        let base = fs::read_to_string(&path)
            .wrap_err_with(|| eyre!("could not read: {}\n", path.display()))?;
        return Ok(base);
    }

    // BLANK returns nothing, so no more processing needs to be done
    if filename == "BLANK" {
        log!(opts, format!("\t\t\treturning blank: {}", key), 4);
        return Ok(String::new());
    };

    // DEFAULT override for variables defined higher in chain
    if filename == "DEFAULT" {
        log!(opts, "\t\t\tdefault pattern", 4);
        filename = "default";
    }

    log!(opts, "\t\t\tbuilding path from key", 4);
    let pattern_path = key.replace('.', "/") + "/" + filename;
    let mut path = opts.pattern.join(pattern_path);
    path.set_extension("meta");

    log!(opts, "\t\t\tparsing file", 4);
    let pattern = &fs::read_to_string(&path)
        .wrap_err_with(|| eyre!("could not read: {}\n", path.display()))?;
    let mut pattern =
        parse_file(pattern).wrap_err_with(|| eyre!("could not parse: {}\n", path.display()))?;

    // copy over maps for expanding contained variables
    // TODO: Make this a merge so patterns can define/override their own variables
    pattern.variables = file.variables.clone();
    pattern.arrays = file.arrays.clone();
    pattern.patterns = file.patterns.clone();

    log!(opts, "\t\t\tbuilding pattern", 4);
    metafile_to_string(&pattern, opts, Some(key))
}

fn find_dest(path: &Path, opts: &Options) -> Result<PathBuf> {
    log!(opts, "\t\tfinding destination", 3);
    let source = opts.source.to_string_lossy();
    let build = opts.build.to_string_lossy();

    let path = path
        .canonicalize()
        .wrap_err_with(|| eyre!("could not get absolute path: {}\n", path.display()))?;
    let path = path.to_string_lossy();
    let path = path.replace(&*source, &build);
    let mut path = PathBuf::from(path);

    path.set_extension("html");

    Ok(path)
}

fn expand_arrays(output: String, file: &MetaFile, name: Option<&str>) -> Result<String> {
    let map: HashMap<&str, &[&str]> = file
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
            (*array, value)
        })
        .collect();

    let mut expanded = String::new();
    // loop to duplicate the output template for each array member
    for i in 0..get_max_size(&map) {
        // get a fresh copy of the file
        let mut str = output.clone();
        // replace each key in the file
        for (key, val) in map.iter() {
            str = str.replace(&format!("-{{{key}}}"), val.get(i).unwrap_or(&""));
        }
        // concatenate to final file
        expanded.push_str(&str);
    }

    Ok(expanded)
}

fn get_max_size(map: &HashMap<&str, &[&str]>) -> usize {
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

        let opts = Options {
            root: dir.clone(),
            source: dir.join("source"),
            build: dir.join("build"),
            pattern: dir.join("pattern"),
            verbose: 0,
            quiet: false,
            force: false,
            undefined: false,
            clean: true,
        };

        Ok(opts)
    }

    #[test]
    fn test_find_dest() -> Result<()> {
        let opts = build_options()?;
        let path = opts.source.join("dir1/dir.meta");
        assert_eq!(find_dest(&path, &opts)?, opts.build.join("dir1/dir.html"));
        Ok(())
    }
}
