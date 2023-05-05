use crate::{parse_file, MetaFile, RootDirs, Source, Substitution};
use color_eyre::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn build_metafile(file: &MetaFile, dirs: &RootDirs, path: &Path) -> Result<()> {
    Ok(std::fs::write(
        find_dest(path, dirs)?,
        metafile_to_string(file, dirs, None)?,
    )?)
}

fn metafile_to_string(file: &MetaFile, dirs: &RootDirs, name: Option<&str>) -> Result<String> {
    let mut output = String::default();

    for section in file.source.iter() {
        match section {
            // concatenate any char sequences
            Source::Str(str) => output.push_str(str),
            // expand all variables and recursively expand patterns
            Source::Sub(sub) => {
                let expanded = match sub {
                    Substitution::Variable(key) => file
                        .get_var(key)
                        .map(|val| val.to_string())
                        .unwrap_or_default(),
                    Substitution::Pattern(key) => get_pattern(key, file, dirs)?,
                    // comments have already been removed at this point,
                    // so we use them to mark keys for array substitution
                    Substitution::Array(key) => format!("-{{{key}}}"),
                };
                output.push_str(&expanded);
            }
        }
    }

    // deal with arrays
    Ok(expand_arrays(output, file, name)?)
}

fn get_pattern(key: &str, file: &MetaFile, dirs: &RootDirs) -> Result<String> {
    let filename = match file.get_pat(key) {
        Some(file) => file,
        None => "default",
    };

    let pattern_path = key.replace('.', "/") + "/" + filename;
    let mut path = dirs.pattern.join(pattern_path).canonicalize()?;
    path.set_extension(".meta");
    let pattern = parse_file(path.to_str().unwrap_or_default())?;
    metafile_to_string(&pattern, dirs, Some(key))
}

fn find_dest(path: &Path, dirs: &RootDirs) -> Result<PathBuf> {
    let path = path.to_string_lossy().to_string().replace(
        dirs.source.to_str().unwrap_or_default(),
        dirs.build.to_str().unwrap_or_default(),
    );

    Ok(PathBuf::from(path).canonicalize()?)
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
        // make a hash map of keys in the source to previously defined arrays
        .map(|array| {
            let key = name.unwrap_or_default().to_owned() + "." + array;
            let value = file.get_arr(&key).unwrap_or_default();
            (*array, value)
        })
        .collect();

    let mut expanded = String::new();
    // loop to duplicate the output template for each array member
    for i in 0.. {
        // get a fresh copy of the file
        let mut str = output.clone();
        // replace each key in the file
        for (key, val) in map.iter() {
            str = str.replace(&format!("-{{{key}}}"), val[i]);
        }
        // concatenate to final file
        expanded.push_str(&str);
    }

    Ok(expanded)
}
