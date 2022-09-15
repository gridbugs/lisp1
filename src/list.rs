use crate::value::{Atom, Pair, Value};
use std::{borrow::Borrow, rc::Rc};

pub fn from_vec_with_end(mut vec: Vec<Value>, end: Value) -> Value {
    let mut list = end;
    for value in vec.drain(..).rev() {
        list = Value::Pair(Pair(Rc::new(value), Rc::new(list)));
    }
    list
}

pub fn from_vec(vec: Vec<Value>) -> Value {
    from_vec_with_end(vec, Value::Atom(Atom::Nil))
}

pub fn to_vec<L: Borrow<Value>>(list: L) -> Vec<Rc<Value>> {
    let mut list = list.borrow();
    let mut ret = Vec::new();
    loop {
        match list {
            Value::Atom(Atom::Nil) => break ret,
            Value::Pair(Pair(head, tail)) => {
                ret.push(head.clone());
                list = &**tail;
            }
            _ => panic!("not a list"),
        }
    }
}

pub fn is_list(value: &Rc<Value>) -> bool {
    match &**value {
        Value::Atom(Atom::Nil) => true,
        Value::Pair(Pair(_, tail)) => is_list(&*tail),
        _ => false,
    }
}

pub fn length(value: &Rc<Value>) -> usize {
    match &**value {
        Value::Atom(Atom::Nil) => 0,
        Value::Pair(Pair(_, tail)) => 1 + length(&*tail),
        _ => panic!("not a list"),
    }
}

pub fn map<F: FnMut(&Rc<Value>) -> Rc<Value>>(list: &Rc<Value>, mut f: F) -> Rc<Value> {
    match &**list {
        Value::Atom(Atom::Nil) => list.clone(),
        Value::Pair(Pair(head, tail)) => Rc::new(Value::Pair(Pair(f(head), map(tail, f)))),
        _ => panic!("not a list"),
    }
}

pub fn split_head<L: Borrow<Value>>(list: L) -> (Rc<Value>, Rc<Value>) {
    match list.borrow() {
        Value::Atom(Atom::Nil) => panic!("list is empty"),
        Value::Pair(Pair(first, rest)) => (first.clone(), rest.clone()),
        _ => panic!("not a list"),
    }
}

pub fn head(list: &Rc<Value>) -> Rc<Value> {
    match &**list {
        Value::Atom(Atom::Nil) => panic!("list is empty"),
        Value::Pair(Pair(first, _)) => first.clone(),
        _ => panic!("not a list"),
    }
}

pub fn take2<L: Borrow<Value>>(list: L) -> (Rc<Value>, Rc<Value>) {
    let (first, rest) = split_head(list.borrow());
    let second = head(&rest);
    (first, second)
}

pub fn take3<L: Borrow<Value>>(list: L) -> (Rc<Value>, Rc<Value>, Rc<Value>) {
    let (first, rest) = split_head(list);
    let (second, rest) = split_head(rest);
    let third = head(&rest);
    (first, second, third)
}
