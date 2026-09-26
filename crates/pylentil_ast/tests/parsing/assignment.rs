use super::*;
const F: &str = "parser/assignment.py";

#[test]
fn parse_assignment_simple() {
    assert_eq!(stmt(F, "simple"), assign(vec![store(name("x"))], int(1)));
}

#[test]
fn parse_assignment_expression_value() {
    assert_eq!(
        stmt(F, "expression_value"),
        assign(
            vec![store(name("x"))],
            bin_op(int(1), PyBinaryOp::Add, int(2))
        )
    );
}

#[test]
fn parse_assignment_name_value() {
    assert_eq!(
        stmt(F, "name_value"),
        assign(vec![store(name("x"))], name("y"))
    );
}

#[test]
fn parse_assignment_tuple_unpack() {
    assert_eq!(
        stmt(F, "tuple_unpack"),
        assign(
            vec![store(tuple(vec![name("a"), name("b")]))],
            tuple(vec![int(1), int(2)])
        )
    );
}

#[test]
fn parse_assignment_parenthesized_target() {
    assert_eq!(
        stmt(F, "parenthesized_target"),
        assign(
            vec![store(parenthesized_tuple(vec![name("a"), name("b")]))],
            tuple(vec![int(1), int(2)])
        )
    );
}

#[test]
fn parse_assignment_nested_unpack() {
    assert_eq!(
        stmt(F, "nested_unpack"),
        assign(
            vec![store(tuple(vec![
                parenthesized_tuple(vec![name("a"), name("b")]),
                name("c")
            ]))],
            tuple(vec![parenthesized_tuple(vec![int(1), int(2)]), int(3)])
        )
    );
}

#[test]
fn parse_assignment_swap() {
    assert_eq!(
        stmt(F, "swap"),
        assign(
            vec![store(tuple(vec![name("a"), name("b")]))],
            tuple(vec![name("b"), name("a")])
        )
    );
}

#[test]
fn parse_assignment_chained() {
    assert_eq!(
        stmt(F, "chained"),
        assign(vec![store(name("a")), store(name("b"))], int(1))
    );
}

#[test]
fn parse_assignment_chained_three() {
    assert_eq!(
        stmt(F, "chained_three"),
        assign(
            vec![store(name("a")), store(name("b")), store(name("c"))],
            int(1)
        )
    );
}

#[test]
fn parse_assignment_augmented_add() {
    assert_eq!(
        stmt(F, "augmented_add"),
        aug_assign(store(name("x")), PyBinaryOp::Add, int(1))
    );
}

#[test]
fn parse_assignment_augmented_mul() {
    assert_eq!(
        stmt(F, "augmented_mul"),
        aug_assign(store(name("x")), PyBinaryOp::Mul, int(2))
    );
}

#[test]
fn parse_assignment_annotated() {
    assert_eq!(
        stmt(F, "annotated"),
        ann_assign(name("x"), name("int"), Some(int(1)))
    );
}

#[test]
fn parse_assignment_annotation_only() {
    assert_eq!(
        stmt(F, "annotation_only"),
        ann_assign(name("x"), name("int"), None)
    );
}

#[test]
fn parse_assignment_attribute_target() {
    assert_eq!(
        stmt(F, "attribute_target"),
        assign(vec![store(attribute(name("obj"), "field"))], int(1))
    );
}

#[test]
fn parse_assignment_subscript_target() {
    assert_eq!(
        stmt(F, "subscript_target"),
        assign(vec![store(subscript(name("items"), int(0)))], int(1))
    );
}

#[test]
fn parse_assignment_starred_target() {
    assert_eq!(
        stmt(F, "starred_target"),
        assign(
            vec![store(tuple(vec![name("a"), starred(name("rest"))]))],
            name("items")
        )
    );
}

#[test]
fn parse_assignment_walrus() {
    assert_eq!(expr(F, "walrus"), named_expr(name("n"), int(10)));
}

#[test]
fn parse_assignment_value_is_a_tuple() {
    assert_eq!(
        stmt(F, "value_is_a_tuple"),
        assign(vec![store(name("x"))], tuple(vec![int(1), int(2)]))
    );
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
    assert_eq!(
        expr(F, "equality_is_not_assignment"),
        compare(name("x"), vec![PyComparisonOp::Eq], vec![int(1)])
    );
}
