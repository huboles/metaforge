use crate::{parse_file, MetaFile, Src, Sub};
use color_eyre::{eyre::bail, Result};
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOutput};
use std::{collections::HashMap, fs};

pub fn build_metafile(file: &MetaFile) -> Result<String> {
    if file.header.blank {
        return Ok(String::new());
    }

    let html = get_source_html(file)?;

    let pattern = get_pattern("base", file)?;
    let mut base = parse_file(pattern, file.opts)?;

    base.merge(file);
    base.patterns.insert("SOURCE".to_string(), html);

    let output = metafile_to_string(&base)?;

    Ok(output)
}

fn metafile_to_string(file: &MetaFile) -> Result<String> {
    if file.header.blank {
        return Ok(String::new());
    }

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
        expand_arrays(output, file)
    } else {
        Ok(output)
    }
}

fn get_source_html(file: &MetaFile) -> Result<String> {
    let string = metafile_to_string(file)?;

    if file.opts.no_pandoc || !file.header.pandoc {
        return Ok(string);
    }

    let mut pandoc = Pandoc::new();
    pandoc
        .set_input(InputKind::Pipe(string))
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
    if key == "SOURCE" {
        if let Some(source) = file.patterns.get("SOURCE") {
            return Ok(source.to_string());
        }
    }

    let mut filename: String;
    if let Some(name) = file.get_pat(key) {
        filename = name.to_string();
    } else {
        // anything not defined should have a default.meta file to fall back to
        filename = "default".to_string()
    }

    // BLANK returns nothing, so no more processing needs to be done
    if filename == "BLANK" {
        return Ok(String::from(""));
    };

    // DEFAULT override for patterns overriding globals
    if filename == "DEFAULT" {
        filename = "default".to_string();
    }

    // if we're building from base pattern we need to wait on
    // parsing/expansion so we can build and convert source to html
    // we just want to return the string right now
    if key == "base" {
        let pattern_path = key.to_string() + "/" + &filename;
        let mut path = file.opts.pattern.join(pattern_path);
        path.set_extension("meta");

        return match fs::read_to_string(&path) {
            Ok(str) => Ok(str),
            Err(_) => bail!("could not find base file {}", path.display()),
        };
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
    use crate::Options;
    use std::path::PathBuf;

    fn unit_test(test: &str, result: &str) -> Result<()> {
        let dir = PathBuf::from("files/test_site").canonicalize()?;

        let mut opts = Options::new();
        opts.root = dir.clone();
        opts.source = dir.join("source");
        opts.build = dir.join("build");
        opts.pattern = dir.join("pattern");
        opts.clean = true;

        let test_dir = opts.source.join("unit_tests");
        let mut file_path = test_dir.join(test);
        file_path.set_extension("meta");
        let file = MetaFile::build(file_path, &opts)?;

        let output = build_metafile(&file)?;

        assert_eq!(output, result);

        Ok(())
    }

    #[test]
    fn test_find_dest() -> Result<()> {
        unit_test("find_dest", "<html>\n\n</html>\n")
    }

    #[test]
    fn test_blank() -> Result<()> {
        unit_test("blank/blank_pattern", "")?;
        unit_test("blank/blank_variable", "<html>\n</html>\n")?;
        unit_test("blank/blank_array", "<html>\n</html>\n")?;
        Ok(())
    }

    #[test]
    fn test_comment() -> Result<()> {
        unit_test("blank/comment", "<html>\n\n</html>\n")?;
        unit_test(
            "blank/inline_comment",
            "<html>\n<p>inline comment</p>\n</html>\n",
        )?;
        Ok(())
    }

    #[test]
    fn test_expand() -> Result<()> {
        unit_test(
            "expand/variable_in_source",
            "<html>\n<p>GOOD</p>\n</html>\n",
        )?;
        unit_test("expand/variable_in_pattern", "<html>\nGOOD</html>\n")?;
        unit_test("expand/array_in_source", "<html>\n<p>12345</p>\n</html>\n")?;
        unit_test("expand/array_in_pattern", "<html>\n12345</html>\n")?;
        unit_test("expand/pattern_in_source", "<p>GOOD</p>\n")?;
        unit_test("expand/pattern_in_pattern", "<html>\nGOOD\nGOOD\n</html>\n")?;
        Ok(())
    }

    #[test]
    fn test_override() -> Result<()> {
        unit_test("override/variable", "<html>\n<p>GOOD</p>\n</html>\n")?;
        unit_test("override/pattern", "<html>\nGOOD\nGOOD\n</html>\n")?;
        Ok(())
    }

    #[test]
    #[ignore = "fix global variables"]
    fn test_global() -> Result<()> {
        unit_test("global/variable", "GOODGOOD\n")?;
        unit_test("global/pattern", "GOODGOOD")?;
        Ok(())
    }
}
