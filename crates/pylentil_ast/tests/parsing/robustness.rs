use super::*;

#[test]
fn a_malformed_float_is_rejected() {
    assert!(matches!(parse_outcome("1.2.3\n"), Outcome::Err(_)));
}

#[test]
fn an_identifier_cannot_start_with_a_digit() {
    assert!(matches!(parse_outcome("2fast\n"), Outcome::Err(_)));
}

#[test]
fn deep_nesting_does_not_blow_the_stack() {
    let code = format!("{}1{}\n", "(".repeat(200), ")".repeat(200));
    assert!(!matches!(parse_outcome(&code), Outcome::Panic(_)));
}

#[test]
fn a_stray_layout_token_is_never_a_statement() {
    // Whatever the lexer emits, the parser must not accept a bare Dedent or
    // Newline as an expression.
    assert!(matches!(parse_outcome("if a:\n    b\n  c\n"), Outcome::Err(_)));
}

#[test]
fn nothing_in_the_fixtures_makes_the_parser_panic() {
    let mut panics = Vec::new();
    for fixture in [
        "parser/literals.py",
        "parser/arithmetic.py",
        "parser/bitwise.py",
        "parser/comparisons.py",
        "parser/boolean.py",
        "parser/tuples.py",
        "parser/assignment.py",
        "parser/calls.py",
        "parser/subscripts.py",
        "parser/collections.py",
        "parser/if_statements.py",
        "parser/statements.py",
        "parser/imports.py",
        "parser/functions.py",
        "parser/modules.py",
    ] {
        for c in cases(fixture) {
            if let Outcome::Panic(m) = parse_outcome(&c.code) {
                panics.push(format!("  {fixture} :: {}: {m}", c.name));
            }
        }
    }
    assert!(
        panics.is_empty(),
        "invalid input must produce errors, never panics:\n{}",
        panics.join("\n")
    );
}
