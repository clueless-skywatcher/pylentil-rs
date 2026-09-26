use super::*;
const F: &str = "parser/if_statements.py";

#[test]
fn parse_if_statements_simple() {
    assert_eq!(
        stmt(F, "simple"),
        if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![])
    );
}

#[test]
fn parse_if_statements_multi_statement_body() {
    assert_eq!(
        stmt(F, "multi_statement_body"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b")), expr_stmt(name("c"))],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_assignment_in_body() {
    assert_eq!(
        stmt(F, "assignment_in_body"),
        if_stmt(
            name("a"),
            vec![assign(vec![store(name("x"))], int(1))],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_two_assignments_in_body() {
    assert_eq!(
        stmt(F, "two_assignments_in_body"),
        if_stmt(
            name("a"),
            vec![
                assign(vec![store(name("x"))], int(1)),
                assign(vec![store(name("y"))], int(2))
            ],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_assignment_then_expression() {
    assert_eq!(
        stmt(F, "assignment_then_expression"),
        if_stmt(
            name("a"),
            vec![assign(vec![store(name("x"))], int(1)), expr_stmt(name("y"))],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_expression_then_assignment() {
    assert_eq!(
        stmt(F, "expression_then_assignment"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("y")), assign(vec![store(name("x"))], int(1))],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_with_else() {
    assert_eq!(
        stmt(F, "with_else"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![expr_stmt(name("c"))]
        )
    );
}

#[test]
fn parse_if_statements_else_with_assignment() {
    assert_eq!(
        stmt(F, "else_with_assignment"),
        if_stmt(
            name("a"),
            vec![assign(vec![store(name("x"))], int(1))],
            vec![assign(vec![store(name("x"))], int(2))]
        )
    );
}

#[test]
fn parse_if_statements_elif() {
    assert_eq!(
        stmt(F, "elif"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![if_stmt(name("c"), vec![expr_stmt(name("d"))], vec![])]
        )
    );
}

#[test]
fn parse_if_statements_elif_else() {
    assert_eq!(
        stmt(F, "elif_else"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![if_stmt(
                name("c"),
                vec![expr_stmt(name("d"))],
                vec![expr_stmt(name("e"))]
            )]
        )
    );
}

#[test]
fn parse_if_statements_two_elifs() {
    assert_eq!(
        stmt(F, "two_elifs"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![if_stmt(
                name("c"),
                vec![expr_stmt(name("d"))],
                vec![if_stmt(name("e"), vec![expr_stmt(name("f"))], vec![])]
            )]
        )
    );
}

#[test]
fn parse_if_statements_nested() {
    assert_eq!(
        stmt(F, "nested"),
        if_stmt(
            name("a"),
            vec![if_stmt(name("b"), vec![expr_stmt(name("c"))], vec![])],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_nested_with_else() {
    assert_eq!(
        stmt(F, "nested_with_else"),
        if_stmt(
            name("a"),
            vec![if_stmt(
                name("b"),
                vec![expr_stmt(name("c"))],
                vec![expr_stmt(name("d"))]
            )],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_nested_then_sibling_statement() {
    assert_eq!(
        stmt(F, "nested_then_sibling_statement"),
        if_stmt(
            name("a"),
            vec![
                if_stmt(name("b"), vec![expr_stmt(name("c"))], vec![]),
                expr_stmt(name("d"))
            ],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_comparison_condition() {
    assert_eq!(
        stmt(F, "comparison_condition"),
        if_stmt(
            compare(name("x"), vec![PyComparisonOp::Lt], vec![int(10)]),
            vec![assign(vec![store(name("y"))], int(1))],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_boolean_condition() {
    assert_eq!(
        stmt(F, "boolean_condition"),
        if_stmt(
            bool_op(PyBoolOp::And, vec![name("a"), name("b")]),
            vec![expr_stmt(name("c"))],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_tuple_condition() {
    assert_eq!(
        stmt(F, "tuple_condition"),
        if_stmt(
            tuple(vec![name("a"), name("b")]),
            vec![expr_stmt(name("c"))],
            vec![]
        )
    );
}

#[test]
fn parse_if_statements_parenthesized_condition() {
    assert_eq!(
        stmt(F, "parenthesized_condition"),
        if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![])
    );
}

#[test]
fn parse_if_statements_inline_body() {
    assert_eq!(
        stmt(F, "inline_body"),
        if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![])
    );
}

#[test]
fn parse_if_statements_inline_body_with_else() {
    assert_eq!(
        stmt(F, "inline_body_with_else"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![expr_stmt(name("c"))]
        )
    );
}

#[test]
fn parse_if_statements_statement_after_block() {
    assert_eq!(
        body(F, "statement_after_block"),
        vec![
            if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![]),
            expr_stmt(name("c"))
        ]
    );
}

#[test]
fn parse_if_statements_two_if_statements() {
    assert_eq!(
        body(F, "two_if_statements"),
        vec![
            if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![]),
            if_stmt(name("c"), vec![expr_stmt(name("d"))], vec![])
        ]
    );
}

#[test]
fn parse_if_statements_blank_line_before_else() {
    assert_eq!(
        stmt(F, "blank_line_before_else"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![expr_stmt(name("c"))]
        )
    );
}

#[test]
fn parse_if_statements_missing_colon() {
    assert_rejected(F, "missing_colon");
}

#[test]
fn parse_if_statements_missing_body() {
    assert_rejected(F, "missing_body");
}

#[test]
fn parse_if_statements_missing_condition() {
    assert_rejected(F, "missing_condition");
}

#[test]
fn parse_if_statements_else_without_if() {
    assert_rejected(F, "else_without_if");
}
