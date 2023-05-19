use crate::{MetaFile, Scope, Src};
use eyre::Result;
use std::collections::HashMap;

pub fn expand_arrays(input: String, file: &MetaFile) -> Result<String> {
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

            let value = if let Some(val) = file.get_arr(&Scope::into_global(long_key.to_string())) {
                val
            } else if let Some(val) = file.get_arr(&Scope::into_local(long_key.to_string())) {
                val
            } else if let Some(val) = file.get_arr(&Scope::into_global(key)) {
                val
            } else if let Some(val) = file.get_arr(&Scope::into_local(key)) {
                val
            } else if file.opts.undefined {
                panic!("undefined array called: {}, {}", key, long_key);
            } else {
                &[]
            };
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
