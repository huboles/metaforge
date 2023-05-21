use super::*;

impl<'a> MetaFile<'a> {
    pub fn get_variable(&self, key: &str) -> Result<String> {
        log!(
            self.opts,
            format!(
                "substituting {} in {}",
                key.to_string(),
                self.path.display()
            ),
            2
        );
        let long_key = self.name()? + "." + &key.to_string();
        if let Some(val) = self.variables.get(&Scope::into_local(&long_key)) {
            Ok(val.clone())
        } else if let Some(val) = self.variables.get(&Scope::into_global(&long_key)) {
            Ok(val.clone())
        } else if let Some(val) = self.variables.get(&Scope::into_local(key)) {
            Ok(val.clone())
        } else if let Some(val) = self.variables.get(&Scope::into_global(key)) {
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
