mod dir;
mod file;
mod header;

pub use dir::*;
pub use file::*;
pub use header::*;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! source (
    (var($s:expr)) => { crate::Src::Sub(crate::Sub::Var($s.to_string()))};
    (arr($s:expr)) => { crate::Src::Sub(crate::Sub::Arr($s.to_string()))};
    (pat($s:expr)) => { crate::Src::Sub(crate::Sub::Pat($s.to_string()))};
    ($s:expr) => { Src::Str($s.to_string())};
);

#[derive(Debug, Clone, PartialEq)]
pub enum Src {
    Str(String),
    Sub(Sub),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sub {
    Var(String),
    Arr(String),
    Pat(String),
}
