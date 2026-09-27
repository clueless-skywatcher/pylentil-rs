use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/numbers.py", name))
}

#[test]
fn lex_numbers_decimal_integer() {
    p_assert_eq!(tokens("decimal_integer"), Ok(vec!["Int(42)".into()]));
}

#[test]
fn lex_numbers_zero() {
    p_assert_eq!(tokens("zero"), Ok(vec!["Int(0)".into()]));
}

#[test]
fn lex_numbers_leading_zeros() {
    p_assert_eq!(tokens("leading_zeros"), Ok(vec!["Int(007)".into()]));
}

#[test]
fn lex_numbers_huge_integer() {
    // The lexer only carries the digits; range is the parser's problem.
    assert!(tokens("huge_integer").is_ok());
}

#[test]
fn lex_numbers_simple_float() {
    p_assert_eq!(tokens("simple_float"), Ok(vec!["Float(3.14)".into()]));
}

#[test]
fn lex_numbers_float_trailing_dot() {
    p_assert_eq!(tokens("float_trailing_dot"), Ok(vec!["Float(2.)".into()]));
}

#[test]
fn lex_numbers_float_leading_dot() {
    p_assert_eq!(tokens("float_leading_dot"), Ok(vec!["Float(.5)".into()]));
}

#[test]
fn lex_numbers_underscore_separated() {
    p_assert_eq!(tokens("underscore_separated"), Ok(vec!["Int(1_000_000)".into()]));
}

#[test]
fn lex_numbers_hexadecimal() {
    p_assert_eq!(tokens("hexadecimal"), Ok(vec!["Hexadecimal(deadbeef)".into()]));
}

#[test]
fn lex_numbers_hexadecimal_lowercase() {
    p_assert_eq!(tokens("hexadecimal_lowercase"), Ok(vec!["Hexadecimal(1f)".into()]));
}

#[test]
fn lex_numbers_binary() {
    p_assert_eq!(tokens("binary"), Ok(vec!["Binary(1010)".into()]));
}

#[test]
fn lex_numbers_octal() {
    p_assert_eq!(tokens("octal"), Ok(vec!["Octal(755)".into()]));
}

#[test]
fn lex_numbers_exponent() {
    p_assert_eq!(tokens("exponent"), Ok(vec!["ENotation(1e10)".into()]));
}

#[test]
fn lex_numbers_exponent_capital() {
    p_assert_eq!(tokens("exponent_capital"), Ok(vec!["ENotation(1E10)".into()]));
}

#[test]
fn lex_numbers_negative_exponent() {
    p_assert_eq!(tokens("negative_exponent"), Ok(vec!["ENotation(1.5e-3)".into()]));
}

#[test]
fn lex_numbers_imaginary() {
    p_assert_eq!(tokens("imaginary").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_numbers_imaginary_float() {
    p_assert_eq!(tokens("imaginary_float").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_numbers_malformed_two_dots() {
    assert_lex_no_panic("lexer/numbers.py", "malformed_two_dots");
}

#[test]
fn lex_numbers_number_touching_identifier() {
    // Python tokenises `1if` as 1 then `if`.
    p_assert_eq!(tokens("number_touching_identifier"), Ok(vec!["Int(1)".into(), "If".into()]));
}
