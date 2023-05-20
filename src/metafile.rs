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

#[derive(Debug, Clone, PartialEq)]
pub enum Src {
    Str(String),
    Var(String),
    Arr(String),
    Pat(String),
}

impl Src {
    pub fn to_var(var: impl ToString) -> Self {
        Src::Var(var.to_string())
    }

    pub fn to_arr(arr: impl ToString) -> Self {
        Src::Arr(arr.to_string())
    }

    pub fn to_pat(pat: impl ToString) -> Self {
        Src::Pat(pat.to_string())
    }

    pub fn to_str(str: impl ToString) -> Self {
        Src::Str(str.to_string())
    }
}

impl ToString for Src {
    fn to_string(&self) -> String {
        match self {
            Src::Var(x) | Src::Arr(x) | Src::Pat(x) | Src::Str(x) => x.to_string(),
        }
    }
}
