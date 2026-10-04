//! Node positions. `==` on AST nodes ignores spans, so these compare offsets.

use super::*;
use pylentil_ast::ast::PyExceptHandler;

fn at(span: PySpan) -> (usize, Option<usize>) {
    (span.start, span.end)
}

#[test]
fn parse_spans_assign_and_binop() {
    let statement = parse_stmt("x = 1 + 22\n");
    p_assert_eq!(at(statement.span()), (0, Some(10)));

    let PyStatement::Assign { value, .. } = statement else {
        panic!("expected an assignment, got {statement:#?}");
    };
    p_assert_eq!(at(value.span()), (4, Some(10)));
}

#[test]
fn parse_spans_call() {
    p_assert_eq!(at(parse_expr("f(a, k=1)\n").span()), (0, Some(9)));
}

#[test]
fn parse_spans_parenthesized_tuple_includes_parens() {
    p_assert_eq!(at(parse_expr("(1, 2)\n").span()), (0, Some(6)));
}

#[test]
fn parse_spans_walrus_excludes_parens() {
    p_assert_eq!(at(parse_expr("(y := 3)\n").span()), (1, Some(7)));
}

#[test]
fn parse_spans_function_ends_at_last_statement() {
    // `return` is at 22..28; `y` starts at 30.
    let body = parse_body("def f():\n    pass\n    return\n\ny = 1\n");

    p_assert_eq!(at(body[0].span()), (0, Some(28)));
    p_assert_eq!(body[1].span().start, 30);
}

#[test]
fn parse_spans_elif_starts_at_its_keyword() {
    // `elif` is at 12 and `d` at 24..25.
    let statement = parse_stmt("if a:\n    b\nelif c:\n    d\n");
    p_assert_eq!(at(statement.span()), (0, Some(25)));

    let PyStatement::If { orelse, .. } = statement else {
        panic!("expected an if statement, got {statement:#?}");
    };
    p_assert_eq!(at(orelse[0].span()), (12, Some(25)));
}

#[test]
fn parse_spans_except_type() {
    // `ValueError` is at 18..28 and `y` at 39..40.
    let statement = parse_stmt("try:\n    x\nexcept ValueError as e:\n    y\n");
    p_assert_eq!(at(statement.span()), (0, Some(40)));

    let PyStatement::Try { handlers, .. } = statement else {
        panic!("expected a try statement, got {statement:#?}");
    };
    let PyExceptHandler { type_, span, .. } = &handlers[0];
    p_assert_eq!(at(*span), (11, Some(40)));
    p_assert_eq!(at(type_.as_ref().unwrap().span()), (18, Some(28)));
}

#[test]
fn parse_spans_call_keyword() {
    let PyExpr::Call { keywords, .. } = parse_expr("f(a, k=1)\n") else {
        panic!("expected a call");
    };
    p_assert_eq!(at(keywords[0].span), (5, Some(8)));
}

#[test]
fn parse_spans_import_alias() {
    let PyStatement::Import { names, .. } = parse_stmt("import os.path as p\n") else {
        panic!("expected an import");
    };
    p_assert_eq!(at(names[0].span), (7, Some(19)));

    let PyStatement::ImportFrom { names, .. } = parse_stmt("from m import *\n") else {
        panic!("expected a from-import");
    };
    p_assert_eq!(at(names[0].span), (14, Some(15)));
}

#[test]
fn parse_spans_parameters_exclude_defaults() {
    // `a: int` is at 6..12, `*rest` at 18..23 and `**kw` at 25..29.
    let PyStatement::FunctionDef { args, .. } =
        parse_stmt("def f(a: int = 1, *rest, **kw):\n    pass\n")
    else {
        panic!("expected a function");
    };
    p_assert_eq!(at(args.args[0].span), (6, Some(12)));
    p_assert_eq!(at(args.vararg.as_ref().unwrap().span), (18, Some(23)));
    p_assert_eq!(at(args.kwarg.as_ref().unwrap().span), (25, Some(29)));
}

#[test]
fn parse_spans_comprehension_clause() {
    let PyExpr::ListComp { generators, .. } = parse_expr("[x for x in y if x]\n") else {
        panic!("expected a list comprehension");
    };
    p_assert_eq!(at(generators[0].span), (3, Some(18)));
}
