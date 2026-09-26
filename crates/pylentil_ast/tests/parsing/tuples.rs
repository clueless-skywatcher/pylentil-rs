use super::*;
const F: &str = "parser/tuples.py";

#[test]
fn parse_tuples_bare_pair() {
    assert_eq!(expr(F, "bare_pair"), tuple(vec![name("a"), name("b")]));
}

#[test]
fn parse_tuples_bare_triple() {
    // Commas are one flat sequence, not nested pairs.
    assert_eq!(
        expr(F, "bare_triple"),
        tuple(vec![name("a"), name("b"), name("c")])
    );
}

#[test]
fn parse_tuples_bare_four() {
    assert_eq!(
        expr(F, "bare_four"),
        tuple(vec![name("a"), name("b"), name("c"), name("d")])
    );
}

#[test]
fn parse_tuples_trailing_comma() {
    assert_eq!(expr(F, "trailing_comma"), tuple(vec![name("a")]));
}

#[test]
fn parse_tuples_parenthesized_pair() {
    assert_eq!(
        expr(F, "parenthesized_pair"),
        parenthesized_tuple(vec![name("a"), name("b")])
    );
}

#[test]
fn parse_tuples_parenthesized_single_is_not_a_tuple() {
    assert_eq!(expr(F, "parenthesized_single_is_not_a_tuple"), name("a"));
}

#[test]
fn parse_tuples_single_element_tuple() {
    assert_eq!(
        expr(F, "single_element_tuple"),
        parenthesized_tuple(vec![name("a")])
    );
}

#[test]
fn parse_tuples_empty_tuple() {
    assert_eq!(expr(F, "empty_tuple"), parenthesized_tuple(vec![]));
}

#[test]
fn parse_tuples_nested_on_the_left() {
    assert_eq!(
        expr(F, "nested_on_the_left"),
        tuple(vec![
            parenthesized_tuple(vec![name("a"), name("b")]),
            name("c")
        ])
    );
}

#[test]
fn parse_tuples_nested_on_the_right() {
    assert_eq!(
        expr(F, "nested_on_the_right"),
        tuple(vec![
            name("a"),
            parenthesized_tuple(vec![name("b"), name("c")])
        ])
    );
}

#[test]
fn parse_tuples_nested_both_sides() {
    assert_eq!(
        expr(F, "nested_both_sides"),
        tuple(vec![
            parenthesized_tuple(vec![name("a"), name("b")]),
            parenthesized_tuple(vec![name("c"), name("d")])
        ])
    );
}

#[test]
fn parse_tuples_deeply_nested() {
    assert_eq!(
        expr(F, "deeply_nested"),
        parenthesized_tuple(vec![
            parenthesized_tuple(vec![name("a"), name("b")]),
            parenthesized_tuple(vec![
                name("c"),
                parenthesized_tuple(vec![name("d"), name("e")])
            ])
        ])
    );
}

#[test]
fn parse_tuples_tuple_of_expressions() {
    assert_eq!(
        expr(F, "tuple_of_expressions"),
        tuple(vec![
            bin_op(int(1), PyBinaryOp::Add, int(2)),
            bin_op(int(3), PyBinaryOp::Mul, int(4))
        ])
    );
}

#[test]
fn parse_tuples_tuple_of_literals() {
    assert_eq!(
        expr(F, "tuple_of_literals"),
        tuple(vec![int(1), string("two"), boolean(true), none()])
    );
}

#[test]
fn parse_tuples_tuple_with_comparison() {
    assert_eq!(
        expr(F, "tuple_with_comparison"),
        tuple(vec![
            compare(name("a"), vec![PyComparisonOp::Lt], vec![name("b")]),
            name("c")
        ])
    );
}

#[test]
fn parse_tuples_redundant_parentheses() {
    assert_eq!(
        expr(F, "redundant_parentheses"),
        parenthesized_tuple(vec![name("a"), name("b")])
    );
}

#[test]
fn parse_tuples_leading_comma() {
    assert_rejected(F, "leading_comma");
}

#[test]
fn parse_tuples_double_comma() {
    assert_rejected(F, "double_comma");
}
