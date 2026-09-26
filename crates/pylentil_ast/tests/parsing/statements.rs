use super::*;
const F: &str = "parser/statements.py";

#[test]
fn parse_statements_pass() {
    assert_eq!(stmt(F, "pass"), PyStatement::Pass);
}

#[test]
fn parse_statements_break() {
    assert_eq!(stmt(F, "break"), PyStatement::Break);
}

#[test]
fn parse_statements_continue() {
    assert_eq!(stmt(F, "continue"), PyStatement::Continue);
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
    assert_eq!(
        stmt(F, "while_loop"),
        while_stmt(name("a"), vec![expr_stmt(name("b"))], vec![])
    );
}

#[test]
fn parse_statements_while_else() {
    assert_parses(F, "while_else");
}

#[test]
fn parse_statements_for_loop() {
    assert_eq!(
        stmt(F, "for_loop"),
        for_stmt(
            store(name("i")),
            name("items"),
            vec![expr_stmt(call(name("print"), vec![name("i")], vec![]))]
        )
    );
}

#[test]
fn parse_statements_for_unpacking() {
    assert_parses(F, "for_unpacking");
}

#[test]
fn parse_statements_function_definition() {
    assert_eq!(
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
    assert_eq!(
        stmt(F, "function_no_arguments"),
        function_def("f", PyArguments::default(), vec![PyStatement::Pass])
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
    assert_eq!(
        stmt(F, "class_definition"),
        class_def("C", vec![PyStatement::Pass])
    );
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
    assert_parses(F, "del");
}

#[test]
fn parse_statements_global() {
    assert_parses(F, "global");
}

#[test]
fn parse_statements_nonlocal() {
    assert_parses(F, "nonlocal");
}

#[test]
fn parse_statements_assert() {
    assert_parses(F, "assert");
}

#[test]
fn parse_statements_assert_with_message() {
    assert_parses(F, "assert_with_message");
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
    assert_parses(F, "with_statement");
}

#[test]
fn parse_statements_with_multiple_items() {
    assert_parses(F, "with_multiple_items");
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
    assert_parses(F, "yield_value");
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
    assert_eq!(
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
