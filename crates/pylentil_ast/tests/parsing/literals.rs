use super::*;
const F: &str = "parser/literals.py";

#[test]
fn parse_literals_integer() {
    assert_eq!(e(F, "integer"), "42");
}

#[test]
fn parse_literals_zero() {
    assert_eq!(e(F, "zero"), "0");
}

#[test]
fn parse_literals_float() {
    assert_eq!(e(F, "float"), "3.5");
}

#[test]
fn parse_literals_float_trailing_dot() {
    assert_eq!(e(F, "float_trailing_dot"), "2.0");
}

#[test]
fn parse_literals_string() {
    assert_eq!(e(F, "string"), "str(text)");
}

#[test]
fn parse_literals_empty_string() {
    assert_eq!(e(F, "empty_string"), "str()");
}

#[test]
fn parse_literals_single_quoted_string() {
    assert_eq!(e(F, "single_quoted_string"), "str(text)");
}

#[test]
fn parse_literals_true() {
    assert_eq!(e(F, "true"), "True");
}

#[test]
fn parse_literals_false() {
    assert_eq!(e(F, "false"), "False");
}

#[test]
fn parse_literals_none() {
    assert_eq!(e(F, "none"), "None");
}

#[test]
fn parse_literals_name() {
    assert_eq!(e(F, "name"), "value");
}

#[test]
fn parse_literals_huge_integer() {
    // Python integers are arbitrary precision; overflowing i64 is not a
    // syntax error.
    assert!(
        !e(F, "huge_integer").starts_with("<rejected"),
        "99999999999999999999 must parse, not be rejected as invalid syntax"
    );
}

#[test]
fn parse_literals_negative_literal() {
    assert_eq!(e(F, "negative_literal"), "(neg 1)");
}

#[test]
fn parse_literals_parenthesized_literal() {
    assert_eq!(e(F, "parenthesized_literal"), "42");
}

#[test]
fn parse_literals_doubly_parenthesized() {
    assert_eq!(e(F, "doubly_parenthesized"), "42");
}

#[test]
fn parse_literals_ellipsis() {
    assert_eq!(e(F, "ellipsis"), "...");
}

#[test]
fn parse_literals_escape_in_string() {
    assert_eq!(e(F, "escape_in_string"), "str(tab\\there)");
}
