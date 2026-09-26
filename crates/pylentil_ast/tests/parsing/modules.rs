use super::*;
const F: &str = "parser/modules.py";

fn s(name: &str) -> Vec<String> {
    stmts(&case(F, name)).unwrap_or_else(|e| vec![format!("<rejected: {e:?}>")])
}

#[test]
fn parse_modules_single_statement() {
    assert_eq!(s("single_statement"), vec!["a"]);
}

#[test]
fn parse_modules_two_statements() {
    assert_eq!(s("two_statements"), vec!["a", "b"]);
}

#[test]
fn parse_modules_three_statements() {
    assert_eq!(s("three_statements"), vec!["a", "b", "c"]);
}

#[test]
fn parse_modules_statements_with_blank_lines() {
    assert_eq!(s("statements_with_blank_lines"), vec!["a", "b"]);
}

#[test]
fn parse_modules_leading_blank_lines() {
    assert_eq!(s("leading_blank_lines"), vec!["a"]);
}

#[test]
fn parse_modules_assignment_then_expression() {
    assert_eq!(s("assignment_then_expression"), vec!["(assign (x) 1)", "x"]);
}

#[test]
fn parse_modules_expression_then_assignment() {
    assert_eq!(s("expression_then_assignment"), vec!["x", "(assign (x) 1)"]);
}

#[test]
fn parse_modules_two_assignments() {
    assert_eq!(s("two_assignments"), vec!["(assign (x) 1)", "(assign (y) 2)"]);
}

#[test]
fn parse_modules_statement_block_statement() {
    assert_eq!(s("statement_block_statement"), vec!["a", "(if b (c) ())", "d"]);
}

#[test]
fn parse_modules_block_then_block() {
    assert_eq!(s("block_then_block"), vec!["(if a (b) ())", "(if c (d) ())"]);
}

#[test]
fn parse_modules_block_last() {
    assert_eq!(s("block_last"), vec!["a", "(if b (c) ())"]);
}

#[test]
fn parse_modules_consecutive_blank_lines() {
    assert_eq!(s("consecutive_blank_lines"), vec!["a", "b"]);
}

#[test]
fn parse_modules_many_statements() {
    assert_eq!(s("many_statements").len(), 5);
}

// ------------------------------------------------ inline snippets --
// These check hand-written code, not fixture cases.

#[test]
fn a_module_without_a_trailing_newline() {
    assert_eq!(stmts("a = 1"), Ok(vec!["(assign (a) 1)".into()]));
}
