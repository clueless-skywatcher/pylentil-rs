use super::*;
const F: &str = "parser/literals.py";

#[test]
fn numbers_and_text() {
    assert_eq!(e(F, "integer"), "42");
    assert_eq!(e(F, "zero"), "0");
    assert_eq!(e(F, "float"), "3.5");
    assert_eq!(e(F, "float_trailing_dot"), "2.0");
    assert_eq!(e(F, "string"), "str(text)");
    assert_eq!(e(F, "single_quoted_string"), "str(text)");
    assert_eq!(e(F, "empty_string"), "str()");
}

#[test]
fn constants_and_names() {
    assert_eq!(e(F, "true"), "True");
    assert_eq!(e(F, "false"), "False");
    assert_eq!(e(F, "none"), "None");
    assert_eq!(e(F, "name"), "value");
}

#[test]
fn an_escape_sequence_stays_inside_the_string() {
    assert_eq!(e(F, "escape_in_string"), "str(tab\\there)");
}

#[test]
fn a_negative_literal() {
    assert_eq!(e(F, "negative_literal"), "(neg 1)");
}

#[test]
fn an_integer_too_large_for_i64_is_still_an_integer() {
    // Python integers are arbitrary precision; overflowing i64 is not a
    // syntax error.
    assert!(
        !e(F, "huge_integer").starts_with("<rejected"),
        "99999999999999999999 must parse, not be rejected as invalid syntax"
    );
}

#[test]
fn ellipsis_is_a_constant() {
    assert_eq!(e(F, "ellipsis"), "...");
}
