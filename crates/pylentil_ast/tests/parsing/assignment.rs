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
fn parse_assignment_simple() {
    assert_eq!(s("simple"), "(assign (x) 1)");
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
fn parse_assignment_expression_value() {
    assert_eq!(s("expression_value"), "(assign (x) (+ 1 2))");
}

#[test]
fn parse_assignment_name_value() {
    assert_eq!(s("name_value"), "(assign (x) y)");
}

#[test]
fn parse_assignment_tuple_unpack() {
    assert_eq!(s("tuple_unpack"), "(assign ((tuple a b)) (tuple 1 2))");
}

#[test]
fn parse_assignment_parenthesized_target() {
    assert_eq!(s("parenthesized_target"), "(assign ((ptuple a b)) (tuple 1 2))");
}

#[test]
fn parse_assignment_nested_unpack() {
    assert_eq!(s("nested_unpack"), "(assign ((tuple (ptuple a b) c)) (tuple (ptuple 1 2) 3))");
}

#[test]
fn parse_assignment_swap() {
    assert_eq!(s("swap"), "(assign ((tuple a b)) (tuple b a))");
}

#[test]
fn parse_assignment_chained() {
    assert_eq!(s("chained"), "(assign (a b) 1)");
}

#[test]
fn parse_assignment_chained_three() {
    assert_eq!(s("chained_three"), "(assign (a b c) 1)");
}

#[test]
fn parse_assignment_augmented_add() {
    assert_eq!(s("augmented_add"), "(augassign x += 1)");
}

#[test]
fn parse_assignment_augmented_mul() {
    assert_eq!(s("augmented_mul"), "(augassign x *= 2)");
}

#[test]
fn parse_assignment_annotated() {
    assert_eq!(s("annotated"), "(annassign x int 1)");
}

#[test]
fn parse_assignment_annotation_only() {
    assert_eq!(s("annotation_only"), "(annassign x int -)");
}

#[test]
fn parse_assignment_attribute_target() {
    assert_eq!(s("attribute_target"), "(assign ((attr obj field)) 1)");
}

#[test]
fn parse_assignment_subscript_target() {
    assert_eq!(s("subscript_target"), "(assign ((subscript items 0)) 1)");
}

#[test]
fn parse_assignment_starred_target() {
    assert_eq!(s("starred_target"), "(assign ((tuple a (star rest))) items)");
}

#[test]
fn parse_assignment_walrus() {
    assert_eq!(s("walrus"), "(:= n 10)");
}

#[test]
fn parse_assignment_value_is_a_tuple() {
    assert_eq!(s("value_is_a_tuple"), "(assign (x) (tuple 1 2))");
}

#[test]
fn parse_assignment_assignment_split_across_lines() {
    // `x` and `= 1` are two lines: Python reports a syntax error rather
    // than joining them into one assignment.
    assert_rejected(F, "assignment_split_across_lines");
}

#[test]
fn parse_assignment_missing_right_hand_side() {
    assert_no_panic(F, "missing_right_hand_side");
}

#[test]
fn parse_assignment_missing_left_hand_side() {
    assert_no_panic(F, "missing_left_hand_side");
}

#[test]
fn parse_assignment_assign_to_literal() {
    // assert_rejected(F, "missing_right_hand_side");
    // assert_rejected(F, "missing_left_hand_side");
    assert_rejected(F, "assign_to_literal");
}

#[test]
fn parse_assignment_equality_is_not_assignment() {
    assert_eq!(s("equality_is_not_assignment"), "(compare x == 1)");
}
