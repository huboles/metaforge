use super::*;

impl<'a> MetaFile<'a> {
    pub fn expand_arrays(&self, input: String) -> Result<String> {
        log!(
            self.opts,
            format!("expanding arrays in {}", self.path.display()),
            2
        );

        let map: HashMap<String, &[String]> = self
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
                let class = self.class().unwrap_or_default();
                let class_key = Scope::Local(class + "." + key);
                let name = self.name().unwrap_or_default();
                let name_key = Scope::Local(name + "." + key);

                let value = if let Some(val) = self.arrays.get(&name_key) {
                    &val[..]
                } else if let Some(val) = self.arrays.get(&name_key.to_global()) {
                    &val[..]
                } else if let Some(val) = self.arrays.get(&class_key) {
                    &val[..]
                } else if let Some(val) = self.arrays.get(&class_key.to_global()) {
                    &val[..]
                } else if let Some(val) = self.arrays.get(&Scope::into_global(key)) {
                    &val[..]
                } else if let Some(val) = self.arrays.get(&Scope::into_local(key)) {
                    &val[..]
                } else if self.opts.undefined {
                    panic!(
                        "{}",
                        MetaError::UndefinedExpand {
                            val: key.to_string(),
                            path: self.path.to_string_lossy().to_string(),
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
        let size = if self.header.equal_arrays {
            let mut size = (0, false);
            for val in map.values() {
                if !size.1 {
                    size = (val.len(), true);
                } else if size.0 != val.len() {
                    return Err(eyre::Error::from(MetaError::Array));
                }
            }
            Ok::<usize, eyre::Error>(size.0)
        } else {
            let mut max = 0;
            for val in map.values() {
                if max < val.len() {
                    max = val.len();
                }
            }
            Ok(max)
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
}
