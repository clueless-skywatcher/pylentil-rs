use super::*;

#[test]
fn integer_literal() {
    p_assert_eq!(parse_expr("42\n"), int(42));
}

#[test]
fn float_literal() {
    p_assert_eq!(parse_expr("3.5\n"), float("3.5"));
}

#[test]
fn string_literal() {
    p_assert_eq!(parse_expr("\"text\"\n"), string("text"));
}

#[test]
fn booleans_and_none() {
    p_assert_eq!(parse_expr("True\n"), boolean(true));
    p_assert_eq!(parse_expr("False\n"), boolean(false));
    p_assert_eq!(parse_expr("None\n"), none());
}

#[test]
fn a_name() {
    p_assert_eq!(parse_expr("value\n"), name("value"));
}

#[test]
fn addition() {
    p_assert_eq!(
        parse_expr("1 + 2\n"),
        bin_op(int(1), PyBinaryOp::Add, int(2))
    );
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    p_assert_eq!(
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
    p_assert_eq!(parse_expr("(42)\n"), int(42));
    p_assert_eq!(parse_expr("((42))\n"), int(42));
}

#[test]
fn a_comparison() {
    p_assert_eq!(
        parse_expr("a == b\n"),
        compare(name("a"), vec![PyComparisonOp::Eq], vec![name("b")])
    );
}

#[test]
fn a_parenthesized_tuple() {
    p_assert_eq!(
        parse_expr("(a, b)\n"),
        parenthesized_tuple(vec![name("a"), name("b")])
    );
}

#[test]
fn simple_assignment() {
    p_assert_eq!(
        parse_stmt("x = 1\n"),
        assign(vec![store(name("x"))], int(1))
    );
}

#[test]
fn an_if_statement() {
    p_assert_eq!(
        parse_stmt("if a:\n    b\n"),
        if_stmt(name("a"), vec![expr_stmt(name("b"))], vec![])
    );
}

#[test]
fn an_if_else_statement() {
    p_assert_eq!(
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
    p_assert_eq!(
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
    p_assert_eq!(parse_body(""), vec![]);
    p_assert_eq!(parse_body("\n\n\n"), vec![]);
}
