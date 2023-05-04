use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, Clone)]
pub struct MetaFile<'a> {
    pub variables: HashMap<&'a str, &'a str>,
    pub arrays: HashMap<&'a str, Vec<&'a str>>,
    pub patterns: HashMap<&'a str, &'a str>,
    pub source: Vec<Source<'a>>,
}

impl<'a> MetaFile<'a> {
    pub fn new() -> MetaFile<'a> {
        MetaFile {
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

#[macro_export]
macro_rules! source (
    (var($s:expr)) => { Source::Sub(Substitution::Variable($s))};
    (arr($s:expr)) => { Source::Sub(Substitution::Array($s))};
    (pat($s:expr)) => { Source::Sub(Substitution::Pattern($s))};
    ($s:expr) => { Source::Str($s)};
);

#[derive(Debug, Clone, PartialEq)]
pub enum Source<'a> {
    Str(&'a str),
    Sub(Substitution<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Substitution<'a> {
    Variable(&'a str),
    Array(&'a str),
    Pattern(&'a str),
}

#[derive(Debug, Clone)]
pub struct RootDirs {
    pub source: PathBuf,
    pub build: PathBuf,
    pub pattern: PathBuf,
}
