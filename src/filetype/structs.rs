use std::path::PathBuf;

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

#[derive(Debug, Clone, Default)]
pub struct RootDirs {
    pub root: PathBuf,
    pub source: PathBuf,
    pub build: PathBuf,
    pub pattern: PathBuf,
}

impl RootDirs {
    pub fn new() -> Self {
        Self {
            root: PathBuf::new(),
            source: PathBuf::new(),
            build: PathBuf::new(),
            pattern: PathBuf::new(),
        }
    }
}
