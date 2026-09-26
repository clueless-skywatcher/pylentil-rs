use super::*;
const F: &str = "parser/statements.py";

#[test]
fn parse_statements_pass() {
    assert_eq!(one_stmt(&case(F, "pass")), Ok("pass".into()));
}

#[test]
fn parse_statements_break() {
    assert_eq!(one_stmt(&case(F, "break")), Ok("break".into()));
}

#[test]
fn parse_statements_continue() {
    assert_eq!(one_stmt(&case(F, "continue")), Ok("continue".into()));
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
    assert_eq!(one_stmt(&case(F, "while_loop")), Ok("(while a (b) ())".into()));
}

#[test]
fn parse_statements_while_else() {
    assert_parses(F, "while_else");
}

#[test]
fn parse_statements_for_loop() {
    assert_eq!(one_stmt(&case(F, "for_loop")), Ok("(for i items ((call print i)))".into()));
}

#[test]
fn parse_statements_for_unpacking() {
    assert_parses(F, "for_unpacking");
}

#[test]
fn parse_statements_function_definition() {
    assert_eq!(one_stmt(&case(F, "function_definition")), Ok("(def f (a b) ((return a)))".into()));
}

#[test]
fn parse_statements_function_no_arguments() {
    assert_eq!(one_stmt(&case(F, "function_no_arguments")), Ok("(def f () (pass))".into()));
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
    assert_eq!(one_stmt(&case(F, "class_definition")), Ok("(class C (pass))".into()));
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
        stmts(&case(F, "semicolon_separated")),
        Ok(vec!["(assign (a) 1)".into(), "(assign (b) 2)".into()])
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
