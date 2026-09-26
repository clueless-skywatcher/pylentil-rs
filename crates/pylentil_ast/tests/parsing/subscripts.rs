use super::*;
const F: &str = "parser/subscripts.py";

#[test]
fn parse_subscripts_subscript() {
    assert_eq!(expr(F, "subscript"), subscript(name("a"), int(0)));
}

#[test]
fn parse_subscripts_chained_subscript() {
    assert_eq!(
        expr(F, "chained_subscript"),
        subscript(subscript(name("a"), int(0)), int(1))
    );
}

#[test]
fn parse_subscripts_subscript_with_expression() {
    assert_eq!(
        expr(F, "subscript_with_expression"),
        subscript(name("a"), bin_op(name("i"), PyBinaryOp::Add, int(1)))
    );
}

#[test]
fn parse_subscripts_slice() {
    assert_eq!(
        expr(F, "slice"),
        subscript(name("a"), slice(Some(int(1)), Some(int(2)), None))
    );
}

#[test]
fn parse_subscripts_slice_with_step() {
    assert_eq!(
        expr(F, "slice_with_step"),
        subscript(name("a"), slice(Some(int(1)), Some(int(2)), Some(int(3))))
    );
}

#[test]
fn parse_subscripts_open_slice() {
    assert_eq!(
        expr(F, "open_slice"),
        subscript(name("a"), slice(None, None, None))
    );
}

#[test]
fn parse_subscripts_subscript_of_call() {
    assert_eq!(
        expr(F, "subscript_of_call"),
        subscript(call(name("f"), vec![], vec![]), int(0))
    );
}

#[test]
fn parse_subscripts_call_on_subscript() {
    assert_eq!(
        expr(F, "call_on_subscript"),
        call(subscript(name("a"), int(0)), vec![], vec![])
    );
}

/// Any of the three parts may be left out, in any combination.
#[test]
fn parse_subscripts_slice_lower_only() {
    assert_eq!(
        expr(F, "slice_lower_only"),
        subscript(name("a"), slice(Some(int(1)), None, None))
    );
}

#[test]
fn parse_subscripts_slice_upper_only() {
    assert_eq!(
        expr(F, "slice_upper_only"),
        subscript(name("a"), slice(None, Some(int(2)), None))
    );
}

#[test]
fn parse_subscripts_slice_trailing_colon() {
    assert_eq!(
        expr(F, "slice_trailing_colon"),
        subscript(name("a"), slice(Some(int(1)), Some(int(2)), None))
    );
}

#[test]
fn parse_subscripts_open_slice_both_colons() {
    assert_eq!(
        expr(F, "open_slice_both_colons"),
        subscript(name("a"), slice(None, None, None))
    );
}

/// An omitted `upper` still has to let the second colon through, so the
/// step is reached rather than parsed as an expression starting at `:`.
#[test]
fn parse_subscripts_slice_step_only() {
    assert_eq!(
        expr(F, "slice_step_only"),
        subscript(name("a"), slice(None, None, Some(int(2))))
    );
}

#[test]
fn parse_subscripts_slice_omitted_upper_with_step() {
    assert_eq!(
        expr(F, "slice_omitted_upper_with_step"),
        subscript(name("a"), slice(Some(int(1)), None, Some(int(2))))
    );
}

#[test]
fn parse_subscripts_slice_omitted_lower_with_step() {
    assert_eq!(
        expr(F, "slice_omitted_lower_with_step"),
        subscript(name("a"), slice(None, Some(int(2)), Some(int(3))))
    );
}

#[test]
fn parse_subscripts_slice_bounds_are_expressions() {
    assert_eq!(
        expr(F, "slice_bounds_are_expressions"),
        subscript(
            name("a"),
            slice(
                Some(bin_op(name("x"), PyBinaryOp::Add, int(1))),
                Some(bin_op(name("y"), PyBinaryOp::Mul, int(2))),
                None
            )
        )
    );
}

#[test]
fn parse_subscripts_slice_bounds_are_calls() {
    assert_eq!(
        expr(F, "slice_bounds_are_calls"),
        subscript(
            name("a"),
            slice(
                Some(call(name("f"), vec![int(1)], vec![])),
                Some(call(name("g"), vec![int(2)], vec![])),
                None
            )
        )
    );
}

#[test]
fn parse_subscripts_slice_bound_is_a_subscript() {
    assert_eq!(
        expr(F, "slice_bound_is_a_subscript"),
        subscript(
            name("a"),
            slice(Some(subscript(name("b"), int(0))), Some(name("c")), None)
        )
    );
}

#[test]
fn parse_subscripts_tuple_index() {
    assert_eq!(
        expr(F, "tuple_index"),
        subscript(name("a"), tuple(vec![int(1), int(2)]))
    );
}

#[test]
fn parse_subscripts_empty_subscript() {
    assert_rejected(F, "empty_subscript");
}

#[test]
fn parse_subscripts_too_many_slice_parts() {
    assert_rejected(F, "too_many_slice_parts");
}
