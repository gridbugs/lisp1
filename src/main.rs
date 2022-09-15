#![allow(dead_code)]
mod built_in;
mod error;
mod eval;
mod language;
mod list;
mod parse;
mod pretty;
mod value;

use std::{
    io::{self, BufRead},
    rc::Rc,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        if let Ok(string) = line.as_ref() {
            buffer.push_str(string.as_str());
        }
    }
    let ast = parse::parse(buffer.as_str()).unwrap();
    let mut runtime = eval::Runtime::new();
    for v in ast {
        runtime.eval(&Rc::new(v));
    }
    Ok(())
}
