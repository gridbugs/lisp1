use crate::value::{Atom, Pair, Value};
use std::rc::Rc;

pub fn from_vec(mut vec: Vec<Value>) -> Value {
    let mut list = Value::Atom(Atom::Nil);
    for value in vec.drain(..).rev() {
        list = Value::Pair(Pair(Rc::new(value), Rc::new(list)));
    }
    list
}
