use super::*;

#[test]
fn integer_literal() {
    assert_eq!(expr("42\n"), Ok("42".into()));
}

#[test]
fn float_literal() {
    assert_eq!(expr("3.5\n"), Ok("3.5".into()));
}

#[test]
fn string_literal() {
    assert_eq!(expr("\"text\"\n"), Ok("str(text)".into()));
}

#[test]
fn booleans_and_none() {
    assert_eq!(expr("True\n"), Ok("True".into()));
    assert_eq!(expr("False\n"), Ok("False".into()));
    assert_eq!(expr("None\n"), Ok("None".into()));
}

#[test]
fn a_name() {
    assert_eq!(expr("value\n"), Ok("value".into()));
}

#[test]
fn addition() {
    assert_eq!(expr("1 + 2\n"), Ok("(+ 1 2)".into()));
}

#[test]
fn multiplication_binds_tighter_than_addition() {
    assert_eq!(expr("1 + 2 * 3\n"), Ok("(+ 1 (* 2 3))".into()));
}

#[test]
fn parentheses_are_transparent() {
    assert_eq!(expr("(42)\n"), Ok("42".into()));
    assert_eq!(expr("((42))\n"), Ok("42".into()));
}

#[test]
fn a_comparison() {
    assert_eq!(expr("a == b\n"), Ok("(compare a == b)".into()));
}

#[test]
fn a_parenthesized_tuple() {
    assert_eq!(expr("(a, b)\n"), Ok("(ptuple a b)".into()));
}

#[test]
fn simple_assignment() {
    assert_eq!(one_stmt("x = 1\n"), Ok("(assign (x) 1)".into()));
}

#[test]
fn an_if_statement() {
    assert_eq!(one_stmt("if a:\n    b\n"), Ok("(if a (b) ())".into()));
}

#[test]
fn an_if_else_statement() {
    assert_eq!(one_stmt("if a:\n    b\nelse:\n    c\n"), Ok("(if a (b) (c))".into()));
}

#[test]
fn a_module_holds_several_statements() {
    assert_eq!(stmts("a\nb\nc\n"), Ok(vec!["a".into(), "b".into(), "c".into()]));
}

#[test]
fn an_empty_module_has_no_statements() {
    assert_eq!(stmts(""), Ok(vec![]));
    assert_eq!(stmts("\n\n\n"), Ok(vec![]));
}
