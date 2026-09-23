use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/operators.py", name))
}

#[test]
fn arithmetic_and_power() {
    assert_eq!(tokens("arithmetic").map(|t| t.len()), Ok(11));
    assert_eq!(
        tokens("power"),
        Ok(vec!["Ident(a)".into(), "DoubleStar".into(), "Ident(b)".into()])
    );
}

#[test]
fn left_shift() {
    assert_eq!(
        tokens("left_shift"),
        Ok(vec!["Ident(a)".into(), "LShift".into(), "Ident(b)".into()])
    );
}

#[test]
fn right_shift() {
    assert_eq!(
        tokens("right_shift"),
        Ok(vec!["Ident(a)".into(), "RShift".into(), "Ident(b)".into()])
    );
}

#[test]
fn a_greater_followed_by_a_less_is_not_a_shift() {
    let toks = tokens("greater_then_less_adjacent");
    assert_eq!(
        toks,
        Ok(vec!["Ident(a)".into(), "Greater".into(), "Less".into(), "Ident(b)".into()]),
        "`><` is two comparison tokens, never a shift"
    );
}

#[test]
fn comparisons_and_bitwise() {
    assert_eq!(tokens("comparisons").map(|t| t.len()), Ok(5));
    assert_eq!(tokens("bitwise").map(|t| t.len()), Ok(7));
    assert_eq!(tokens("invert"), Ok(vec!["Tilde".into(), "Ident(a)".into()]));
}

#[test]
fn a_lone_bang_is_rejected() {
    assert!(matches!(lex_outcome(&case("lexer/operators.py", "lone_bang")), Outcome::Err(_)));
}

#[test]
fn brackets_and_semicolons() {
    assert_eq!(tokens("brackets").map(|t| t.len()), Ok(6));
    assert_eq!(tokens("semicolon_separated").map(|t| t.len()), Ok(7));
}

#[test]
fn floor_division_is_one_token() {
    assert_eq!(tokens("floor_division").map(|t| t.len()), Ok(3));
}

#[test]
fn matrix_multiplication_operator() {
    assert_eq!(tokens("matrix_multiply").map(|t| t.len()), Ok(3));
}

#[test]
fn augmented_assignment_operators_are_single_tokens() {
    let mut broken = Vec::new();
    for c in cases("lexer/operators.py") {
        if !c.name.starts_with("augmented_") {
            continue;
        }
        // `a OP= 1` is three tokens in Python.
        match content(&c.code) {
            Ok(t) if t.len() == 3 => {}
            other => broken.push(format!("  {}: {other:?}", c.name)),
        }
    }
    assert!(broken.is_empty(), "augmented assignment must be one token:\n{}", broken.join("\n"));
}

#[test]
fn walrus_is_one_token() {
    assert_eq!(tokens("walrus").map(|t| t.len()), Ok(3));
}

#[test]
fn return_arrow_is_one_token() {
    // def f() -> int:  ->  Def Ident LParen RParen Arrow Ident Colon
    assert_eq!(tokens("arrow").map(|t| t.len()), Ok(7));
}

#[test]
fn decorator_at_sign_is_lexable() {
    assert!(lex_outcome(&case("lexer/operators.py", "decorator")).is_accepted());
}

#[test]
fn ellipsis_is_one_token() {
    assert_eq!(tokens("ellipsis").map(|t| t.len()), Ok(1));
}
