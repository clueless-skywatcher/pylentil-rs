use super::*;
const F: &str = "parser/comparisons.py";

#[test]
fn parse_comparisons_equal() {
    assert_eq!(
        expr(F, "equal"),
        compare(name("a"), vec![PyComparisonOp::Eq], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_not_equal() {
    assert_eq!(
        expr(F, "not_equal"),
        compare(name("a"), vec![PyComparisonOp::NotEq], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_less_than() {
    assert_eq!(
        expr(F, "less_than"),
        compare(name("a"), vec![PyComparisonOp::Lt], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_greater_or_equal() {
    assert_eq!(
        expr(F, "greater_or_equal"),
        compare(name("a"), vec![PyComparisonOp::Gte], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_chained() {
    // Python folds a < b < c into a single Compare, not nested ones.
    assert_eq!(
        expr(F, "chained"),
        compare(
            name("a"),
            vec![PyComparisonOp::Lt, PyComparisonOp::Lt],
            vec![name("b"), name("c")]
        )
    );
}

#[test]
fn parse_comparisons_chained_mixed_operators() {
    assert_eq!(
        expr(F, "chained_mixed_operators"),
        compare(
            name("a"),
            vec![PyComparisonOp::Lt, PyComparisonOp::Eq],
            vec![name("b"), name("c")]
        )
    );
}

#[test]
fn parse_comparisons_chained_three_operators() {
    assert_eq!(
        expr(F, "chained_three_operators"),
        compare(
            name("a"),
            vec![PyComparisonOp::Lt, PyComparisonOp::Lte, PyComparisonOp::Lt],
            vec![name("b"), name("c"), name("d")]
        )
    );
}

#[test]
fn parse_comparisons_comparison_binds_looser_than_addition() {
    assert_eq!(
        expr(F, "comparison_binds_looser_than_addition"),
        compare(
            bin_op(name("a"), PyBinaryOp::Add, int(1)),
            vec![PyComparisonOp::Eq],
            vec![name("b")]
        )
    );
}

#[test]
fn parse_comparisons_comparison_on_both_sides() {
    assert_eq!(
        expr(F, "comparison_on_both_sides"),
        compare(
            bin_op(name("a"), PyBinaryOp::Add, int(1)),
            vec![PyComparisonOp::Eq],
            vec![bin_op(name("b"), PyBinaryOp::Mul, int(2))]
        )
    );
}

#[test]
fn parse_comparisons_chained_with_arithmetic() {
    assert_eq!(
        expr(F, "chained_with_arithmetic"),
        compare(
            int(1),
            vec![PyComparisonOp::Lt, PyComparisonOp::Lt],
            vec![bin_op(name("x"), PyBinaryOp::Add, int(1)), int(10)]
        )
    );
}

#[test]
fn parse_comparisons_is_operator() {
    assert_eq!(
        expr(F, "is_operator"),
        compare(name("a"), vec![PyComparisonOp::Is], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_is_not_operator() {
    assert_eq!(
        expr(F, "is_not_operator"),
        compare(name("a"), vec![PyComparisonOp::IsNot], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_in_operator() {
    assert_eq!(
        expr(F, "in_operator"),
        compare(name("a"), vec![PyComparisonOp::In], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_not_in_operator() {
    assert_eq!(
        expr(F, "not_in_operator"),
        compare(name("a"), vec![PyComparisonOp::NotIn], vec![name("b")])
    );
}

#[test]
fn parse_comparisons_not_operator() {
    assert_eq!(expr(F, "not_operator"), unary_op(PyUnaryOp::Not, name("a")));
}

#[test]
fn parse_comparisons_not_with_comparison() {
    assert_eq!(
        expr(F, "not_with_comparison"),
        unary_op(
            PyUnaryOp::Not,
            compare(name("a"), vec![PyComparisonOp::Eq], vec![name("b")])
        )
    );
}

#[test]
fn parse_comparisons_parenthesized_comparison() {
    assert_eq!(
        expr(F, "parenthesized_comparison"),
        compare(
            compare(name("a"), vec![PyComparisonOp::Lt], vec![name("b")]),
            vec![PyComparisonOp::Lt],
            vec![name("c")]
        )
    );
}

#[test]
fn parse_comparisons_missing_right_operand() {
    assert_rejected(F, "missing_right_operand");
}

#[test]
fn parse_comparisons_double_operator() {
    assert_rejected(F, "double_operator");
}
