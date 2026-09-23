use super::*;
const F: &str = "parser/if_statements.py";

fn s(name: &str) -> String {
    let code = case(F, name);
    match parse_outcome(&code) {
        Outcome::Ok(body) => body.join(" "),
        Outcome::Err(err) => format!("<rejected: {err:?}>"),
        Outcome::Panic(m) => format!("<panic: {m}>"),
    }
}

#[test]
fn a_simple_block() {
    assert_eq!(s("simple"), "(if a (b) ())");
}

#[test]
fn an_else_branch() {
    assert_eq!(s("with_else"), "(if a (b) (c))");
}

#[test]
fn nesting() {
    assert_eq!(s("nested"), "(if a ((if b (c) ())) ())");
    assert_eq!(s("nested_with_else"), "(if a ((if b (c) (d))) ())");
}

#[test]
fn a_statement_after_the_block_is_a_sibling() {
    assert_eq!(s("statement_after_block"), "(if a (b) ()) c");
    assert_eq!(s("two_if_statements"), "(if a (b) ()) (if c (d) ())");
}

#[test]
fn conditions_may_be_expressions() {
    assert_eq!(s("comparison_condition"), "(if (compare x < 10) ((assign (y) 1)) ())");
    assert_eq!(s("parenthesized_condition"), "(if a (b) ())");
}

#[test]
fn a_body_may_hold_several_statements() {
    assert_eq!(s("multi_statement_body"), "(if a (b c) ())");
}

#[test]
fn a_body_may_hold_assignments() {
    assert_eq!(s("assignment_in_body"), "(if a ((assign (x) 1)) ())");
    assert_eq!(s("two_assignments_in_body"), "(if a ((assign (x) 1) (assign (y) 2)) ())");
    assert_eq!(s("assignment_then_expression"), "(if a ((assign (x) 1) y) ())");
    assert_eq!(s("expression_then_assignment"), "(if a (y (assign (x) 1)) ())");
    assert_eq!(s("else_with_assignment"), "(if a ((assign (x) 1)) ((assign (x) 2)))");
}

#[test]
fn a_nested_block_may_be_followed_by_a_sibling() {
    assert_eq!(s("nested_then_sibling_statement"), "(if a ((if b (c) ()) d) ())");
}

#[test]
fn a_boolean_condition() {
    assert_eq!(s("boolean_condition"), "(if (and a b) (c) ())");
}

#[test]
fn elif_is_an_else_holding_an_if() {
    assert_eq!(s("elif"), "(if a (b) ((if c (d) ())))");
    assert_eq!(s("elif_else"), "(if a (b) ((if c (d) (e))))");
    assert_eq!(s("two_elifs"), "(if a (b) ((if c (d) ((if e (f) ())))))");
}

#[test]
fn a_blank_line_before_else_is_allowed() {
    assert_eq!(s("blank_line_before_else"), "(if a (b) (c))");
}

#[test]
fn an_inline_body_is_allowed() {
    assert_eq!(s("inline_body"), "(if a (b) ())");
    assert_eq!(s("inline_body_with_else"), "(if a (b) (c))");
}

#[test]
fn a_tuple_condition_is_still_a_condition() {
    assert_eq!(s("tuple_condition"), "(if (tuple a b) (c) ())");
}

#[test]
fn malformed_if_statements_are_rejected() {
    for name in ["missing_colon", "missing_body", "missing_condition", "else_without_if"] {
        assert_rejected(F, name);
    }
}
