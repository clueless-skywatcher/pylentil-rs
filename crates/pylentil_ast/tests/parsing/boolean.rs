use super::*;
const F: &str = "parser/boolean.py";

#[test]
fn parse_boolean_and_expression() {
    assert_eq!(e(F, "and_expression"), "(and a b)");
}

#[test]
fn parse_boolean_or_expression() {
    assert_eq!(e(F, "or_expression"), "(or a b)");
}

#[test]
fn parse_boolean_and_binds_tighter_than_or() {
    assert_eq!(e(F, "and_binds_tighter_than_or"), "(or a (and b c))");
}

#[test]
fn parse_boolean_and_on_the_left() {
    assert_eq!(e(F, "and_on_the_left"), "(or (and a b) c)");
}

#[test]
fn parse_boolean_not_binds_tighter_than_and() {
    assert_eq!(e(F, "not_binds_tighter_than_and"), "(and (not a) b)");
}

#[test]
fn parse_boolean_chained_and() {
    // Python flattens a and b and c into a single BoolOp with three values.
    assert_eq!(e(F, "chained_and"), "(and a b c)");
}

#[test]
fn parse_boolean_chained_or() {
    assert_eq!(e(F, "chained_or"), "(or a b c)");
}

#[test]
fn parse_boolean_comparison_in_boolean() {
    assert_eq!(e(F, "comparison_in_boolean"), "(and (compare a < b) (compare c < d))");
}

#[test]
fn parse_boolean_parentheses_override_precedence() {
    assert_eq!(e(F, "parentheses_override_precedence"), "(and (or a b) c)");
}

#[test]
fn parse_boolean_double_not() {
    assert_eq!(e(F, "double_not"), "(not (not a))");
}

#[test]
fn parse_boolean_not_with_parentheses() {
    assert_eq!(e(F, "not_with_parentheses"), "(not (and a b))");
}

#[test]
fn parse_boolean_ternary() {
    assert_eq!(e(F, "ternary"), "(ifexp b a c)");
}

#[test]
fn parse_boolean_ternary_nested() {
    assert_eq!(e(F, "ternary_nested"), "(ifexp b a (ifexp d c e))");
}

#[test]
fn parse_boolean_ternary_with_arithmetic() {
    assert_eq!(e(F, "ternary_with_arithmetic"), "(ifexp c (+ 1 2) 3)");
}

#[test]
fn parse_boolean_and_missing_operand() {
    assert_rejected(F, "and_missing_operand");
}
