use crate::{error::RuntimeError, list, value::Value};
use std::rc::Rc;

mod name {
    pub const ADD: &str = "+";
    pub const SUB: &str = "-";
    pub const MUL: &str = "*";
    pub const EQ: &str = "=";
    pub const PRINTLN: &str = "println";
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuiltIn {
    Add,
    Sub,
    Mul,
    Eq,
    Println,
}

fn check_args_length(args: &Rc<Value>, required_length: usize) -> Result<(), RuntimeError> {
    if !list::is_list(args) {
        return Err(RuntimeError::Message(format!(
            "arguments is not in a list: {}",
            args
        )));
    }
    let found_length = list::length(args);
    if found_length != required_length {
        return Err(RuntimeError::Message(format!(
            "expected {} arguments, found {}",
            required_length, found_length
        )));
    }
    Ok(())
}

fn add(args: &Rc<Value>) -> Result<Rc<Value>, RuntimeError> {
    use crate::value::{Atom::*, Value::*};
    check_args_length(args, 2)?;
    let (lhs, rhs) = list::take2(args.clone());
    let result = match (&*lhs, &*rhs) {
        (Atom(I64(lhs)), Atom(I64(rhs))) => Atom(I64(lhs + rhs)),
        _ => {
            return Err(RuntimeError::Message(format!(
                "incorrect types in arguments to add"
            )))
        }
    };
    Ok(Rc::new(result))
}

fn sub(args: &Rc<Value>) -> Result<Rc<Value>, RuntimeError> {
    use crate::value::{Atom::*, Value::*};
    check_args_length(args, 2)?;
    let (lhs, rhs) = list::take2(args.clone());
    let result = match (&*lhs, &*rhs) {
        (Atom(I64(lhs)), Atom(I64(rhs))) => Atom(I64(lhs - rhs)),
        _ => {
            return Err(RuntimeError::Message(format!(
                "incorrect types in arguments to sub"
            )))
        }
    };
    Ok(Rc::new(result))
}

fn mul(args: &Rc<Value>) -> Result<Rc<Value>, RuntimeError> {
    use crate::value::{Atom::*, Value::*};
    check_args_length(args, 2)?;
    let (lhs, rhs) = list::take2(args.clone());
    let result = match (&*lhs, &*rhs) {
        (Atom(I64(lhs)), Atom(I64(rhs))) => Atom(I64(lhs * rhs)),
        _ => {
            return Err(RuntimeError::Message(format!(
                "incorrect types in arguments to mul: {}",
                args
            )))
        }
    };
    Ok(Rc::new(result))
}

fn eq(args: &Rc<Value>) -> Result<Rc<Value>, RuntimeError> {
    use crate::value::{Atom::*, Value::*};
    check_args_length(args, 2)?;
    let (lhs, rhs) = list::take2(args.clone());
    let result = match (&*lhs, &*rhs) {
        (Atom(I64(lhs)), Atom(I64(rhs))) => Atom(Bool(lhs == rhs)),
        _ => {
            return Err(RuntimeError::Message(format!(
                "incorrect types in arguments to eq: {}",
                args
            )))
        }
    };
    Ok(Rc::new(result))
}

fn println(args: &Rc<Value>) -> Result<Rc<Value>, RuntimeError> {
    check_args_length(args, 1)?;
    let arg = list::head(args);
    println!("{}", arg);
    Ok(Rc::new(Value::nil()))
}

impl BuiltIn {
    pub fn from_str(s: &str) -> Option<Self> {
        use name::*;
        use BuiltIn::*;
        match s {
            ADD => Some(Add),
            SUB => Some(Sub),
            MUL => Some(Mul),
            EQ => Some(Eq),
            PRINTLN => Some(Println),
            _ => None,
        }
    }

    pub fn eval(&self, args: &Rc<Value>) -> Result<Rc<Value>, RuntimeError> {
        use BuiltIn::*;
        match self {
            Add => add(args),
            Sub => sub(args),
            Mul => mul(args),
            Eq => eq(args),
            Println => println(args),
        }
    }
}
