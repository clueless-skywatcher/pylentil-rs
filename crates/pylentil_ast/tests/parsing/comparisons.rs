use super::*;
const F: &str = "parser/comparisons.py";

#[test]
fn parse_comparisons_equal() {
    assert_eq!(e(F, "equal"), "(compare a == b)");
}

#[test]
fn parse_comparisons_not_equal() {
    assert_eq!(e(F, "not_equal"), "(compare a != b)");
}

#[test]
fn parse_comparisons_less_than() {
    assert_eq!(e(F, "less_than"), "(compare a < b)");
}

#[test]
fn parse_comparisons_greater_or_equal() {
    assert_eq!(e(F, "greater_or_equal"), "(compare a >= b)");
}

#[test]
fn parse_comparisons_chained() {
    // Python folds a < b < c into a single Compare, not nested ones.
    assert_eq!(e(F, "chained"), "(compare a < b < c)");
}

#[test]
fn parse_comparisons_chained_mixed_operators() {
    assert_eq!(e(F, "chained_mixed_operators"), "(compare a < b == c)");
}

#[test]
fn parse_comparisons_chained_three_operators() {
    assert_eq!(e(F, "chained_three_operators"), "(compare a < b <= c < d)");
}

#[test]
fn parse_comparisons_comparison_binds_looser_than_addition() {
    assert_eq!(e(F, "comparison_binds_looser_than_addition"), "(compare (+ a 1) == b)");
}

#[test]
fn parse_comparisons_comparison_on_both_sides() {
    assert_eq!(e(F, "comparison_on_both_sides"), "(compare (+ a 1) == (* b 2))");
}

#[test]
fn parse_comparisons_chained_with_arithmetic() {
    assert_eq!(e(F, "chained_with_arithmetic"), "(compare 1 < (+ x 1) < 10)");
}

#[test]
fn parse_comparisons_is_operator() {
    assert_eq!(e(F, "is_operator"), "(compare a is b)");
}

#[test]
fn parse_comparisons_is_not_operator() {
    assert_eq!(e(F, "is_not_operator"), "(compare a is-not b)");
}

#[test]
fn parse_comparisons_in_operator() {
    assert_eq!(e(F, "in_operator"), "(compare a in b)");
}

#[test]
fn parse_comparisons_not_in_operator() {
    assert_eq!(e(F, "not_in_operator"), "(compare a not-in b)");
}

#[test]
fn parse_comparisons_not_operator() {
    assert_eq!(e(F, "not_operator"), "(not a)");
}

#[test]
fn parse_comparisons_not_with_comparison() {
    assert_eq!(e(F, "not_with_comparison"), "(not (compare a == b))");
}

#[test]
fn parse_comparisons_parenthesized_comparison() {
    assert_eq!(e(F, "parenthesized_comparison"), "(compare (compare a < b) < c)");
}

#[test]
fn parse_comparisons_missing_right_operand() {
    assert_rejected(F, "missing_right_operand");
}

#[test]
fn parse_comparisons_double_operator() {
    assert_rejected(F, "double_operator");
}
