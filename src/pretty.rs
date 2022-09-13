mod atom {
    use crate::value::Atom;
    use std::fmt;

    fn nil(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "()")
    }

    fn symbol(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", s)
    }

    fn string(s: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", s)
    }

    fn i64(i: i64, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i)
    }

    fn bool(b: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", b)
    }

    pub fn fmt_atom(atom: &Atom, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match atom {
            Atom::Nil => nil(f),
            Atom::Symbol(s) => symbol(s.as_ref(), f),
            Atom::String(s) => string(s.as_ref(), f),
            Atom::I64(i) => i64(*i, f),
            Atom::Bool(b) => bool(*b, f),
        }
    }
}

mod pair {
    use super::{atom::fmt_atom, value::fmt_value};
    use crate::value::{Pair, Value};
    use std::fmt;

    pub fn inner(pair: &Pair, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_value(&pair.0, f)?;
        match &*pair.1 {
            Value::Atom(atom) => {
                if atom.is_nil() {
                    Ok(())
                } else {
                    write!(f, " . ")?;
                    fmt_atom(atom, f)
                }
            }
            Value::Pair(pair) => {
                write!(f, " ")?;
                inner(pair, f)
            }
        }
    }

    pub fn fmt_pair(pair: &Pair, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        inner(pair, f)?;
        write!(f, ")")
    }
}

mod value {
    use super::{atom, pair};
    use crate::value::Value;
    use std::fmt;

    pub fn fmt_value(value: &Value, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match value {
            Value::Atom(atom) => atom::fmt_atom(atom, f),
            Value::Pair(pair) => pair::fmt_pair(pair, f),
        }
    }
}

use crate::value::Value;
use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        value::fmt_value(self, f)
    }
}
