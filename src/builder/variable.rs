use crate::{log, MetaError, MetaFile, Scope};
use eyre::Result;

pub fn get_variable(key: &str, file: &MetaFile) -> Result<String> {
    log!(
        file.opts,
        format!("substituting {key} in {}", file.path.display()),
        2
    );
    let long_key = file.name()? + "." + key;
    if let Some(val) = file.get_var(&Scope::into_local(&long_key)) {
        Ok(val.clone())
    } else if let Some(val) = file.get_var(&Scope::into_global(&long_key)) {
        Ok(val.clone())
    } else if let Some(val) = file.get_var(&Scope::into_local(key)) {
        Ok(val.clone())
    } else if let Some(val) = file.get_var(&Scope::into_global(key)) {
        Ok(val.clone())
    } else if file.opts.undefined || file.header.panic_undefined {
        return Err(MetaError::UndefinedExpand {
            val: key.to_string(),
            path: file.name()?,
        }
        .into());
    } else {
        Ok(String::new())
    }
}
