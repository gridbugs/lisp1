use crate::value::Value;
use std::rc::Rc;

mod keyword {
    use crate::value::Value;

    const QUOTE: &str = "quote";
    const DEFINE: &str = "define";
    const LAMBDA: &str = "lambda";
    const IF: &str = "if";

    pub fn quote() -> Value {
        Value::symbol(QUOTE)
    }

    pub enum Keyword {
        Quote,
        Define,
        Lambda,
        If,
    }

    impl Keyword {
        pub fn from_str(s: &str) -> Option<Keyword> {
            use Keyword::*;
            match s {
                QUOTE => Some(Quote),
                DEFINE => Some(Define),
                LAMBDA => Some(Lambda),
                IF => Some(If),
                _ => None,
            }
        }
    }
}

pub use keyword::Keyword;

pub fn quote_value(value: Value) -> Value {
    Value::pair(
        Rc::new(keyword::quote()),
        Rc::new(Value::pair(Rc::new(value), Rc::new(Value::nil()))),
    )
}
