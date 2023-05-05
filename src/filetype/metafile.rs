use crate::Source;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct MetaFile<'a> {
    pub variables: HashMap<&'a str, &'a str>,
    pub arrays: HashMap<&'a str, Vec<&'a str>>,
    pub patterns: HashMap<&'a str, &'a str>,
    pub source: Vec<Source<'a>>,
}

impl<'a> MetaFile<'a> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            patterns: HashMap::new(),
            source: Vec::new(),
        }
    }

    pub fn get_var(&self, key: &str) -> Option<&str> {
        self.variables.get(key).copied()
    }

    pub fn get_arr(&self, key: &str) -> Option<&[&str]> {
        self.arrays.get(key).map(|val| &val[..])
    }

    pub fn get_pat(&self, key: &str) -> Option<&str> {
        self.patterns.get(key).copied()
    }
}
