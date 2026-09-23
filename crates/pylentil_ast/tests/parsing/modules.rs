use super::*;
const F: &str = "parser/modules.py";

fn s(name: &str) -> Vec<String> {
    stmts(&case(F, name)).unwrap_or_else(|e| vec![format!("<rejected: {e:?}>")])
}

#[test]
fn consecutive_statements() {
    assert_eq!(s("single_statement"), vec!["a"]);
    assert_eq!(s("two_statements"), vec!["a", "b"]);
    assert_eq!(s("three_statements"), vec!["a", "b", "c"]);
}

#[test]
fn blank_lines_are_not_statements() {
    assert_eq!(s("statements_with_blank_lines"), vec!["a", "b"]);
    assert_eq!(s("consecutive_blank_lines"), vec!["a", "b"]);
}

#[test]
fn assignments_and_expressions_mix() {
    assert_eq!(s("two_assignments"), vec!["(assign (x) 1)", "(assign (y) 2)"]);
    assert_eq!(s("assignment_then_expression"), vec!["(assign (x) 1)", "x"]);
    assert_eq!(s("expression_then_assignment"), vec!["x", "(assign (x) 1)"]);
}

#[test]
fn blocks_and_statements_mix() {
    assert_eq!(s("statement_block_statement"), vec!["a", "(if b (c) ())", "d"]);
    assert_eq!(s("block_then_block"), vec!["(if a (b) ())", "(if c (d) ())"]);
    assert_eq!(s("block_last"), vec!["a", "(if b (c) ())"]);
}

#[test]
fn a_longer_module() {
    assert_eq!(s("many_statements").len(), 5);
}

#[test]
fn a_module_without_a_trailing_newline() {
    assert_eq!(stmts("a = 1"), Ok(vec!["(assign (a) 1)".into()]));
}
