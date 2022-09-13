mod string;

mod atom {
    use super::string;
    use crate::value::Atom;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, i64},
        combinator::{map, recognize, value, verify},
        multi::many0,
        sequence::pair,
        IResult,
    };

    fn parse_nil(input: &str) -> IResult<&str, Atom> {
        value(Atom::Nil, tag("nil"))(input)
    }

    fn parse_symbol_str(input: &str) -> IResult<&str, &str> {
        fn is_valid_first(c: char) -> bool {
            c.is_alphabetic()
                || c == '_'
                || c == '-'
                || c == '+'
                || c == '*'
                || c == '?'
                || c == '='
                || c == '/'
                || c == '!'
                || c == '&'
                || c == '&'
                || c == '|'
        }
        let first = verify(anychar, |&c| is_valid_first(c));
        let rest = verify(anychar, |&c| is_valid_first(c) || c.is_numeric());
        recognize(pair(first, many0(rest)))(input)
    }

    #[test]
    fn test_parse_string() {
        use nom::{error::Error, error::ErrorKind, Err};

        assert_eq!(parse_symbol_str("foo"), Ok(("", "foo")));
        assert_eq!(parse_symbol_str("foo-bar?"), Ok(("", "foo-bar?")));

        assert_eq!(
            parse_symbol_str(""),
            Err(Err::Error(Error {
                input: "",
                code: ErrorKind::Eof,
            }))
        );

        assert_eq!(
            parse_symbol_str("42x"),
            Err(Err::Error(Error {
                input: "42x",
                code: ErrorKind::Verify,
            }))
        );

        assert_eq!(
            parse_symbol_str(">"),
            Err(Err::Error(Error {
                input: ">",
                code: ErrorKind::Verify,
            }))
        );
    }

    fn parse_symbol(input: &str) -> IResult<&str, Atom> {
        map(parse_symbol_str, |s| Atom::Symbol(s.to_string()))(input)
    }

    fn parse_string(input: &str) -> IResult<&str, Atom> {
        map(string::parse_string, Atom::String)(input)
    }

    fn parse_i64(input: &str) -> IResult<&str, Atom> {
        map(i64, Atom::I64)(input)
    }

    fn parse_bool(input: &str) -> IResult<&str, Atom> {
        alt((
            value(Atom::Bool(true), tag("true")),
            value(Atom::Bool(false), tag("false")),
        ))(input)
    }

    pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
        alt((parse_bool, parse_i64, parse_nil, parse_symbol, parse_string))(input)
    }
}

mod list {
    use super::value;
    use crate::{list, value::Value};
    use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};

    pub fn parse_list(input: &str) -> IResult<&str, Value> {
        map(
            delimited(tag("("), value::parse_values, tag(")")),
            list::from_vec,
        )(input)
    }
}

mod quote {
    use super::value;
    use crate::{language, value::Value};
    use nom::{bytes::complete::tag, combinator::map, sequence::preceded, IResult};

    pub fn parse_quote(input: &str) -> IResult<&str, Value> {
        map(
            preceded(tag("'"), value::parse_value),
            language::quote_value,
        )(input)
    }
}

mod value {
    use super::{atom, list, quote};
    use crate::value::Value;
    use nom::{
        branch::alt, character::complete::multispace0, combinator::map, multi::many0,
        sequence::delimited, IResult,
    };

    fn parse_value_no_space(input: &str) -> IResult<&str, Value> {
        alt((
            map(atom::parse_atom, Value::Atom),
            list::parse_list,
            quote::parse_quote,
        ))(input)
    }

    pub fn parse_value(input: &str) -> IResult<&str, Value> {
        delimited(multispace0, parse_value_no_space, multispace0)(input)
    }

    pub fn parse_values(input: &str) -> IResult<&str, Vec<Value>> {
        many0(parse_value)(input)
    }
}

use crate::value::Value;
use nom::{combinator::all_consuming, Finish};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError;

pub fn parse(input: &str) -> Result<Vec<Value>, ParseError> {
    all_consuming(value::parse_values)(input)
        .finish()
        .map_err(|_| ParseError)
        .map(|(_, output)| output)
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::{
        language, list,
        value::{Atom, Value},
    };

    #[test]
    fn empty_text() {
        assert_eq!(parse(""), Ok(vec![]));
    }

    #[test]
    fn nil() {
        assert_eq!(parse("()"), Ok(vec![Value::nil()]));
    }

    #[test]
    fn string() {
        assert_eq!(
            parse("\"Hello, World!\""),
            Ok(vec![Value::Atom(Atom::String("Hello, World!".to_string()))])
        );
    }

    #[test]
    fn list_of_strings() {
        let string = r#"
            ("foo" "bar"
                "baz")
        "#;
        let expected_result = vec![list::from_vec(vec![
            Value::string("foo"),
            Value::string("bar"),
            Value::string("baz"),
        ])];
        assert_eq!(parse(string), Ok(expected_result));
    }

    #[test]
    fn list_of_i64() {
        let string = r#"
            (42 -1234567890)
        "#;
        let expected_result = vec![list::from_vec(vec![
            Value::i64(42),
            Value::i64(-1234567890),
        ])];
        assert_eq!(parse(string), Ok(expected_result));
    }

    #[test]
    fn list_of_symbols() {
        let string = r#"
            (foo - Bar BAZ42)
        "#;
        let expected_result = vec![list::from_vec(vec![
            Value::symbol("foo"),
            Value::symbol("-"),
            Value::symbol("Bar"),
            Value::symbol("BAZ42"),
        ])];
        assert_eq!(parse(string), Ok(expected_result));
    }

    #[test]
    fn quoted_list() {
        let string = r#"
            '(foo "bar" (42 ()))
        "#;
        let expected_result = vec![language::quote_value(list::from_vec(vec![
            Value::symbol("foo"),
            Value::string("bar"),
            list::from_vec(vec![Value::i64(42), Value::nil()]),
        ]))];
        assert_eq!(parse(string), Ok(expected_result));
    }

    #[test]
    fn boolean_expression() {
        let string = r#"
            (!= true (&& false false))
        "#;
        let expected_result = vec![list::from_vec(vec![
            Value::symbol("!="),
            Value::bool(true),
            list::from_vec(vec![
                Value::symbol("&&"),
                Value::bool(false),
                Value::bool(false),
            ]),
        ])];
        assert_eq!(parse(string), Ok(expected_result));
    }

    fn round_trip_helper(string: &str) {
        let ast0 = parse(string).unwrap();
        let pretty = format!("{}", ast0[0]);
        let ast1 = parse(pretty.as_ref()).unwrap();
        assert_eq!(ast0, ast1);
    }

    #[test]
    fn round_trip_simple_list() {
        round_trip_helper(
            r#"
            (foo bar baz)
        "#,
        );
    }

    #[test]
    fn round_trip_factorial() {
        round_trip_helper(
            r#"
            (define factorial (lambda (n)
                (if (= n 0)
                    1
                    (* n (factorial (- n 1))))))
        "#,
        );
    }

    #[test]
    fn round_trip_quote() {
        round_trip_helper(
            r#"
            '(1 2 3)
        "#,
        );
    }
}
