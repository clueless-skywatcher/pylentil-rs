use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/operators.py", name))
}

#[test]
fn lex_operators_arithmetic() {
    p_assert_eq!(tokens("arithmetic").map(|t| t.len()), Ok(11));
}

#[test]
fn lex_operators_power() {
    p_assert_eq!(
        tokens("power"),
        Ok(vec![
            "Ident(a)".into(),
            "DoubleStar".into(),
            "Ident(b)".into()
        ])
    );
}

#[test]
fn lex_operators_floor_division() {
    p_assert_eq!(tokens("floor_division").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_matrix_multiply() {
    p_assert_eq!(tokens("matrix_multiply").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_left_shift() {
    p_assert_eq!(
        tokens("left_shift"),
        Ok(vec!["Ident(a)".into(), "LShift".into(), "Ident(b)".into()])
    );
}

#[test]
fn lex_operators_right_shift() {
    p_assert_eq!(
        tokens("right_shift"),
        Ok(vec!["Ident(a)".into(), "RShift".into(), "Ident(b)".into()])
    );
}

#[test]
fn lex_operators_comparisons() {
    p_assert_eq!(tokens("comparisons").map(|t| t.len()), Ok(5));
}

#[test]
fn lex_operators_relational() {
    p_assert_eq!(
        tokens("relational"),
        Ok(vec![
            "Ident(a)".into(),
            "Less".into(),
            "Ident(b)".into(),
            "Greater".into(),
            "Ident(c)".into()
        ])
    );
}

#[test]
fn lex_operators_relational_or_equal() {
    p_assert_eq!(
        tokens("relational_or_equal"),
        Ok(vec![
            "Ident(a)".into(),
            "LessEqual".into(),
            "Ident(b)".into(),
            "GreaterEqual".into(),
            "Ident(c)".into()
        ])
    );
}

#[test]
fn lex_operators_greater_then_less_adjacent() {
    let toks = tokens("greater_then_less_adjacent");
    p_assert_eq!(
        toks,
        Ok(vec![
            "Ident(a)".into(),
            "Greater".into(),
            "Less".into(),
            "Ident(b)".into()
        ]),
        "`><` is two comparison tokens, never a shift"
    );
}

#[test]
fn lex_operators_less_then_greater_adjacent() {
    p_assert_eq!(
        tokens("less_then_greater_adjacent"),
        Ok(vec![
            "Ident(a)".into(),
            "Less".into(),
            "Greater".into(),
            "Ident(b)".into()
        ]),
        "`<>` is two comparison tokens"
    );
}

#[test]
fn lex_operators_bitwise() {
    p_assert_eq!(tokens("bitwise").map(|t| t.len()), Ok(7));
}

#[test]
fn lex_operators_invert() {
    p_assert_eq!(
        tokens("invert"),
        Ok(vec!["Tilde".into(), "Ident(a)".into()])
    );
}

#[test]
fn lex_operators_augmented_add() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_add").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_sub() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_sub").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_mul() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_mul").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_div() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_div").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_floordiv() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_floordiv").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_pow() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_pow").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_mod() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_mod").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_bitand() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_bitand").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_bitor() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_bitor").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_bitxor() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_bitxor").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_augmented_shift() {
    // `a OP= 1` is three tokens in Python.
    p_assert_eq!(tokens("augmented_shift").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_walrus() {
    p_assert_eq!(tokens("walrus").map(|t| t.len()), Ok(3));
}

#[test]
fn lex_operators_arrow() {
    // def f() -> int:  ->  Def Ident LParen RParen Arrow Ident Colon
    p_assert_eq!(tokens("arrow").map(|t| t.len()), Ok(7));
}

#[test]
fn lex_operators_ellipsis() {
    p_assert_eq!(tokens("ellipsis").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_operators_decorator() {
    assert!(lex_outcome(&case("lexer/operators.py", "decorator")).is_accepted());
}

#[test]
fn lex_operators_semicolon_separated() {
    p_assert_eq!(tokens("semicolon_separated").map(|t| t.len()), Ok(7));
}

#[test]
fn lex_operators_lone_bang() {
    assert!(matches!(
        lex_outcome(&case("lexer/operators.py", "lone_bang")),
        Outcome::Err(_)
    ));
}

#[test]
fn lex_operators_brackets() {
    p_assert_eq!(tokens("brackets").map(|t| t.len()), Ok(6));
}
