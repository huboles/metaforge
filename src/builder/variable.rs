use crate::{MetaFile, Scope};
use color_eyre::{eyre::bail, Result};

pub fn get_variable(key: &str, file: &MetaFile) -> Result<String> {
    let long_key = file.name()? + "." + key;
    if let Some(val) = file.get_var(&Scope::into_local(long_key.to_string())) {
        Ok(val.clone())
    } else if let Some(val) = file.get_var(&Scope::into_global(long_key.to_string())) {
        Ok(val.clone())
    } else if let Some(val) = file.get_var(&Scope::into_local(key)) {
        Ok(val.clone())
    } else if let Some(val) = file.get_var(&Scope::into_global(key)) {
        Ok(val.clone())
    } else if file.opts.undefined {
        bail!("undefined variable: {}, {}", key.to_string(), &long_key)
    } else {
        Ok(String::new())
    }
}
