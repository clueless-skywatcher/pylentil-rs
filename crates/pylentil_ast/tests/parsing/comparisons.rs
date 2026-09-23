use super::*;
const F: &str = "parser/comparisons.py";

#[test]
fn simple_comparisons() {
    assert_eq!(e(F, "equal"), "(compare a == b)");
    assert_eq!(e(F, "not_equal"), "(compare a != b)");
    assert_eq!(e(F, "less_than"), "(compare a < b)");
    assert_eq!(e(F, "greater_or_equal"), "(compare a >= b)");
}

#[test]
fn comparisons_chain_into_one_node() {
    // Python folds a < b < c into a single Compare, not nested ones.
    assert_eq!(e(F, "chained"), "(compare a < b < c)");
    assert_eq!(e(F, "chained_mixed_operators"), "(compare a < b == c)");
    assert_eq!(e(F, "chained_three_operators"), "(compare a < b <= c < d)");
}

#[test]
fn arithmetic_binds_tighter_than_comparison() {
    assert_eq!(e(F, "comparison_binds_looser_than_addition"), "(compare (+ a 1) == b)");
    assert_eq!(e(F, "comparison_on_both_sides"), "(compare (+ a 1) == (* b 2))");
    assert_eq!(e(F, "chained_with_arithmetic"), "(compare 1 < (+ x 1) < 10)");
}

#[test]
fn parentheses_stop_a_chain() {
    assert_eq!(e(F, "parenthesized_comparison"), "(compare (compare a < b) < c)");
}

#[test]
fn identity_and_membership_operators() {
    assert_eq!(e(F, "is_operator"), "(compare a is b)");
    assert_eq!(e(F, "is_not_operator"), "(compare a is-not b)");
    assert_eq!(e(F, "in_operator"), "(compare a in b)");
    assert_eq!(e(F, "not_in_operator"), "(compare a not-in b)");
}

#[test]
fn not_binds_looser_than_comparison() {
    assert_eq!(e(F, "not_operator"), "(not a)");
    assert_eq!(e(F, "not_with_comparison"), "(not (compare a == b))");
}

#[test]
fn malformed_comparisons_are_rejected() {
    assert_rejected(F, "missing_right_operand");
    assert_rejected(F, "double_operator");
}
