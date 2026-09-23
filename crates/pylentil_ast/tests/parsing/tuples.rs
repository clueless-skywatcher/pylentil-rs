use super::*;
const F: &str = "parser/tuples.py";

#[test]
fn a_bare_pair() {
    assert_eq!(e(F, "bare_pair"), "(tuple a b)");
}

#[test]
fn a_parenthesized_pair_is_marked_parenthesized() {
    assert_eq!(e(F, "parenthesized_pair"), "(ptuple a b)");
}

#[test]
fn parentheses_alone_do_not_make_a_tuple() {
    assert_eq!(e(F, "parenthesized_single_is_not_a_tuple"), "a");
}

#[test]
fn nesting_is_preserved() {
    assert_eq!(e(F, "nested_on_the_left"), "(tuple (ptuple a b) c)");
    assert_eq!(e(F, "nested_on_the_right"), "(tuple a (ptuple b c))");
    assert_eq!(e(F, "nested_both_sides"), "(tuple (ptuple a b) (ptuple c d))");
    assert_eq!(e(F, "deeply_nested"), "(ptuple (ptuple a b) (ptuple c (ptuple d e)))");
}

#[test]
fn elements_may_be_expressions() {
    assert_eq!(e(F, "tuple_of_expressions"), "(tuple (+ 1 2) (* 3 4))");
    assert_eq!(e(F, "tuple_of_literals"), "(tuple 1 str(two) True None)");
    assert_eq!(e(F, "tuple_with_comparison"), "(tuple (compare a < b) c)");
}

#[test]
fn a_longer_tuple_stays_flat() {
    // Commas are one flat sequence, not nested pairs.
    assert_eq!(e(F, "bare_triple"), "(tuple a b c)");
    assert_eq!(e(F, "bare_four"), "(tuple a b c d)");
}

#[test]
fn a_trailing_comma_makes_a_one_element_tuple() {
    assert_eq!(e(F, "trailing_comma"), "(tuple a)");
    assert_eq!(e(F, "single_element_tuple"), "(ptuple a)");
}

#[test]
fn the_empty_tuple() {
    assert_eq!(e(F, "empty_tuple"), "(ptuple )");
}

#[test]
fn redundant_parentheses_do_not_nest() {
    assert_eq!(e(F, "redundant_parentheses"), "(ptuple a b)");
}

#[test]
fn malformed_tuples_are_rejected() {
    assert_rejected(F, "leading_comma");
    assert_rejected(F, "double_comma");
}
