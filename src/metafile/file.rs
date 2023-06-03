mod arrays;
mod attributes;
mod patterns;
mod source;
mod variables;

use crate::{log, parse_string, MetaError, Options};
use eyre::Result;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc};
use std::{collections::HashMap, path::PathBuf};

use super::*;

#[derive(Debug, Clone)]
pub struct MetaFile<'a> {
    pub opts: &'a Options,
    pub path: PathBuf,
    pub header: Header,
    pub variables: HashMap<Scope, String>,
    pub arrays: HashMap<Scope, Vec<String>>,
    pub patterns: HashMap<Scope, String>,
    pub source: Vec<Src>,
}

impl<'a> MetaFile<'a> {
    pub fn new(opts: &'a Options) -> Self {
        Self {
            opts,
            path: PathBuf::new(),
            header: Header::new(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
            patterns: HashMap::new(),
            source: Vec::new(),
        }
    }

    pub fn build(path: PathBuf, opts: &'a Options) -> Result<Self, MetaError> {
        let str = match std::fs::read_to_string(&path) {
            Ok(str) => str,
            Err(_) => {
                return Err(MetaError::FileNotFound {
                    path: path.to_string_lossy().to_string(),
                })
            }
        };

        let mut metafile = parse_string(str, opts).map_err(|e| MetaError::ParserError {
            file: path.to_string_lossy().to_string(),
            error: e.to_string(),
        })?;

        metafile.path = path;
        Ok(metafile)
    }

    pub fn construct(&mut self) -> Result<String, Box<MetaError>> {
        log!(self.opts, format!("building {}", self.path.display()), 1);

        if self.header.blank {
            return Ok(String::new());
        } else if self.header.ignore {
            return Err(Box::new(MetaError::Ignored));
        }

        if self.header.copy_only {
            let dest = self.dest().map_err(MetaError::from)?;
            let source: String = self.source.iter().map(|s| s.to_string()).collect();
            std::fs::write(dest, source).unwrap();
            return Err(Box::new(MetaError::Ignored));
        }

        let src_str = if self.header.pandoc.map_or(true, |x| x) {
            self.pandoc().map_err(MetaError::from)
        } else {
            self.get_source().map_err(MetaError::from)
        }?;

        let pattern = self.get_pattern("base").map_err(MetaError::from)?;
        let mut base = parse_string(pattern, self.opts).map_err(|e| MetaError::ParserError {
            file: self.path.to_string_lossy().to_string(),
            error: e.to_string(),
        })?;

        base.merge(self);
        base.patterns
            .insert(Scope::create_global("SOURCE"), src_str);
        let mut base_path = self.opts.pattern.join("base").join(
            self.patterns
                .get(&Scope::create_global("base"))
                .unwrap_or(&"default".into()),
        );

        base_path.set_extension("meta");
        base.path = base_path;

        let output = base.get_source().map_err(MetaError::from)?;

        Ok(output)
    }

    pub fn merge(&mut self, other: &Self) {
        macro_rules! merge (
            ($m:ident) => {
                for (key, val) in other.$m.iter() {
                    if key.is_global() && !self.$m.contains_key(key) {
                        self.$m.insert(key.clone(), val.clone());
                    }
                }
            };
        );

        merge!(variables);
        merge!(arrays);
        merge!(patterns);
    }
}
