use super::*;
const F: &str = "parser/modules.py";

#[test]
fn parse_modules_single_statement() {
    assert_eq!(body(F, "single_statement"), vec![expr_stmt(name("a"))]);
}

#[test]
fn parse_modules_two_statements() {
    assert_eq!(
        body(F, "two_statements"),
        vec![expr_stmt(name("a")), expr_stmt(name("b"))]
    );
}

#[test]
fn parse_modules_three_statements() {
    assert_eq!(
        body(F, "three_statements"),
        vec![
            expr_stmt(name("a")),
            expr_stmt(name("b")),
            expr_stmt(name("c"))
        ]
    );
}

#[test]
fn parse_modules_statements_with_blank_lines() {
    assert_eq!(
        body(F, "statements_with_blank_lines"),
        vec![expr_stmt(name("a")), expr_stmt(name("b"))]
    );
}

#[test]
fn parse_modules_leading_blank_lines() {
    assert_eq!(body(F, "leading_blank_lines"), vec![expr_stmt(name("a"))]);
}

#[test]
fn parse_modules_assignment_then_expression() {
    assert_eq!(
        body(F, "assignment_then_expression"),
        vec![assign(vec![store(name("x"))], int(1)), expr_stmt(name("x"))]
    );
}

#[test]
fn parse_modules_expression_then_assignment() {
    assert_eq!(
        body(F, "expression_then_assignment"),
        vec![expr_stmt(name("x")), assign(vec![store(name("x"))], int(1))]
    );
}

#[test]
fn parse_modules_two_assignments() {
    assert_eq!(
        body(F, "two_assignments"),
        vec![
            assign(vec![store(name("x"))], int(1)),
            assign(vec![store(name("y"))], int(2))
        ]
    );
}

#[test]
fn parse_modules_statement_block_statement() {
    assert_eq!(
        body(F, "statement_block_statement"),
        vec![
            expr_stmt(name("a")),
            if_stmt(name("b"), vec![expr_stmt(name("c"))], vec![]),
            expr_stmt(name("d"))
        ]
    );
}

#[test]
fn parse_modules_block_then_block() {
    assert_eq!(
        body(F, "block_then_block"),
        vec![
            if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![]),
            if_stmt(name("c"), vec![expr_stmt(name("d"))], vec![])
        ]
    );
}

#[test]
fn parse_modules_block_last() {
    assert_eq!(
        body(F, "block_last"),
        vec![
            expr_stmt(name("a")),
            if_stmt(name("b"), vec![expr_stmt(name("c"))], vec![])
        ]
    );
}

#[test]
fn parse_modules_consecutive_blank_lines() {
    assert_eq!(
        body(F, "consecutive_blank_lines"),
        vec![expr_stmt(name("a")), expr_stmt(name("b"))]
    );
}

#[test]
fn parse_modules_many_statements() {
    assert_eq!(body(F, "many_statements").len(), 5);
}

// ------------------------------------------------ inline snippets --
// These check hand-written code, not fixture cases.

#[test]
fn a_module_without_a_trailing_newline() {
    assert_eq!(
        parse_body("a = 1"),
        vec![assign(vec![store(name("a"))], int(1))]
    );
}
