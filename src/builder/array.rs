use crate::{MetaFile, Src, Sub};
use color_eyre::Result;
use std::collections::HashMap;

pub fn expand_arrays(input: String, file: &MetaFile) -> Result<String> {
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
