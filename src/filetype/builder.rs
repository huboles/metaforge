use crate::{parse_file, MetaFile, Options, Source, Substitution};
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

pub fn build_metafile(path: &Path, dirs: &Options) -> Result<()> {
    let file = fs::read_to_string(path)
        .wrap_err_with(|| eyre!("failed to read: {}\n", path.to_string_lossy()))?;
    let file = parse_file(&file)
        .wrap_err_with(|| eyre!("failed to parse: {}\n", path.to_string_lossy()))?;

    let html = get_source_html(&file, dirs)
        .wrap_err_with(|| eyre!("failed converting to html: {}\n", path.to_string_lossy()))?;

    let pattern = get_pattern("base", &file, dirs).wrap_err("failed to get base pattern\n")?;
    let mut base = parse_file(&pattern).wrap_err("failed to parse base pattern\n")?;

    base.variables = file.variables;
    base.arrays = file.arrays;
    base.patterns = file.patterns;

    base.patterns.insert("SOURCE", &html);

    let output = metafile_to_string(&base, dirs, Some("base"))
        .wrap_err_with(|| eyre!("failed to build: {}\n", path.to_string_lossy()))?;

    let dest = find_dest(path, dirs).wrap_err_with(|| {
        format!(
            "could not find destination file: {}\n",
            path.to_string_lossy()
        )
    })?;

    // want newline to end file
    fs::write(&dest, output + "\n")
        .wrap_err_with(|| eyre!("could not write to: {}\n", dest.to_string_lossy()))?;
    Ok(())
}

pub fn metafile_to_string(file: &MetaFile, dirs: &Options, name: Option<&str>) -> Result<String> {
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
                    Substitution::Pattern(key) => get_pattern(key, file, dirs)
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
        expand_arrays(output, file, name)
    } else {
        Ok(output)
    }
}

fn get_source_html(file: &MetaFile, dirs: &Options) -> Result<String> {
    let file = metafile_to_string(file, dirs, Some("SOURCE")).wrap_err("failed building source")?;
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

fn get_pattern(key: &str, file: &MetaFile, dirs: &Options) -> Result<String> {
    // SOURCE is already expanded in the initial build_metafile() call
    // we just need to return that
    if key == "SOURCE" {
        let source = file.patterns.get("SOURCE").unwrap_or(&"");
        return Ok(source.to_string());
    }

    // anything not defined should have a default.meta file to fall back to
    let mut filename = file.get_pat(key).unwrap_or("default");

    // if we're building from base pattern we need to wait on
    // parsing/expansion so we can build and convert source to html
    // we just want to return the string right now
    if key == "base" {
        let pattern_path = key.to_string() + "/" + filename;
        let mut path = dirs.pattern.join(pattern_path);
        path.set_extension("meta");

        let base = fs::read_to_string(&path)
            .wrap_err_with(|| eyre!("could not read: {}\n", path.to_string_lossy()))?;
        return Ok(base);
    }

    // BLANK returns nothing, so no more processing needs to be done
    if filename == "BLANK" {
        return Ok(String::new());
    };

    // DEFAULT override for variables defined higher in chain
    if filename == "DEFAULT" {
        filename = "default";
    }

    let pattern_path = key.replace('.', "/") + "/" + filename;
    let mut path = dirs.pattern.join(pattern_path);
    path.set_extension("meta");

    let pattern = &fs::read_to_string(&path)
        .wrap_err_with(|| eyre!("could not read: {}\n", path.to_string_lossy()))?;
    let mut pattern = parse_file(pattern)
        .wrap_err_with(|| eyre!("could not parse: {}\n", path.to_string_lossy()))?;

    // copy over maps for expanding contained variables
    pattern.variables = file.variables.clone();
    pattern.arrays = file.arrays.clone();
    pattern.patterns = file.patterns.clone();

    metafile_to_string(&pattern, dirs, Some(key))
}

fn find_dest(path: &Path, dirs: &Options) -> Result<PathBuf> {
    let source = dirs.source.to_string_lossy();
    let build = dirs.build.to_string_lossy();

    let path = path
        .canonicalize()
        .wrap_err_with(|| eyre!("could not get absolute path: {}\n", path.to_string_lossy()))?;
    let path = path.to_string_lossy();
    let path = path.replace(&*source, &*build);
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
