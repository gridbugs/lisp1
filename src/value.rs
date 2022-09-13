use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atom {
    Nil,
    Symbol(String),
    String(String),
    I64(i64),
    Bool(bool),
}

impl Atom {
    pub fn is_nil(&self) -> bool {
        if let Self::Nil = self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pair(pub Rc<Value>, pub Rc<Value>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Atom(Atom),
    Pair(Pair),
}

impl Value {
    pub const fn nil() -> Self {
        Self::Atom(Atom::Nil)
    }

    pub fn string<S: Into<String>>(s: S) -> Self {
        Self::Atom(Atom::String(s.into()))
    }

    pub fn symbol<S: Into<String>>(s: S) -> Self {
        Self::Atom(Atom::Symbol(s.into()))
    }

    pub const fn i64(i: i64) -> Self {
        Self::Atom(Atom::I64(i))
    }

    pub const fn bool(b: bool) -> Self {
        Self::Atom(Atom::Bool(b))
    }

    pub fn pair(a: Rc<Self>, b: Rc<Self>) -> Self {
        Self::Pair(Pair(a, b))
    }
}
