use crate::{MetaFile, Scope};
use color_eyre::{eyre::bail, Result};
use std::fs;

pub fn get_pattern(key: &str, file: &MetaFile) -> Result<String> {
    // SOURCE is already expanded in the initial build_metafile() call
    if key == "SOURCE" {
        if let Some(source) = file.patterns.get(&Scope::into_global("SOURCE")) {
            return Ok(source.to_string());
        } else {
            return Ok(String::new());
        }
    }

    let mut filename = if let Some(name) = file.get_pat(&Scope::into_local(key)) {
        name.to_string()
    } else if let Some(name) = file.get_pat(&Scope::into_global(key)) {
        name.to_string()
    } else {
        // anything not defined should have a default.meta file to fall back to
        "default".to_string()
    };

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

    super::metafile_to_string(&pattern)
}
