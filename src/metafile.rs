use std::collections::HashMap;

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
}

#[derive(Debug, Clone)]
pub enum Source<'a> {
    Str(&'a str),
    Sub(Substitution<'a>),
}

#[derive(Debug, Clone)]
pub enum Substitution<'a> {
    Variable(&'a str),
    Array(&'a str),
    Pattern(&'a str),
}
