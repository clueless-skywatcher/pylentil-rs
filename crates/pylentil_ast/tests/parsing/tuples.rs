use super::*;
const F: &str = "parser/tuples.py";

#[test]
fn parse_tuples_bare_pair() {
    assert_eq!(e(F, "bare_pair"), "(tuple a b)");
}

#[test]
fn parse_tuples_bare_triple() {
    // Commas are one flat sequence, not nested pairs.
    assert_eq!(e(F, "bare_triple"), "(tuple a b c)");
}

#[test]
fn parse_tuples_bare_four() {
    assert_eq!(e(F, "bare_four"), "(tuple a b c d)");
}

#[test]
fn parse_tuples_trailing_comma() {
    assert_eq!(e(F, "trailing_comma"), "(tuple a)");
}

#[test]
fn parse_tuples_parenthesized_pair() {
    assert_eq!(e(F, "parenthesized_pair"), "(ptuple a b)");
}

#[test]
fn parse_tuples_parenthesized_single_is_not_a_tuple() {
    assert_eq!(e(F, "parenthesized_single_is_not_a_tuple"), "a");
}

#[test]
fn parse_tuples_single_element_tuple() {
    assert_eq!(e(F, "single_element_tuple"), "(ptuple a)");
}

#[test]
fn parse_tuples_empty_tuple() {
    assert_eq!(e(F, "empty_tuple"), "(ptuple )");
}

#[test]
fn parse_tuples_nested_on_the_left() {
    assert_eq!(e(F, "nested_on_the_left"), "(tuple (ptuple a b) c)");
}

#[test]
fn parse_tuples_nested_on_the_right() {
    assert_eq!(e(F, "nested_on_the_right"), "(tuple a (ptuple b c))");
}

#[test]
fn parse_tuples_nested_both_sides() {
    assert_eq!(e(F, "nested_both_sides"), "(tuple (ptuple a b) (ptuple c d))");
}

#[test]
fn parse_tuples_deeply_nested() {
    assert_eq!(e(F, "deeply_nested"), "(ptuple (ptuple a b) (ptuple c (ptuple d e)))");
}

#[test]
fn parse_tuples_tuple_of_expressions() {
    assert_eq!(e(F, "tuple_of_expressions"), "(tuple (+ 1 2) (* 3 4))");
}

#[test]
fn parse_tuples_tuple_of_literals() {
    assert_eq!(e(F, "tuple_of_literals"), "(tuple 1 str(two) True None)");
}

#[test]
fn parse_tuples_tuple_with_comparison() {
    assert_eq!(e(F, "tuple_with_comparison"), "(tuple (compare a < b) c)");
}

#[test]
fn parse_tuples_redundant_parentheses() {
    assert_eq!(e(F, "redundant_parentheses"), "(ptuple a b)");
}

#[test]
fn parse_tuples_leading_comma() {
    assert_rejected(F, "leading_comma");
}

#[test]
fn parse_tuples_double_comma() {
    assert_rejected(F, "double_comma");
}
