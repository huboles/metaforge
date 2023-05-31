mod dir;
mod file;
mod header;
mod scope;

pub use dir::*;
pub use file::*;
pub use header::*;
pub use scope::*;

#[cfg(test)]
mod tests;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Src {
    Str(String),
    Var(String),
    Arr(String),
    Pat(String),
}

impl Src {
    pub fn to_var(var: impl Display) -> Self {
        Src::Var(var.to_string())
    }

    pub fn to_arr(arr: impl Display) -> Self {
        Src::Arr(arr.to_string())
    }

    pub fn to_pat(pat: impl Display) -> Self {
        Src::Pat(pat.to_string())
    }

    pub fn to_str(str: impl Display) -> Self {
        println!("{}", str.to_string());
        Src::Str(str.to_string())
    }
}

impl Display for Src {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Src::Var(x) | Src::Arr(x) | Src::Pat(x) | Src::Str(x) => x.to_string(),
        };

        write!(f, "{str}")
    }
}
