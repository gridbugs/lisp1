use crate::value::Value;
use std::rc::Rc;

mod keyword {
    use crate::value::Value;

    const QUOTE: &str = "quote";

    pub fn quote() -> Value {
        Value::symbol(QUOTE)
    }
}

/// 4       => (quote 4)
/// (1 2 3) => (quote (1 2 3))
pub fn quote_value(value: Value) -> Value {
    Value::pair(
        Rc::new(keyword::quote()),
        Rc::new(Value::pair(Rc::new(value), Rc::new(Value::nil()))),
    )
}
