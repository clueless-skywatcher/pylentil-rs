use super::*;
const F: &str = "parser/subscripts.py";

#[test]
fn subscripts() {
    assert_eq!(e(F, "subscript"), "(subscript a 0)");
    assert_eq!(e(F, "chained_subscript"), "(subscript (subscript a 0) 1)");
    assert_eq!(e(F, "subscript_with_expression"), "(subscript a (+ i 1))");
    assert_eq!(e(F, "subscript_of_call"), "(subscript (call f) 0)");
    assert_eq!(e(F, "call_on_subscript"), "(call (subscript a 0))");
}

#[test]
fn slices() {
    assert_eq!(e(F, "slice"), "(subscript a (slice 1 2 -))");
    assert_eq!(e(F, "slice_with_step"), "(subscript a (slice 1 2 3))");
    assert_eq!(e(F, "open_slice"), "(subscript a (slice - - -))");
}

/// Any of the three parts may be left out, in any combination.
#[test]
fn a_slice_part_may_be_omitted() {
    assert_eq!(e(F, "slice_lower_only"), "(subscript a (slice 1 - -))");
    assert_eq!(e(F, "slice_upper_only"), "(subscript a (slice - 2 -))");
    assert_eq!(e(F, "slice_trailing_colon"), "(subscript a (slice 1 2 -))");
    assert_eq!(e(F, "open_slice_both_colons"), "(subscript a (slice - - -))");
}

/// An omitted `upper` still has to let the second colon through, so the
/// step is reached rather than parsed as an expression starting at `:`.
#[test]
fn a_step_survives_an_omitted_upper() {
    assert_eq!(e(F, "slice_step_only"), "(subscript a (slice - - 2))");
    assert_eq!(e(F, "slice_omitted_upper_with_step"), "(subscript a (slice 1 - 2))");
    assert_eq!(e(F, "slice_omitted_lower_with_step"), "(subscript a (slice - 2 3))");
}

#[test]
fn slice_bounds_are_full_expressions() {
    assert_eq!(e(F, "slice_bounds_are_expressions"), "(subscript a (slice (+ x 1) (* y 2) -))");
    assert_eq!(e(F, "slice_bounds_are_calls"), "(subscript a (slice (call f 1) (call g 2) -))");
    assert_eq!(e(F, "slice_bound_is_a_subscript"), "(subscript a (slice (subscript b 0) c -))");
}

#[test]
fn a_comma_index_is_a_tuple_not_a_slice() {
    assert_eq!(e(F, "tuple_index"), "(subscript a (tuple 1 2))");
}

#[test]
fn malformed_subscripts_are_rejected() {
    assert_rejected(F, "empty_subscript");
    assert_rejected(F, "too_many_slice_parts");
}
