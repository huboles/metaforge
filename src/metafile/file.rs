use crate::{parse_string, Options};
use color_eyre::{eyre::bail, Result};
use std::{collections::HashMap, path::PathBuf};

use super::*;

#[derive(Debug, Clone)]
pub struct MetaFile<'a> {
    pub opts: &'a Options,
    pub path: PathBuf,
    pub header: Header,
    pub variables: HashMap<String, String>,
    pub arrays: HashMap<String, Vec<String>>,
    pub patterns: HashMap<String, String>,
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

    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn get_arr(&self, key: &str) -> Option<&[String]> {
        self.arrays.get(key).map(|a| &a[..])
    }

    pub fn get_pat(&self, key: &str) -> Option<&String> {
        self.patterns.get(key)
    }

    pub fn merge(&mut self, other: &Self) {
        for (key, val) in other.variables.iter() {
            match self.variables.get(key) {
                Some(_) => continue,
                None => self.variables.insert(key.to_string(), val.to_string()),
            };
        }
        for (key, val) in other.arrays.iter() {
            match self.arrays.get(key) {
                Some(_) => continue,
                None => self.arrays.insert(key.to_string(), val.to_vec()),
            };
        }
        for (key, val) in other.patterns.iter() {
            match self.patterns.get(key) {
                Some(_) => continue,
                None => self.patterns.insert(key.to_string(), val.to_string()),
            };
        }
    }
}
