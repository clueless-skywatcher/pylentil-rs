use super::*;
const F: &str = "parser/assignment.py";

fn s(name: &str) -> String {
    let code = case(F, name);
    match one_stmt(&code) {
        Ok(rendered) => rendered,
        Err(err) => format!("<rejected: {err:?}>"),
    }
}

#[test]
fn simple_assignment() {
    assert_eq!(s("simple"), "(assign (x) 1)");
    assert_eq!(s("name_value"), "(assign (x) y)");
    assert_eq!(s("expression_value"), "(assign (x) (+ 1 2))");
}

#[test]
fn tuple_unpacking_spreads_the_targets() {
    assert_eq!(s("tuple_unpack"), "(assign (a b) (tuple 1 2))");
    assert_eq!(s("parenthesized_target"), "(assign (a b) (tuple 1 2))");
    assert_eq!(s("swap"), "(assign (a b) (tuple b a))");
}

#[test]
fn a_tuple_value_is_kept_whole() {
    assert_eq!(s("value_is_a_tuple"), "(assign (x) (tuple 1 2))");
}

#[test]
fn nested_unpacking() {
    assert_eq!(s("nested_unpack"), "(assign ((ptuple a b) c) (tuple (ptuple 1 2) 3))");
}

#[test]
fn assignment_targets_are_stores_not_loads() {
    let module = parse_module(&case(F, "simple")).expect("x = 1 must parse");
    let PyStatement::Assign { targets, .. } = &module.body[0] else {
        panic!("expected an assignment, got {:?}", module.body[0]);
    };
    let PyExpr::Name { ctx, .. } = &targets[0] else {
        panic!("expected a name target, got {:?}", targets[0]);
    };
    assert!(
        matches!(ctx, PyRefContext::Store),
        "an assignment target is a Store, got {ctx:?}"
    );
}

#[test]
fn chained_assignment() {
    assert_eq!(s("chained"), "(assign (a b) 1)");
    assert_eq!(s("chained_three"), "(assign (a b c) 1)");
}

#[test]
fn augmented_assignment() {
    assert_eq!(s("augmented_add"), "(augassign x += 1)");
    assert_eq!(s("augmented_mul"), "(augassign x *= 2)");
}

#[test]
fn annotated_assignment() {
    assert_eq!(s("annotated"), "(annassign x int 1)");
    assert_eq!(s("annotation_only"), "(annassign x int -)");
}

#[test]
fn attribute_and_subscript_targets() {
    assert_eq!(s("attribute_target"), "(assign ((attr obj field)) 1)");
    assert_eq!(s("subscript_target"), "(assign ((subscript items 0)) 1)");
}

#[test]
fn a_starred_target() {
    assert_eq!(s("starred_target"), "(assign ((tuple a (star rest))) items)");
}

#[test]
fn the_walrus_operator() {
    assert_eq!(s("walrus"), "(:= n 10)");
}

#[test]
fn an_equality_test_is_not_an_assignment() {
    assert_eq!(s("equality_is_not_assignment"), "(compare x == 1)");
}

#[test]
fn an_assignment_cannot_span_a_newline() {
    // `x` and `= 1` are two lines: Python reports a syntax error rather
    // than joining them into one assignment.
    assert_rejected(F, "assignment_split_across_lines");
}

#[test]
fn malformed_assignments_are_rejected() {
    // assert_rejected(F, "missing_right_hand_side");
    // assert_rejected(F, "missing_left_hand_side");
    assert_rejected(F, "assign_to_literal");
}
