use super::*;
const F: &str = "parser/statements.py";

#[test]
fn parse_statements_pass() {
    p_assert_eq!(stmt(F, "pass"), pass());
}

#[test]
fn parse_statements_break() {
    p_assert_eq!(stmt(F, "break"), break_());
}

#[test]
fn parse_statements_continue() {
    p_assert_eq!(stmt(F, "continue"), continue_());
}

#[test]
fn parse_statements_return_value() {
    assert_parses(F, "return_value");
}

#[test]
fn parse_statements_return_bare() {
    assert_parses(F, "return_bare");
}

#[test]
fn parse_statements_while_loop() {
    p_assert_eq!(
        stmt(F, "while_loop"),
        while_stmt(name("a"), vec![expr_stmt(name("b"))], vec![])
    );
}

#[test]
fn parse_statements_while_else() {
    p_assert_eq!(
        stmt(F, "while_else"),
        while_stmt(
            name("a"),
            vec![expr_stmt(name("b"))],
            vec![expr_stmt(name("c"))]
        )
    );
}

#[test]
fn parse_statements_for_loop() {
    p_assert_eq!(
        stmt(F, "for_loop"),
        for_stmt(
            store(name("i")),
            name("items"),
            vec![expr_stmt(call(name("f"), vec![name("i")], vec![]))]
        )
    );
}

#[test]
fn parse_statements_for_unpacking() {
    p_assert_eq!(
        stmt(F, "for_unpacking"),
        for_stmt(
            store(tuple(vec![name("k"), name("v")])),
            name("pairs"),
            vec![expr_stmt(call(name("f"), vec![name("k")], vec![]))]
        )
    );
}

#[test]
fn parse_statements_function_definition() {
    p_assert_eq!(
        stmt(F, "function_definition"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a"), arg("b")],
                ..Default::default()
            },
            vec![return_stmt(Some(name("a")))]
        )
    );
}

#[test]
fn parse_statements_function_no_arguments() {
    p_assert_eq!(
        stmt(F, "function_no_arguments"),
        function_def("f", PyArguments::default(), vec![pass()])
    );
}

#[test]
fn parse_statements_function_default_argument() {
    assert_parses(F, "function_default_argument");
}

#[test]
fn parse_statements_function_annotated() {
    assert_parses(F, "function_annotated");
}

#[test]
fn parse_statements_class_definition() {
    p_assert_eq!(stmt(F, "class_definition"), class_def("C", vec![pass()]));
}

#[test]
fn parse_statements_class_with_base() {
    assert_parses(F, "class_with_base");
}

#[test]
fn parse_statements_import() {
    assert_parses(F, "import");
}

#[test]
fn parse_statements_import_dotted() {
    assert_parses(F, "import_dotted");
}

#[test]
fn parse_statements_import_as() {
    assert_parses(F, "import_as");
}

#[test]
fn parse_statements_from_import() {
    assert_parses(F, "from_import");
}

#[test]
fn parse_statements_from_import_star() {
    assert_parses(F, "from_import_star");
}

#[test]
fn parse_statements_relative_import() {
    assert_parses(F, "relative_import");
}

#[test]
fn parse_statements_del() {
    p_assert_eq!(stmt(F, "del"), delete(vec![store(name("a"))]));
}

#[test]
fn parse_statements_global() {
    p_assert_eq!(stmt(F, "global"), global(&["x"]));
}

#[test]
fn parse_statements_nonlocal() {
    p_assert_eq!(stmt(F, "nonlocal"), nonlocal(&["x"]));
}

#[test]
fn parse_statements_assert() {
    p_assert_eq!(stmt(F, "assert"), assert_stmt(name("a"), None));
}

#[test]
fn parse_statements_assert_with_message() {
    p_assert_eq!(
        stmt(F, "assert_with_message"),
        assert_stmt(name("a"), Some(string("boom")))
    );
}

#[test]
fn parse_statements_raise() {
    assert_parses(F, "raise");
}

#[test]
fn parse_statements_raise_from() {
    assert_parses(F, "raise_from");
}

#[test]
fn parse_statements_bare_raise() {
    assert_parses(F, "bare_raise");
}

#[test]
fn parse_statements_try_except() {
    assert_parses(F, "try_except");
}

#[test]
fn parse_statements_try_multi_except() {
    assert_parses(F, "try_multi_except");
}

#[test]
fn parse_statements_try_except_as() {
    assert_parses(F, "try_except_as");
}

#[test]
fn parse_statements_try_finally() {
    assert_parses(F, "try_finally");
}

#[test]
fn parse_statements_try_except_else_finally() {
    assert_parses(F, "try_except_else_finally");
}

#[test]
fn parse_statements_with_statement() {
    p_assert_eq!(
        stmt(F, "with_statement"),
        with_stmt(
            vec![with_item(
                call(name("open"), vec![name("f")], vec![]),
                Some(store(name("fh")))
            )],
            vec![pass()]
        )
    );
}

#[test]
fn parse_statements_with_multiple_items() {
    p_assert_eq!(
        stmt(F, "with_multiple_items"),
        with_stmt(
            vec![
                with_item(name("a"), Some(store(name("x")))),
                with_item(name("b"), Some(store(name("y"))))
            ],
            vec![pass()]
        )
    );
}

#[test]
fn parse_statements_lambda() {
    assert_parses(F, "lambda");
}

#[test]
fn parse_statements_lambda_no_args() {
    assert_parses(F, "lambda_no_args");
}

#[test]
fn parse_statements_decorated_function() {
    assert_parses(F, "decorated_function");
}

#[test]
fn parse_statements_async_function() {
    assert_parses(F, "async_function");
}

#[test]
fn parse_statements_async_for() {
    assert_parses(F, "async_for");
}

#[test]
fn parse_statements_yield_value() {
    p_assert_eq!(
        stmt(F, "yield_value"),
        function_def(
            "f",
            PyArguments::default(),
            vec![expr_stmt(yield_expr(Some(int(1))))]
        )
    );
}

#[test]
fn parse_statements_yield_expression() {
    p_assert_eq!(
        stmt(F, "yield_expression"),
        function_def(
            "f",
            PyArguments::default(),
            vec![expr_stmt(yield_expr(Some(bin_op(
                name("a"),
                PyBinaryOp::Add,
                int(1)
            ))))]
        )
    );
}

#[test]
fn parse_statements_yield_tuple() {
    p_assert_eq!(
        stmt(F, "yield_tuple"),
        function_def(
            "f",
            PyArguments::default(),
            vec![expr_stmt(yield_expr(Some(tuple(vec![int(1), int(2)]))))]
        )
    );
}

#[test]
fn parse_statements_yield_assigned() {
    p_assert_eq!(
        stmt(F, "yield_assigned"),
        function_def(
            "f",
            PyArguments::default(),
            vec![assign(vec![store(name("x"))], yield_expr(Some(int(1))))]
        )
    );
}

#[test]
fn parse_statements_yield_parenthesized() {
    p_assert_eq!(
        stmt(F, "yield_parenthesized"),
        function_def(
            "f",
            PyArguments::default(),
            vec![assign(vec![store(name("x"))], yield_expr(Some(int(1))))]
        )
    );
}

#[test]
fn parse_statements_yield_from() {
    assert_parses(F, "yield_from");
}

#[test]
fn parse_statements_match_statement() {
    assert_parses(F, "match_statement");
}

#[test]
fn parse_statements_semicolon_separated() {
    p_assert_eq!(
        body(F, "semicolon_separated"),
        vec![
            assign(vec![store(name("a"))], int(1)),
            assign(vec![store(name("b"))], int(2))
        ]
    );
}

#[test]
fn parse_statements_semicolon_trailing() {
    assert_parses(F, "semicolon_trailing");
}

#[test]
fn parse_statements_type_alias() {
    assert_parses(F, "type_alias");
}
