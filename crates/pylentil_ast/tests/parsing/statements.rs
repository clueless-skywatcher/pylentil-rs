use super::*;
const F: &str = "parser/statements.py";

#[test]
fn simple_statements() {
    assert_eq!(one_stmt(&case(F, "pass")), Ok("pass".into()));
    assert_eq!(one_stmt(&case(F, "break")), Ok("break".into()));
    assert_eq!(one_stmt(&case(F, "continue")), Ok("continue".into()));
}

#[test]
fn loops() {
    assert_eq!(one_stmt(&case(F, "while_loop")), Ok("(while a (b) ())".into()));
    assert_eq!(one_stmt(&case(F, "for_loop")), Ok("(for i items ((call print i)))".into()));
}

#[test]
fn function_definitions() {
    assert_eq!(one_stmt(&case(F, "function_no_arguments")), Ok("(def f (pass))".into()));
    assert_eq!(one_stmt(&case(F, "function_definition")), Ok("(def f ((return a)))".into()));
}

#[test]
fn class_definitions() {
    assert_eq!(one_stmt(&case(F, "class_definition")), Ok("(class C (pass))".into()));
}

#[test]
fn statements_separated_by_semicolons() {
    assert_eq!(
        stmts(&case(F, "semicolon_separated")),
        Ok(vec!["(assign (a) 1)".into(), "(assign (b) 2)".into()])
    );
}

#[test]
fn every_statement_form_parses() {
    let failures = parse_failures(F);
    assert!(
        failures.is_empty(),
        "{} of {} statement forms fail to parse:\n{}",
        failures.len(),
        cases(F).len(),
        failures.join("\n")
    );
}
