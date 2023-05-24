use super::*;

impl<'a> MetaFile<'a> {
    pub fn dest(&self) -> Result<PathBuf> {
        let mut path = self
            .opts
            .build
            .join(self.path.strip_prefix(&self.opts.source)?);
        path.set_extension(&self.header.filetype);

        Ok(path)
    }

    pub fn class(&self) -> Result<String> {
        // only care about classes in the pattern dir
        self.path
            .strip_prefix(&self.opts.pattern)?
            .parent()
            .map(|s| s.to_string_lossy().to_string().replace('/', "."))
            .ok_or(
                MetaError::Name {
                    file: self.path.to_string_lossy().to_string(),
                }
                .into(),
            )
    }

    pub fn name(&self) -> Result<String> {
        let path = if self.path.starts_with(&self.opts.pattern) {
            self.path.strip_prefix(&self.opts.pattern)?
        } else if self.path.starts_with(&self.opts.source) {
            self.path.strip_prefix(&self.opts.source)?
        } else {
            return Err(MetaError::Name {
                file: self.path.to_string_lossy().to_string(),
            }
            .into());
        };

        Ok(path
            .components()
            .map(|x| {
                x.as_os_str()
                    .to_string_lossy()
                    .to_string()
                    .replace(".meta", "")
            })
            .collect::<Vec<String>>()
            .join("."))
    }
}
