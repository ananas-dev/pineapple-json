use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::{char, multispace0, one_of},
    combinator::{map, value},
    error::ParseError,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, separated_pair},
};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn string_litteral(i: &str) -> IResult<&str, String> {
    map(
        delimited(
            char('"'),
            alt((escaped(is_not(r#"\""#), '\\', one_of(r#""n\"#)), tag(""))),
            char('"'),
        ),
        String::from,
    )(i)
}

fn json_null(i: &str) -> IResult<&str, JsonValue> {
    value(JsonValue::Null, tag("null"))(i)
}

fn json_bool(i: &str) -> IResult<&str, JsonValue> {
    alt((
        value(JsonValue::Bool(true), tag("true")),
        value(JsonValue::Bool(false), tag("false")),
    ))(i)
}

fn json_number(i: &str) -> IResult<&str, JsonValue> {
    map(double, JsonValue::Number)(i)
}

fn json_string(i: &str) -> IResult<&str, JsonValue> {
    map(string_litteral, JsonValue::String)(i)
}

fn json_array(i: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(char('['), separated_list0(char(','), json_value), char(']')),
        JsonValue::Array,
    )(i)
}

fn json_object(i: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('{'),
            separated_list0(
                char(','),
                separated_pair(ws(string_litteral), char(':'), json_value),
            ),
            char('}'),
        ),
        |obj| JsonValue::Object(obj.into_iter().collect()),
    )(i)
}

pub fn json_value(i: &str) -> IResult<&str, JsonValue> {
    ws(alt((
        json_null,
        json_bool,
        json_number,
        json_string,
        json_array,
        json_object,
    )))(i)
}
