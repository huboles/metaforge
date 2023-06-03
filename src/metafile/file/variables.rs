use super::*;

impl<'a> MetaFile<'a> {
    pub fn get_variable(&self, key: &str) -> Result<String> {
        log!(
            self.opts,
            format!("substituting {key} in {}", self.path.display()),
            2
        );
        let long_key = self.name()? + "." + key;
        if let Some(val) = self.variables.get(&Scope::create_local(&long_key)) {
            Ok(val.clone())
        } else if let Some(val) = self.variables.get(&Scope::create_global(&long_key)) {
            Ok(val.clone())
        } else if let Some(val) = self.variables.get(&Scope::create_local(key)) {
            Ok(val.clone())
        } else if let Some(val) = self.variables.get(&Scope::create_global(key)) {
            Ok(val.clone())
        } else if self.opts.undefined || self.header.panic_undefined {
            return Err(MetaError::UndefinedExpand {
                val: key.to_string(),
                path: self.name()?,
            }
            .into());
        } else {
            Ok(String::new())
        }
    }
}
