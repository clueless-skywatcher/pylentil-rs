use super::*;
const F: &str = "parser/literals.py";

#[test]
fn parse_literals_integer() {
    p_assert_eq!(expr(F, "integer"), int(42));
}

#[test]
fn parse_literals_zero() {
    p_assert_eq!(expr(F, "zero"), int(0));
}

#[test]
fn parse_literals_float() {
    p_assert_eq!(expr(F, "float"), float("3.5"));
}

#[test]
fn parse_literals_float_trailing_dot() {
    p_assert_eq!(expr(F, "float_trailing_dot"), float("2.0"));
}

#[test]
fn parse_literals_string() {
    p_assert_eq!(expr(F, "string"), string("text"));
}

#[test]
fn parse_literals_empty_string() {
    p_assert_eq!(expr(F, "empty_string"), string(""));
}

#[test]
fn parse_literals_single_quoted_string() {
    p_assert_eq!(expr(F, "single_quoted_string"), string("text"));
}

#[test]
fn parse_literals_true() {
    p_assert_eq!(expr(F, "true"), boolean(true));
}

#[test]
fn parse_literals_false() {
    p_assert_eq!(expr(F, "false"), boolean(false));
}

#[test]
fn parse_literals_none() {
    p_assert_eq!(expr(F, "none"), none());
}

#[test]
fn parse_literals_name() {
    p_assert_eq!(expr(F, "name"), name("value"));
}

#[test]
fn parse_literals_huge_integer() {
    // Python integers are arbitrary precision; overflowing i64 is not a
    // syntax error.
    p_assert_eq!(expr(F, "huge_integer"), int("99999999999999999999"));
}

#[test]
fn parse_literals_negative_literal() {
    p_assert_eq!(
        expr(F, "negative_literal"),
        unary_op(PyUnaryOp::UnarySub, int(1))
    );
}

#[test]
fn parse_literals_parenthesized_literal() {
    p_assert_eq!(expr(F, "parenthesized_literal"), int(42));
}

#[test]
fn parse_literals_doubly_parenthesized() {
    p_assert_eq!(expr(F, "doubly_parenthesized"), int(42));
}

#[test]
fn parse_literals_ellipsis() {
    p_assert_eq!(expr(F, "ellipsis"), ellipsis());
}

#[test]
fn parse_literals_escape_in_string() {
    p_assert_eq!(expr(F, "escape_in_string"), string("tab\\there"));
}
