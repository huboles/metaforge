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

    pub fn build(path: PathBuf, opts: &'a Options) -> Result<Self> {
        let str = match std::fs::read_to_string(&path) {
            Ok(str) => str,
            Err(_) => {
                return Err(MetaError::FileNotFound {
                    path: path.to_string_lossy().to_string(),
                }
                .into())
            }
        };
        let mut metafile = parse_string(str, opts)?;

        metafile.path = path;
        Ok(metafile)
    }

    pub fn construct(&self) -> Result<String, Box<MetaError>> {
        log!(self.opts, format!("building {}", self.path.display()), 1);

        if self.header.blank {
            return Ok(String::new());
        } else if self.header.ignore {
            return Err(Box::new(MetaError::Ignored));
        }

        let html = self.to_html().map_err(MetaError::from)?;

        let pattern = self.get_pattern("base").map_err(MetaError::from)?;
        let mut base = crate::parse_string(pattern, self.opts).map_err(MetaError::from)?;

        base.merge(self);
        base.patterns.insert(Scope::into_global("SOURCE"), html);

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
