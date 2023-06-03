use std::fmt::Display;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Scope {
    Local(String),
    Global(String),
}

impl Scope {
    pub fn create_local(str: impl Display) -> Scope {
        Scope::Local(str.to_string())
    }

    pub fn create_global(str: impl ToString) -> Scope {
        Scope::Global(str.to_string())
    }

    pub fn is_global(&self) -> bool {
        match self {
            Scope::Local(_) => false,
            Scope::Global(_) => true,
        }
    }

    pub fn is_local(&self) -> bool {
        match self {
            Scope::Local(_) => true,
            Scope::Global(_) => false,
        }
    }

    pub fn local(&self) -> Scope {
        Scope::Local(self.to_string())
    }

    pub fn global(&self) -> Scope {
        Scope::Global(self.to_string())
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Scope::Local(x) | Scope::Global(x) => x.to_string(),
        };

        write!(f, "{str}")
    }
}
