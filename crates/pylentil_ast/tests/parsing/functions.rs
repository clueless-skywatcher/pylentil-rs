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
fn a_function_without_parameters() {
    assert_eq!(s("no_args"), "(def f () (pass))");
}

#[test]
fn positional_parameters() {
    assert_eq!(s("one_arg"), "(def f (a) (pass))");
    assert_eq!(s("two_args"), "(def f (a b) (pass))");
    assert_eq!(s("trailing_comma"), "(def f (a b) (pass))");
}

#[test]
fn defaults_attach_to_the_rightmost_parameters() {
    assert_eq!(s("default_arg"), "(def f (a=1) (pass))");
    assert_eq!(s("defaults_mixed"), "(def f (a b=1 c=2) (pass))");
    assert_eq!(s("default_expression"), "(def f (a=(+ 1 2)) (pass))");
}

#[test]
fn parameter_annotations() {
    assert_eq!(s("annotated"), "(def f (a:int) (pass))");
    assert_eq!(s("annotated_default"), "(def f (a:int=1) (pass))");
    assert_eq!(s("annotation_expression"), "(def f (a:(subscript list int)) (pass))");
}

#[test]
fn return_annotations() {
    assert_eq!(s("return_annotation"), "(def f () -> int (pass))");
    assert_eq!(s("return_annotation_expression"), "(def f () -> (subscript list int) (pass))");
}

#[test]
fn star_parameters() {
    assert_eq!(s("vararg"), "(def f (*args) (pass))");
    assert_eq!(s("vararg_annotated"), "(def f (*args:int) (pass))");
    assert_eq!(s("kwarg"), "(def f (**kw) (pass))");
    assert_eq!(s("vararg_and_kwarg"), "(def f (a *args **kw) (pass))");
}

#[test]
fn keyword_only_parameters() {
    assert_eq!(s("kwonly"), "(def f (a * b) (pass))");
    assert_eq!(s("kwonly_default"), "(def f (* b=1) (pass))");
    assert_eq!(s("kwonly_after_vararg"), "(def f (*args b c=2) (pass))");
    assert_eq!(s("kwonly_then_kwarg"), "(def f (* b **kw) (pass))");
}

#[test]
fn positional_only_parameters() {
    assert_eq!(s("posonly"), "(def f (a b / c) (pass))");
    assert_eq!(s("posonly_only"), "(def f (a /) (pass))");
    assert_eq!(s("posonly_with_defaults"), "(def f (a b=1 / c=2) (pass))");
}

#[test]
fn every_parameter_kind_at_once() {
    assert_eq!(
        s("everything"),
        "(def f (a / b c=1 *rest d e=2 **kw) -> int ((return a)))"
    );
}

#[test]
fn bodies() {
    // assert_eq!(s("inline_body"), "(def f () (pass))");
    // assert_eq!(s("inline_body_two_statements"), "(def f () (a b))");
    // assert_eq!(s("multi_statement_body"), "(def f () (a b))");
    assert_eq!(s("body_with_return"), "(def f (a) ((return a)))");
    // assert_eq!(s("docstring_body"), "(def f () (str(doc)))");
}

#[test]
fn a_def_is_an_ordinary_compound_statement() {
    assert_eq!(s("nested_def"), "(def f () ((def g () (pass))))");
    assert_eq!(many("def_then_statement"), vec!["(def f () (pass))", "x"]);
    assert_eq!(s("def_in_if"), "(if a ((def f () (pass))) ())");
}

#[test]
fn decorators_are_kept_in_source_order() {
    assert_eq!(s("decorated"), "(def @dec f () (pass))");
    assert_eq!(s("two_decorators"), "(def @a @b f () (pass))");
    assert_eq!(s("decorator_call"), "(def @(call (attr app route) str(/)) f () (pass))");
}

#[test]
fn an_async_def() {
    assert_eq!(s("async_def"), "(async-def f () (pass))");
}

const MALFORMED: &[&str] = &[
    "missing_name",
    "missing_parens",
    "missing_colon",
    "missing_body",
    "positional_after_default",
    "duplicate_vararg",
    "kwarg_not_last",
    "two_posonly_markers",
    "posonly_marker_first",
    "bare_star_last",
    "literal_parameter",
    "duplicate_parameter",
    "unclosed_parens",
];

#[test]
fn malformed_definitions_are_rejected() {
    for name in MALFORMED {
        assert_rejected(F, name);
    }
}

#[test]
fn every_well_formed_definition_parses() {
    let real: Vec<String> = parse_failures(F)
        .into_iter()
        .filter(|f| {
            let name = f.trim_start().split(':').next().unwrap_or("");
            !MALFORMED.contains(&name)
        })
        .collect();
    assert!(real.is_empty(), "function definitions must parse:\n{}", real.join("\n"));
}
