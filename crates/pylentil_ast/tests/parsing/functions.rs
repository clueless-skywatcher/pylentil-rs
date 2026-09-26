use super::*;
const F: &str = "parser/functions.py";

fn s(name: &str) -> String {
    let code = case(F, name);
    match parse_outcome(&code) {
        Outcome::Ok(body) => body.join(" "),
        Outcome::Err(err) => format!("<rejected: {err:?}>"),
        Outcome::Panic(m) => format!("<panic: {m}>"),
    }
}

fn many(name: &str) -> Vec<String> {
    stmts(&case(F, name)).unwrap_or_else(|e| vec![format!("<rejected: {e:?}>")])
}

#[test]
fn parse_functions_no_args() {
    assert_eq!(s("no_args"), "(def f () (pass))");
}

#[test]
fn parse_functions_one_arg() {
    assert_eq!(s("one_arg"), "(def f (a) (pass))");
}

#[test]
fn parse_functions_two_args() {
    assert_eq!(s("two_args"), "(def f (a b) (pass))");
}

#[test]
fn parse_functions_trailing_comma() {
    assert_eq!(s("trailing_comma"), "(def f (a b) (pass))");
}

#[test]
fn parse_functions_default_arg() {
    assert_eq!(s("default_arg"), "(def f (a=1) (pass))");
}

#[test]
fn parse_functions_defaults_mixed() {
    assert_eq!(s("defaults_mixed"), "(def f (a b=1 c=2) (pass))");
}

#[test]
fn parse_functions_default_expression() {
    assert_eq!(s("default_expression"), "(def f (a=(+ 1 2)) (pass))");
}

#[test]
fn parse_functions_annotated() {
    assert_eq!(s("annotated"), "(def f (a:int) (pass))");
}

#[test]
fn parse_functions_annotated_default() {
    assert_eq!(s("annotated_default"), "(def f (a:int=1) (pass))");
}

#[test]
fn parse_functions_annotation_expression() {
    assert_eq!(s("annotation_expression"), "(def f (a:(subscript list int)) (pass))");
}

#[test]
fn parse_functions_return_annotation() {
    assert_eq!(s("return_annotation"), "(def f () -> int (pass))");
}

#[test]
fn parse_functions_return_annotation_expression() {
    assert_eq!(s("return_annotation_expression"), "(def f () -> (subscript list int) (pass))");
}

#[test]
fn parse_functions_vararg() {
    assert_eq!(s("vararg"), "(def f (*args) (pass))");
}

#[test]
fn parse_functions_vararg_annotated() {
    assert_eq!(s("vararg_annotated"), "(def f (*args:int) (pass))");
}

#[test]
fn parse_functions_kwarg() {
    assert_eq!(s("kwarg"), "(def f (**kw) (pass))");
}

#[test]
fn parse_functions_vararg_and_kwarg() {
    assert_eq!(s("vararg_and_kwarg"), "(def f (a *args **kw) (pass))");
}

#[test]
fn parse_functions_kwonly() {
    assert_eq!(s("kwonly"), "(def f (a * b) (pass))");
}

#[test]
fn parse_functions_kwonly_default() {
    assert_eq!(s("kwonly_default"), "(def f (* b=1) (pass))");
}

#[test]
fn parse_functions_kwonly_after_vararg() {
    assert_eq!(s("kwonly_after_vararg"), "(def f (*args b c=2) (pass))");
}

#[test]
fn parse_functions_kwonly_then_kwarg() {
    assert_eq!(s("kwonly_then_kwarg"), "(def f (* b **kw) (pass))");
}

#[test]
fn parse_functions_posonly() {
    assert_eq!(s("posonly"), "(def f (a b / c) (pass))");
}

#[test]
fn parse_functions_posonly_only() {
    assert_eq!(s("posonly_only"), "(def f (a /) (pass))");
}

#[test]
fn parse_functions_posonly_with_defaults() {
    assert_eq!(s("posonly_with_defaults"), "(def f (a b=1 / c=2) (pass))");
}

#[test]
fn parse_functions_everything() {
    assert_eq!(
        s("everything"),
        "(def f (a / b c=1 *rest d e=2 **kw) -> int ((return a)))"
    );
}

#[test]
fn parse_functions_inline_body() {
    assert_parses(F, "inline_body");
}

#[test]
fn parse_functions_inline_body_two_statements() {
    assert_parses(F, "inline_body_two_statements");
}

#[test]
fn parse_functions_multi_statement_body() {
    assert_parses(F, "multi_statement_body");
}

#[test]
fn parse_functions_body_with_return() {
    // assert_eq!(s("inline_body"), "(def f () (pass))");
    // assert_eq!(s("inline_body_two_statements"), "(def f () (a b))");
    // assert_eq!(s("multi_statement_body"), "(def f () (a b))");
    assert_eq!(s("body_with_return"), "(def f (a) ((return a)))");
}

#[test]
fn parse_functions_docstring_body() {
    assert_parses(F, "docstring_body");
}

#[test]
fn parse_functions_nested_def() {
    assert_eq!(s("nested_def"), "(def f () ((def g () (pass))))");
}

#[test]
fn parse_functions_def_then_statement() {
    assert_eq!(many("def_then_statement"), vec!["(def f () (pass))", "x"]);
}

#[test]
fn parse_functions_def_in_if() {
    assert_eq!(s("def_in_if"), "(if a ((def f () (pass))) ())");
}

#[test]
fn parse_functions_decorated() {
    assert_eq!(s("decorated"), "(def @dec f () (pass))");
}

#[test]
fn parse_functions_two_decorators() {
    assert_eq!(s("two_decorators"), "(def @a @b f () (pass))");
}

#[test]
fn parse_functions_decorator_call() {
    assert_eq!(s("decorator_call"), "(def @(call (attr app route) str(/)) f () (pass))");
}

#[test]
fn parse_functions_async_def() {
    assert_eq!(s("async_def"), "(async-def f () (pass))");
}

#[test]
fn parse_functions_missing_name() {
    assert_rejected(F, "missing_name");
}

#[test]
fn parse_functions_missing_parens() {
    assert_rejected(F, "missing_parens");
}

#[test]
fn parse_functions_missing_colon() {
    assert_rejected(F, "missing_colon");
}

#[test]
fn parse_functions_missing_body() {
    assert_rejected(F, "missing_body");
}

#[test]
fn parse_functions_positional_after_default() {
    assert_rejected(F, "positional_after_default");
}

#[test]
fn parse_functions_duplicate_vararg() {
    assert_rejected(F, "duplicate_vararg");
}

#[test]
fn parse_functions_kwarg_not_last() {
    assert_rejected(F, "kwarg_not_last");
}

#[test]
fn parse_functions_two_posonly_markers() {
    assert_rejected(F, "two_posonly_markers");
}

#[test]
fn parse_functions_posonly_marker_first() {
    assert_rejected(F, "posonly_marker_first");
}

#[test]
fn parse_functions_bare_star_last() {
    assert_rejected(F, "bare_star_last");
}

#[test]
fn parse_functions_literal_parameter() {
    assert_rejected(F, "literal_parameter");
}

#[test]
fn parse_functions_duplicate_parameter() {
    assert_rejected(F, "duplicate_parameter");
}

#[test]
fn parse_functions_unclosed_parens() {
    assert_rejected(F, "unclosed_parens");
}
