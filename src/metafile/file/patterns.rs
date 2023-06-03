use super::*;

impl<'a> MetaFile<'a> {
    pub fn get_pattern(&self, key: &str) -> Result<String> {
        log!(self.opts, format!("expanding {key}"), 2);
        // SOURCE is already expanded in the initial construct() call
        if key == "SOURCE" {
            if let Some(source) = self.patterns.get(&Scope::create_global("SOURCE")) {
                return Ok(source.to_string());
            } else {
                return Ok(String::new());
            }
        }

        let is_source = key.split('.').next().unwrap_or("") == "SOURCE";

        let mut filename = if let Some(name) = self.patterns.get(&Scope::create_local(key)) {
            Ok(name.to_string())
        } else if let Some(name) = self.patterns.get(&Scope::create_global(key)) {
            Ok(name.to_string())
        } else if self
            .opts
            .pattern
            .join(key.replace('.', "/") + ".meta")
            .exists()
            || is_source
        {
            Ok(String::new())
        } else if self.header.panic_default {
            Err(MetaError::UndefinedDefault {
                pattern: key.to_string(),
                path: self.path.to_string_lossy().to_string(),
            })
        } else {
            // anything not defined should have a default.meta file to fall back to
            Ok("default".to_string())
        }?;

        // BLANK returns nothing, so no more processing needs to be done
        if filename == "BLANK" {
            return Ok(String::default());
        };

        // DEFAULT override for patterns overriding globals
        if filename == "DEFAULT" {
            filename = "default".to_string();
        }

        // if we're building the base pattern we need to wait on
        // parsing/expansion so we can build and convert source to html
        // for the SOURCE pattern. we just want to return the string right now
        if key == "base" {
            let pattern_path = key.to_string() + "/" + &filename;
            let mut path = self.opts.pattern.join(pattern_path);
            path.set_extension("meta");

            return match std::fs::read_to_string(&path) {
                Ok(str) => Ok(str),
                Err(_) => Err(MetaError::FileNotFound {
                    path: path.to_string_lossy().to_string(),
                }
                .into()),
            };
        }

        let pattern_path = key.replace('.', "/") + "/" + &filename;

        let mut path = if is_source {
            let pattern_path = pattern_path.replace("SOURCE/", "");
            self.opts.source.join(pattern_path)
        } else {
            self.opts.pattern.join(pattern_path)
        };

        path.set_extension("meta");
        let mut pattern = MetaFile::build(path, self.opts)?;

        // copy over maps for expanding contained variables
        pattern.merge(self);

        if pattern.header.pandoc.unwrap_or(false) || is_source {
            pattern.pandoc()
        } else {
            pattern.get_source()
        }
    }
}
