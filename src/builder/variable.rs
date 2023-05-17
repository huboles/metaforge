use crate::MetaFile;
use color_eyre::{eyre::bail, Result};

pub fn get_variable(key: &str, file: &MetaFile) -> Result<String> {
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
