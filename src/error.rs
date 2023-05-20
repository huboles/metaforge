use crate::Rule;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaError {
    #[error("unknown internal error")]
    Unknown,
    #[error("internal break switch")]
    Ignored,
    #[error("internal filetype error")]
    Filetype,
    #[error("internal array error")]
    Array,
    #[error("mismatched array sizes in {path}")]
    UnequalArrays { path: String },
    #[error("could not find {path}")]
    FileNotFound { path: String },
    #[error("could not determine name from {file}")]
    Name { file: String },
    #[error("pandoc could not write to buffer for {file}")]
    Pandoc { file: String },
    #[error("undefined expansion: {val}\n\tin {path}")]
    UndefinedExpand { val: String, path: String },
    #[error("undefined call to default.meta: {pattern}\n\tin {path}")]
    UndefinedDefault { pattern: String, path: String },
    #[error(transparent)]
    MetaError(#[from] Box<MetaError>),
    #[error(transparent)]
    PandocError(#[from] pandoc::PandocError),
    #[error(transparent)]
    ParserError(#[from] pest::error::Error<Rule>),
    #[error(transparent)]
    Other(#[from] eyre::Error),
}
