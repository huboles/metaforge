use crate::{parse_string, Options};
use color_eyre::{eyre::bail, Result};
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
    pub fn build(path: PathBuf, opts: &'a Options) -> Result<Self> {
        let str = match std::fs::read_to_string(&path) {
            Ok(str) => str,
            Err(_) => bail!("{} does not exist", path.display()),
        };
        let mut metafile = parse_string(str, opts)?;
        metafile.path = path;
        Ok(metafile)
    }

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
            color_eyre::eyre::bail!("could not get name from: {}", self.path.display());
        }
    }

    pub fn get_var(&self, key: &Scope) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn get_arr(&self, key: &Scope) -> Option<&[String]> {
        self.arrays.get(key).map(|a| &a[..])
    }

    pub fn get_pat(&self, key: &Scope) -> Option<&String> {
        self.patterns.get(key)
    }

    pub fn var_defined(&self, key: &str) -> bool {
        if self.variables.contains_key(&Scope::into_local(key))
            || self.variables.contains_key(&Scope::into_global(key))
        {
            true
        } else {
            false
        }
    }

    pub fn arr_defined(&self, key: &str) -> bool {
        if self.arrays.contains_key(&Scope::into_local(key))
            || self.arrays.contains_key(&Scope::into_global(key))
        {
            true
        } else {
            false
        }
    }

    pub fn pat_defined(&self, key: &str) -> bool {
        if self.patterns.contains_key(&Scope::into_local(key))
            || self.patterns.contains_key(&Scope::into_global(key))
        {
            true
        } else {
            false
        }
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
