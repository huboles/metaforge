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

    pub fn name(&self) -> Result<String> {
        if self.path.starts_with(&self.opts.source) {
            // in source dir, we want the file name without the '.meta' extension
            let name: String = self
                .path
                .strip_prefix(&self.opts.source)?
                .components()
                .map(|x| {
                    x.as_os_str()
                        .to_string_lossy()
                        .to_string()
                        .replace(".meta", "")
                })
                .collect::<Vec<String>>()
                .join(".");
            Ok(name)
        } else if self.path.starts_with(&self.opts.pattern) {
            // in pattern dir, we want the parent dir
            let name = self.path.strip_prefix(&self.opts.pattern)?;
            let name = name
                .parent()
                .map(|s| s.to_string_lossy().to_string().replace('/', "."))
                .unwrap_or_default();
            Ok(name)
        } else {
            Err(MetaError::Name {
                file: self.path.to_string_lossy().to_string(),
            }
            .into())
        }
    }
}
