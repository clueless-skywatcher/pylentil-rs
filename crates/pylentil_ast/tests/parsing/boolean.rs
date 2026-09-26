use super::*;
const F: &str = "parser/boolean.py";

#[test]
fn parse_boolean_and_expression() {
    assert_eq!(
        expr(F, "and_expression"),
        bool_op(PyBoolOp::And, vec![name("a"), name("b")])
    );
}

#[test]
fn parse_boolean_or_expression() {
    assert_eq!(
        expr(F, "or_expression"),
        bool_op(PyBoolOp::Or, vec![name("a"), name("b")])
    );
}

#[test]
fn parse_boolean_and_binds_tighter_than_or() {
    assert_eq!(
        expr(F, "and_binds_tighter_than_or"),
        bool_op(
            PyBoolOp::Or,
            vec![
                name("a"),
                bool_op(PyBoolOp::And, vec![name("b"), name("c")])
            ]
        )
    );
}

#[test]
fn parse_boolean_and_on_the_left() {
    assert_eq!(
        expr(F, "and_on_the_left"),
        bool_op(
            PyBoolOp::Or,
            vec![
                bool_op(PyBoolOp::And, vec![name("a"), name("b")]),
                name("c")
            ]
        )
    );
}

#[test]
fn parse_boolean_not_binds_tighter_than_and() {
    assert_eq!(
        expr(F, "not_binds_tighter_than_and"),
        bool_op(
            PyBoolOp::And,
            vec![unary_op(PyUnaryOp::Not, name("a")), name("b")]
        )
    );
}

#[test]
fn parse_boolean_chained_and() {
    // Python flattens a and b and c into a single BoolOp with three values.
    assert_eq!(
        expr(F, "chained_and"),
        bool_op(PyBoolOp::And, vec![name("a"), name("b"), name("c")])
    );
}

#[test]
fn parse_boolean_chained_or() {
    assert_eq!(
        expr(F, "chained_or"),
        bool_op(PyBoolOp::Or, vec![name("a"), name("b"), name("c")])
    );
}

#[test]
fn parse_boolean_comparison_in_boolean() {
    assert_eq!(
        expr(F, "comparison_in_boolean"),
        bool_op(
            PyBoolOp::And,
            vec![
                compare(name("a"), vec![PyComparisonOp::Lt], vec![name("b")]),
                compare(name("c"), vec![PyComparisonOp::Lt], vec![name("d")])
            ]
        )
    );
}

#[test]
fn parse_boolean_parentheses_override_precedence() {
    assert_eq!(
        expr(F, "parentheses_override_precedence"),
        bool_op(
            PyBoolOp::And,
            vec![bool_op(PyBoolOp::Or, vec![name("a"), name("b")]), name("c")]
        )
    );
}

#[test]
fn parse_boolean_double_not() {
    assert_eq!(
        expr(F, "double_not"),
        unary_op(PyUnaryOp::Not, unary_op(PyUnaryOp::Not, name("a")))
    );
}

#[test]
fn parse_boolean_not_with_parentheses() {
    assert_eq!(
        expr(F, "not_with_parentheses"),
        unary_op(
            PyUnaryOp::Not,
            bool_op(PyBoolOp::And, vec![name("a"), name("b")])
        )
    );
}

#[test]
fn parse_boolean_ternary() {
    assert_eq!(expr(F, "ternary"), if_exp(name("b"), name("a"), name("c")));
}

#[test]
fn parse_boolean_ternary_nested() {
    assert_eq!(
        expr(F, "ternary_nested"),
        if_exp(
            name("b"),
            name("a"),
            if_exp(name("d"), name("c"), name("e"))
        )
    );
}

#[test]
fn parse_boolean_ternary_with_arithmetic() {
    assert_eq!(
        expr(F, "ternary_with_arithmetic"),
        if_exp(name("c"), bin_op(int(1), PyBinaryOp::Add, int(2)), int(3))
    );
}

#[test]
fn parse_boolean_and_missing_operand() {
    assert_rejected(F, "and_missing_operand");
}
