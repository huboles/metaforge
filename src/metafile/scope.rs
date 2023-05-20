#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Scope {
    Local(String),
    Global(String),
}

impl Scope {
    pub fn into_local(str: impl ToString) -> Scope {
        Scope::Local(str.to_string())
    }

    pub fn into_global(str: impl ToString) -> Scope {
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

    pub fn to_local(&self) -> Scope {
        Scope::Local(self.to_string())
    }

    pub fn to_global(&self) -> Scope {
        Scope::Global(self.to_string())
    }
}

impl ToString for Scope {
    fn to_string(&self) -> String {
        match self {
            Scope::Local(x) | Scope::Global(x) => x.to_string(),
        }
    }
}
