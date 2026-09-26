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
