use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map, value, verify},
    multi::fold_many0,
    sequence::{delimited, preceded},
    IResult,
};

fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    preceded(
        char('\\'),
        alt((
            value('\n', char('n')),
            value('\\', char('\\')),
            value('"', char('"')),
        )),
    )(input)
}

fn parse_string_literal_contents_non_empty(input: &str) -> IResult<&str, &str> {
    verify(is_not("\\\""), |s: &str| !s.is_empty())(input)
}

enum StringContentsFragment<'a> {
    EscapedChar(char),
    LiteralContentsNonEmpty(&'a str),
}

fn parse_string_contents_fragment(input: &str) -> IResult<&str, StringContentsFragment> {
    alt((
        map(parse_escaped_char, StringContentsFragment::EscapedChar),
        map(
            parse_string_literal_contents_non_empty,
            StringContentsFragment::LiteralContentsNonEmpty,
        ),
    ))(input)
}

fn parse_string_contents(input: &str) -> IResult<&str, String> {
    fold_many0(
        parse_string_contents_fragment,
        String::new,
        |mut string, string_contents_fragment| {
            match string_contents_fragment {
                StringContentsFragment::EscapedChar(c) => string.push(c),
                StringContentsFragment::LiteralContentsNonEmpty(s) => string.push_str(s),
            }
            string
        },
    )(input)
}

pub fn parse_string(input: &str) -> IResult<&str, String> {
    delimited(char('"'), parse_string_contents, char('"'))(input)
}

#[test]
fn test_parse_string() {
    use nom::{error::Error, error::ErrorKind, Err};

    assert_eq!(parse_string(r#""""#), Ok(("", "".to_string())));
    assert_eq!(parse_string(r#""foo""#), Ok(("", "foo".to_string())));
    assert_eq!(
        parse_string(r#""foo\"bar""#),
        Ok(("", "foo\"bar".to_string()))
    );

    assert_eq!(
        parse_string(r#""#),
        Err(Err::Error(Error {
            input: "",
            code: ErrorKind::Char,
        }))
    );

    assert_eq!(
        parse_string(r#"foo"#),
        Err(Err::Error(Error {
            input: "foo",
            code: ErrorKind::Char,
        }))
    );

    assert_eq!(
        parse_string(r#"foo\xbar"#),
        Err(Err::Error(Error {
            input: r#"foo\xbar"#,
            code: ErrorKind::Char,
        }))
    );
}
