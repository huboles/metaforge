use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Header {
    pub blank: bool,
    pub panic_default: bool,
    pub panic_undefined: bool,
    pub equal_arrays: bool,
    pub filetype: String,
    pub source: String,
    pub pandoc: Option<bool>,
    pub ignore: bool,
    pub copy_only: bool,
}

impl Header {
    pub fn new() -> Self {
        Self {
            blank: false,
            panic_default: false,
            panic_undefined: false,
            equal_arrays: false,
            filetype: String::from("html"),
            source: String::from("markdown"),
            pandoc: None,
            ignore: false,
            copy_only: false,
        }
    }
}

impl From<HashMap<String, String>> for Header {
    fn from(value: HashMap<String, String>) -> Self {
        let mut header = Header::new();
        for (key, val) in value.iter() {
            match &key[..] {
                "blank" => header.blank = val == "true",
                "panic_default" => header.panic_default = val == "true",
                "panic_undefined" => header.panic_undefined = val == "true",
                "equal_arrays" => header.equal_arrays = val == "true",
                "pandoc" => header.pandoc = Some(val == "true"),
                "filetype" => header.filetype = val.to_string(),
                "source" => header.source = val.to_string(),
                "ignore" => header.ignore = val == "true",
                "copy_only" => header.copy_only = val == "true",
                _ => continue,
            }
        }
        header
    }
}
