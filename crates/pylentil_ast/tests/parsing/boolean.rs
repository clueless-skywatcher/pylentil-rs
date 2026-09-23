use super::*;
const F: &str = "parser/boolean.py";

#[test]
fn and_and_or() {
    assert_eq!(e(F, "and_expression"), "(and a b)");
    assert_eq!(e(F, "or_expression"), "(or a b)");
}

#[test]
fn and_binds_tighter_than_or() {
    assert_eq!(e(F, "and_binds_tighter_than_or"), "(or a (and b c))");
    assert_eq!(e(F, "and_on_the_left"), "(or (and a b) c)");
    assert_eq!(e(F, "parentheses_override_precedence"), "(and (or a b) c)");
}

#[test]
fn not_binds_tighter_than_and() {
    assert_eq!(e(F, "not_binds_tighter_than_and"), "(and (not a) b)");
    assert_eq!(e(F, "double_not"), "(not (not a))");
    assert_eq!(e(F, "not_with_parentheses"), "(not (and a b))");
}

#[test]
fn a_chain_collapses_into_one_node() {
    // Python flattens a and b and c into a single BoolOp with three values.
    assert_eq!(e(F, "chained_and"), "(and a b c)");
    assert_eq!(e(F, "chained_or"), "(or a b c)");
}

#[test]
fn comparison_binds_tighter_than_and() {
    assert_eq!(e(F, "comparison_in_boolean"), "(and (compare a < b) (compare c < d))");
}

#[test]
fn conditional_expressions() {
    assert_eq!(e(F, "ternary"), "(ifexp b a c)");
    assert_eq!(e(F, "ternary_nested"), "(ifexp b a (ifexp d c e))");
    assert_eq!(e(F, "ternary_with_arithmetic"), "(ifexp c (+ 1 2) 3)");
}

#[test]
fn a_missing_operand_is_rejected() {
    assert_rejected(F, "and_missing_operand");
}
