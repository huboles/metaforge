use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaError {
    #[error("unknown internal error")]
    Unknown,
    #[error("file ignored")]
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
    #[error("the parser cannot resolve this input: {input}")]
    UnreachableRule { input: String },
    #[error("{file}\n{error}")]
    ParserError { file: String, error: String },
    #[error(transparent)]
    MetaError(#[from] Box<MetaError>),
    #[error(transparent)]
    PandocError(#[from] pandoc::PandocError),
    #[error(transparent)]
    Other(#[from] eyre::Error),
}

pub fn check_ignore<T>(result: Result<T, MetaError>) -> Result<Option<T>, MetaError> {
    match result {
        Ok(f) => Ok(Some(f)),
        Err(e) => match e {
            MetaError::Ignored => Ok(None),
            e => Err(e.into()),
        },
    }
}
