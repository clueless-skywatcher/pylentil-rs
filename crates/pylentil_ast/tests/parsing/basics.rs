use super::*;

#[test]
fn integer_literal() {
    assert_eq!(parse_expr("42\n"), int(42));
}

#[test]
fn float_literal() {
    assert_eq!(parse_expr("3.5\n"), float("3.5"));
}

#[test]
fn string_literal() {
    assert_eq!(parse_expr("\"text\"\n"), string("text"));
}

#[test]
fn booleans_and_none() {
    assert_eq!(parse_expr("True\n"), boolean(true));
    assert_eq!(parse_expr("False\n"), boolean(false));
    assert_eq!(parse_expr("None\n"), none());
}

#[test]
fn a_name() {
    assert_eq!(parse_expr("value\n"), name("value"));
}

#[test]
fn addition() {
    assert_eq!(
        parse_expr("1 + 2\n"),
        bin_op(int(1), PyBinaryOp::Add, int(2))
    );
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    assert_eq!(
        parse_expr("1 + 2 * 3\n"),
        bin_op(
            int(1),
            PyBinaryOp::Add,
            bin_op(int(2), PyBinaryOp::Mul, int(3))
        )
    );
}

#[test]
fn parentheses_are_transparent() {
    assert_eq!(parse_expr("(42)\n"), int(42));
    assert_eq!(parse_expr("((42))\n"), int(42));
}

#[test]
fn a_comparison() {
    assert_eq!(
        parse_expr("a == b\n"),
        compare(name("a"), vec![PyComparisonOp::Eq], vec![name("b")])
    );
}

#[test]
fn a_parenthesized_tuple() {
    assert_eq!(
        parse_expr("(a, b)\n"),
        parenthesized_tuple(vec![name("a"), name("b")])
    );
}

#[test]
fn simple_assignment() {
    assert_eq!(
        parse_stmt("x = 1\n"),
        assign(vec![store(name("x"))], int(1))
    );
}

#[test]
fn an_if_statement() {
    assert_eq!(
        parse_stmt("if a:\n    b\n"),
        if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![])
    );
}

#[test]
fn an_if_else_statement() {
    assert_eq!(
        parse_stmt("if a:\n    b\nelse:\n    c\n"),
        if_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![expr_stmt(name("c"))]
        )
    );
}

#[test]
fn a_module_holds_several_statements() {
    assert_eq!(
        parse_body("a\nb\nc\n"),
        vec![
            expr_stmt(name("a")),
            expr_stmt(name("b")),
            expr_stmt(name("c"))
        ]
    );
}

#[test]
fn an_empty_module_has_no_statements() {
    assert_eq!(parse_body(""), vec![]);
    assert_eq!(parse_body("\n\n\n"), vec![]);
}
