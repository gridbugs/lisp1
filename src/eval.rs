use crate::{
    built_in::BuiltIn,
    language::Keyword,
    list,
    value::{Atom, Function, Pair, Value},
};
use std::{collections::HashMap, rc::Rc};

// (define fact1 (lambda (n)
//   (if (= n 0)
//      1
//      (fact2 n))))
//
// (define fact2 (lambda (n)
//   (* n (fact1 (- n 1)))))
//
// need an operation that annotates each symbol with how it is interpreted:
// - keyword
// - variable binding
// - bound variable
// - free variable
//
// a closure consists of:
// - code to invoke (ast will do)
// - argument list
// - captured environment
//   - scope hierarchy
//   - names of captured variables

#[derive(Debug)]
pub struct Scope {
    variables_by_name: HashMap<String, Rc<Value>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables_by_name: HashMap::default(),
        }
    }

    pub fn define_variable(&mut self, name: &str, value: Rc<Value>) {
        self.variables_by_name.insert(name.to_string(), value);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScopePath {
    indices: Vec<usize>,
}

impl ScopePath {
    fn new() -> Self {
        Self { indices: vec![0] }
    }

    fn push(&self, index: usize) -> Self {
        let mut ret = self.clone();
        ret.indices.push(index);
        ret
    }

    fn iter(&self) -> impl '_ + Iterator<Item = usize> {
        self.indices.iter().rev().cloned()
    }

    fn current(&self) -> usize {
        *self.indices.last().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lambda {
    argument_names: Vec<String>,
    code_ast: Rc<Value>,
    scope_path: ScopePath,
}

pub struct Runtime {
    scopes: Vec<Scope>,
}

impl Runtime {
    pub fn new() -> Self {
        let scopes = vec![Scope::new()];
        Self { scopes }
    }

    fn make_scope(&mut self) -> usize {
        let ret = self.scopes.len();
        self.scopes.push(Scope::new());
        ret
    }

    fn resolve_name(&self, name: &str, scope_path: ScopePath) -> Option<Rc<Value>> {
        for scope_index in scope_path.iter() {
            if let Some(value) = self.scopes[scope_index].variables_by_name.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn eval_define(&mut self, args: &Value, scope_path: ScopePath) {
        let scope_index = scope_path.current();
        match args {
            Value::Pair(Pair(name, value)) => {
                if let Value::Atom(Atom::Symbol(symbol_name)) = &**name {
                    match &**value {
                        Value::Pair(Pair(value, _rest_should_be_nil)) => {
                            if let Some(result_value) = self.eval(value) {
                                self.scopes[scope_index]
                                    .define_variable(symbol_name.as_str(), result_value);
                            } else {
                                todo!()
                            }
                        }
                        _ => todo!(),
                    }
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }

    fn define_lambda(&mut self, args: &Value, scope_path: ScopePath) -> Lambda {
        let (args, code_ast) = list::take2(args);
        let argument_name_values = list::to_vec(args);
        let argument_names = argument_name_values
            .into_iter()
            .map(|value| match &*value {
                Value::Atom(Atom::Symbol(s)) => s.clone(),
                _ => panic!("expected symbol"),
            })
            .collect::<Vec<_>>();
        Lambda {
            argument_names,
            code_ast,
            scope_path: scope_path.push(self.make_scope()),
        }
    }

    fn call_lambda(&mut self, lambda: &Lambda, args: &Rc<Value>) -> Rc<Value> {
        let args_vec = list::to_vec(args.clone());
        if args_vec.len() != lambda.argument_names.len() {
            panic!("incorrect number of arguments");
        }
        let arg_scope_index = self.make_scope();
        for (name, value) in lambda.argument_names.iter().zip(args_vec.into_iter()) {
            self.scopes[arg_scope_index].define_variable(name.as_str(), value);
        }
        let scope_path = lambda.scope_path.push(arg_scope_index);
        self.eval_with_scope_path(&lambda.code_ast, scope_path)
            .unwrap()
    }

    fn eval_with_scope_path(
        &mut self,
        value: &Rc<Value>,
        scope_path: ScopePath,
    ) -> Option<Rc<Value>> {
        match &**value {
            Value::Function(_) => Some(value.clone()),
            Value::Atom(ref atom) => {
                if let Some(symbol_name) = atom.symbol() {
                    if let Some(built_in) = BuiltIn::from_str(symbol_name) {
                        Some(Rc::new(Value::built_in(built_in)))
                    } else if let Some(variable_value) =
                        self.resolve_name(symbol_name, scope_path.clone())
                    {
                        Some(variable_value)
                    } else {
                        panic!("unhandled symbol: {}", symbol_name)
                    }
                } else {
                    Some(value.clone())
                }
            }
            Value::Pair(Pair(op, args)) => {
                if let Value::Atom(Atom::Symbol(symbol_name)) = &**op {
                    if let Some(keyword) = Keyword::from_str(symbol_name.as_str()) {
                        return match keyword {
                            Keyword::Define => {
                                self.eval_define(&*args, scope_path.clone());
                                None
                            }
                            Keyword::Quote => Some(list::head(args)),
                            Keyword::Lambda => Some(Rc::new(Value::Function(Function::Lambda(
                                self.define_lambda(&*args, scope_path.clone()),
                            )))),
                            Keyword::If => {
                                let (condition, if_true, if_false) = list::take3(args.clone());
                                let condition_evalled = self
                                    .eval_with_scope_path(&condition, scope_path.clone())
                                    .unwrap();
                                let condition_bool =
                                    if let Value::Atom(Atom::Bool(condition_bool)) =
                                        &*condition_evalled
                                    {
                                        *condition_bool
                                    } else {
                                        panic!()
                                    };
                                if condition_bool {
                                    self.eval_with_scope_path(&if_true, scope_path.clone())
                                } else {
                                    self.eval_with_scope_path(&if_false, scope_path.clone())
                                }
                            }
                        };
                    };
                }
                let op_value = self.eval_with_scope_path(op, scope_path.clone()).unwrap();
                match &*op_value {
                    Value::Function(function) => {
                        let args = list::map(args, |arg| {
                            self.eval_with_scope_path(arg, scope_path.clone()).unwrap()
                        });
                        match function {
                            Function::BuiltIn(built_in) => Some(built_in.eval(&args).unwrap()),
                            Function::Lambda(lambda) => Some(self.call_lambda(lambda, &args)),
                        }
                    }
                    _ => panic!(
                        "value in op position cannot be called in statement: {}",
                        value
                    ),
                }
            }
        }
    }

    pub fn eval(&mut self, value: &Rc<Value>) -> Option<Rc<Value>> {
        self.eval_with_scope_path(value, ScopePath::new())
    }

    pub fn get_top_level_variable(&self, name: &str) -> Option<Rc<Value>> {
        self.scopes[0].variables_by_name.get(name).cloned()
    }
}

#[cfg(test)]
mod test {
    use super::Runtime;
    use crate::{parse, value::Value};
    use std::rc::Rc;

    #[test]
    fn simple_defines() {
        let string = r#"
            (define my-string "foo")
            (define my-i64 42)
            (define my-symbol 'bar)
        "#;
        let ast = parse::parse(string).unwrap();
        let mut runtime = Runtime::new();
        for v in ast {
            runtime.eval(&Rc::new(v));
        }
        assert_eq!(
            *runtime.get_top_level_variable("my-string").unwrap(),
            Value::string("foo")
        );
        assert_eq!(
            *runtime.get_top_level_variable("my-i64").unwrap(),
            Value::i64(42)
        );
        assert_eq!(
            *runtime.get_top_level_variable("my-symbol").unwrap(),
            Value::symbol("bar")
        );
    }

    #[test]
    fn arithmetic() {
        let string = r#"
            (define answer (* (+ 2 4) (- 10 3)))
        "#;
        let ast = parse::parse(string).unwrap();
        let mut runtime = Runtime::new();
        for v in ast {
            runtime.eval(&Rc::new(v));
        }
        assert_eq!(
            *runtime.get_top_level_variable("answer").unwrap(),
            Value::i64(42)
        );
    }

    #[test]
    fn variable() {
        let string = r#"
            (define foo 12)
            (define bar 30)
            (define baz (+ foo bar))
        "#;
        let ast = parse::parse(string).unwrap();
        let mut runtime = Runtime::new();
        for v in ast {
            runtime.eval(&Rc::new(v));
        }
        assert_eq!(
            *runtime.get_top_level_variable("baz").unwrap(),
            Value::i64(42)
        );
    }

    #[test]
    fn factorial() {
        let string = r#"
            (define factorial (lambda (n)
                (if (= n 0)
                    1
                    (* n (factorial (- n 1))))))

            (define result (factorial 5))
        "#;
        let ast = parse::parse(string).unwrap();
        let mut runtime = Runtime::new();
        for v in ast {
            runtime.eval(&Rc::new(v));
        }
        assert_eq!(
            *runtime.get_top_level_variable("result").unwrap(),
            Value::i64(120)
        );
    }
}
