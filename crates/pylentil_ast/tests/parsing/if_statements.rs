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
fn parse_if_statements_simple() {
    assert_eq!(s("simple"), "(if a (b) ())");
}

#[test]
fn parse_if_statements_multi_statement_body() {
    assert_eq!(s("multi_statement_body"), "(if a (b c) ())");
}

#[test]
fn parse_if_statements_assignment_in_body() {
    assert_eq!(s("assignment_in_body"), "(if a ((assign (x) 1)) ())");
}

#[test]
fn parse_if_statements_two_assignments_in_body() {
    assert_eq!(s("two_assignments_in_body"), "(if a ((assign (x) 1) (assign (y) 2)) ())");
}

#[test]
fn parse_if_statements_assignment_then_expression() {
    assert_eq!(s("assignment_then_expression"), "(if a ((assign (x) 1) y) ())");
}

#[test]
fn parse_if_statements_expression_then_assignment() {
    assert_eq!(s("expression_then_assignment"), "(if a (y (assign (x) 1)) ())");
}

#[test]
fn parse_if_statements_with_else() {
    assert_eq!(s("with_else"), "(if a (b) (c))");
}

#[test]
fn parse_if_statements_else_with_assignment() {
    assert_eq!(s("else_with_assignment"), "(if a ((assign (x) 1)) ((assign (x) 2)))");
}

#[test]
fn parse_if_statements_elif() {
    assert_eq!(s("elif"), "(if a (b) ((if c (d) ())))");
}

#[test]
fn parse_if_statements_elif_else() {
    assert_eq!(s("elif_else"), "(if a (b) ((if c (d) (e))))");
}

#[test]
fn parse_if_statements_two_elifs() {
    assert_eq!(s("two_elifs"), "(if a (b) ((if c (d) ((if e (f) ())))))");
}

#[test]
fn parse_if_statements_nested() {
    assert_eq!(s("nested"), "(if a ((if b (c) ())) ())");
}

#[test]
fn parse_if_statements_nested_with_else() {
    assert_eq!(s("nested_with_else"), "(if a ((if b (c) (d))) ())");
}

#[test]
fn parse_if_statements_nested_then_sibling_statement() {
    assert_eq!(s("nested_then_sibling_statement"), "(if a ((if b (c) ()) d) ())");
}

#[test]
fn parse_if_statements_comparison_condition() {
    assert_eq!(s("comparison_condition"), "(if (compare x < 10) ((assign (y) 1)) ())");
}

#[test]
fn parse_if_statements_boolean_condition() {
    assert_eq!(s("boolean_condition"), "(if (and a b) (c) ())");
}

#[test]
fn parse_if_statements_tuple_condition() {
    assert_eq!(s("tuple_condition"), "(if (tuple a b) (c) ())");
}

#[test]
fn parse_if_statements_parenthesized_condition() {
    assert_eq!(s("parenthesized_condition"), "(if a (b) ())");
}

#[test]
fn parse_if_statements_inline_body() {
    assert_eq!(s("inline_body"), "(if a (b) ())");
}

#[test]
fn parse_if_statements_inline_body_with_else() {
    assert_eq!(s("inline_body_with_else"), "(if a (b) (c))");
}

#[test]
fn parse_if_statements_statement_after_block() {
    assert_eq!(s("statement_after_block"), "(if a (b) ()) c");
}

#[test]
fn parse_if_statements_two_if_statements() {
    assert_eq!(s("two_if_statements"), "(if a (b) ()) (if c (d) ())");
}

#[test]
fn parse_if_statements_blank_line_before_else() {
    assert_eq!(s("blank_line_before_else"), "(if a (b) (c))");
}

#[test]
fn parse_if_statements_missing_colon() {
    assert_rejected(F, "missing_colon");
}

#[test]
fn parse_if_statements_missing_body() {
    assert_rejected(F, "missing_body");
}

#[test]
fn parse_if_statements_missing_condition() {
    assert_rejected(F, "missing_condition");
}

#[test]
fn parse_if_statements_else_without_if() {
    assert_rejected(F, "else_without_if");
}
