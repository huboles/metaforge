use crate::{log, MetaError, MetaFile, Scope, Src};
use eyre::Result;
use std::collections::HashMap;

pub fn expand_arrays(input: String, file: &MetaFile) -> Result<String> {
    log!(
        file.opts,
        format!("expanding arrays in {}", file.path.display()),
        2
    );

    let map: HashMap<String, &[String]> = file
        .source
        .iter()
        // filter out arrays from source vec
        .filter_map(|x| {
            if let Src::Arr(array) = x {
                Some(array)
            } else {
                None
            }
        })
        // make a hash map of [keys in source] -> [defined arrays]
        .map(|key| {
            // concat array to pattern name to get key in HashMap
            let name = file.name().unwrap_or_default();
            let long_key = name + "." + key;

            let value = if let Some(val) = file.get_arr(&Scope::into_global(&long_key)) {
                val
            } else if let Some(val) = file.get_arr(&Scope::into_local(&long_key)) {
                val
            } else if let Some(val) = file.get_arr(&Scope::into_global(key)) {
                val
            } else if let Some(val) = file.get_arr(&Scope::into_local(key)) {
                val
            } else if file.opts.undefined {
                panic!(
                    "{}",
                    MetaError::UndefinedExpand {
                        val: key.to_string(),
                        path: file.path.to_string_lossy().to_string(),
                    }
                )
            } else {
                &[]
            };
            (key.to_string(), value)
        })
        .collect();

    // loop to duplicate the output template for each array member
    let mut expanded = String::new();
    let size = match get_array_size(&map, file.header.equal_arrays) {
        Ok(num) => Ok(num),
        Err(e) => match e.as_ref() {
            &MetaError::Array => Err(MetaError::UnequalArrays {
                path: file.path.to_string_lossy().to_string(),
            }),
            _ => Err(MetaError::Unknown),
        },
    }?;
    for i in 0..size {
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

fn get_array_size(
    map: &HashMap<String, &[String]>,
    same_size: bool,
) -> Result<usize, Box<MetaError>> {
    if same_size {
        let mut size = (0, false);
        for val in map.values() {
            if !size.1 {
                size = (val.len(), true);
            } else if size.0 != val.len() {
                return Err(Box::new(MetaError::Array));
            }
        }
        return Ok(size.0);
    }

    let mut max = 0;
    for val in map.values() {
        if max < val.len() {
            max = val.len();
        }
    }
    Ok(max)
}
