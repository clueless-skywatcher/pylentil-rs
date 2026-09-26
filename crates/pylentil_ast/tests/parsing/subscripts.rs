use super::*;
const F: &str = "parser/subscripts.py";

#[test]
fn parse_subscripts_subscript() {
    assert_eq!(e(F, "subscript"), "(subscript a 0)");
}

#[test]
fn parse_subscripts_chained_subscript() {
    assert_eq!(e(F, "chained_subscript"), "(subscript (subscript a 0) 1)");
}

#[test]
fn parse_subscripts_subscript_with_expression() {
    assert_eq!(e(F, "subscript_with_expression"), "(subscript a (+ i 1))");
}

#[test]
fn parse_subscripts_slice() {
    assert_eq!(e(F, "slice"), "(subscript a (slice 1 2 -))");
}

#[test]
fn parse_subscripts_slice_with_step() {
    assert_eq!(e(F, "slice_with_step"), "(subscript a (slice 1 2 3))");
}

#[test]
fn parse_subscripts_open_slice() {
    assert_eq!(e(F, "open_slice"), "(subscript a (slice - - -))");
}

#[test]
fn parse_subscripts_subscript_of_call() {
    assert_eq!(e(F, "subscript_of_call"), "(subscript (call f) 0)");
}

#[test]
fn parse_subscripts_call_on_subscript() {
    assert_eq!(e(F, "call_on_subscript"), "(call (subscript a 0))");
}

/// Any of the three parts may be left out, in any combination.
#[test]
fn parse_subscripts_slice_lower_only() {
    assert_eq!(e(F, "slice_lower_only"), "(subscript a (slice 1 - -))");
}

#[test]
fn parse_subscripts_slice_upper_only() {
    assert_eq!(e(F, "slice_upper_only"), "(subscript a (slice - 2 -))");
}

#[test]
fn parse_subscripts_slice_trailing_colon() {
    assert_eq!(e(F, "slice_trailing_colon"), "(subscript a (slice 1 2 -))");
}

#[test]
fn parse_subscripts_open_slice_both_colons() {
    assert_eq!(e(F, "open_slice_both_colons"), "(subscript a (slice - - -))");
}

/// An omitted `upper` still has to let the second colon through, so the
/// step is reached rather than parsed as an expression starting at `:`.
#[test]
fn parse_subscripts_slice_step_only() {
    assert_eq!(e(F, "slice_step_only"), "(subscript a (slice - - 2))");
}

#[test]
fn parse_subscripts_slice_omitted_upper_with_step() {
    assert_eq!(e(F, "slice_omitted_upper_with_step"), "(subscript a (slice 1 - 2))");
}

#[test]
fn parse_subscripts_slice_omitted_lower_with_step() {
    assert_eq!(e(F, "slice_omitted_lower_with_step"), "(subscript a (slice - 2 3))");
}

#[test]
fn parse_subscripts_slice_bounds_are_expressions() {
    assert_eq!(e(F, "slice_bounds_are_expressions"), "(subscript a (slice (+ x 1) (* y 2) -))");
}

#[test]
fn parse_subscripts_slice_bounds_are_calls() {
    assert_eq!(e(F, "slice_bounds_are_calls"), "(subscript a (slice (call f 1) (call g 2) -))");
}

#[test]
fn parse_subscripts_slice_bound_is_a_subscript() {
    assert_eq!(e(F, "slice_bound_is_a_subscript"), "(subscript a (slice (subscript b 0) c -))");
}

#[test]
fn parse_subscripts_tuple_index() {
    assert_eq!(e(F, "tuple_index"), "(subscript a (tuple 1 2))");
}

#[test]
fn parse_subscripts_empty_subscript() {
    assert_rejected(F, "empty_subscript");
}

#[test]
fn parse_subscripts_too_many_slice_parts() {
    assert_rejected(F, "too_many_slice_parts");
}
