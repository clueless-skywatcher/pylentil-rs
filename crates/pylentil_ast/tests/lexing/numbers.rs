use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/numbers.py", name))
}

#[test]
fn lex_numbers_decimal_integer() {
    assert_eq!(tokens("decimal_integer"), Ok(vec!["Int(42)".into()]));
}

#[test]
fn lex_numbers_zero() {
    assert_eq!(tokens("zero"), Ok(vec!["Int(0)".into()]));
}

#[test]
fn lex_numbers_leading_zeros() {
    assert_eq!(tokens("leading_zeros"), Ok(vec!["Int(007)".into()]));
}

#[test]
fn lex_numbers_huge_integer() {
    // The lexer only carries the digits; range is the parser's problem.
    assert!(tokens("huge_integer").is_ok());
}

#[test]
fn lex_numbers_simple_float() {
    assert_eq!(tokens("simple_float"), Ok(vec!["Float(3.14)".into()]));
}

#[test]
fn lex_numbers_float_trailing_dot() {
    assert_eq!(tokens("float_trailing_dot"), Ok(vec!["Float(2.)".into()]));
}

#[test]
fn lex_numbers_float_leading_dot() {
    assert_eq!(tokens("float_leading_dot"), Ok(vec!["Float(.5)".into()]));
}

#[test]
fn lex_numbers_underscore_separated() {
    assert_eq!(tokens("underscore_separated"), Ok(vec!["Int(1_000_000)".into()]));
}

#[test]
fn lex_numbers_hexadecimal() {
    assert_eq!(tokens("hexadecimal"), Ok(vec!["Hexadecimal(deadbeef)".into()]));
}

#[test]
fn lex_numbers_hexadecimal_lowercase() {
    assert_eq!(tokens("hexadecimal_lowercase"), Ok(vec!["Hexadecimal(1f)".into()]));
}

#[test]
fn lex_numbers_binary() {
    assert_eq!(tokens("binary"), Ok(vec!["Binary(1010)".into()]));
}

#[test]
fn lex_numbers_octal() {
    assert_eq!(tokens("octal"), Ok(vec!["Octal(755)".into()]));
}

#[test]
fn lex_numbers_exponent() {
    assert_eq!(tokens("exponent"), Ok(vec!["ENotation(1e10)".into()]));
}

#[test]
fn lex_numbers_exponent_capital() {
    assert_eq!(tokens("exponent_capital"), Ok(vec!["ENotation(1E10)".into()]));
}

#[test]
fn lex_numbers_negative_exponent() {
    assert_eq!(tokens("negative_exponent"), Ok(vec!["ENotation(1.5e-3)".into()]));
}

#[test]
fn lex_numbers_imaginary() {
    assert_eq!(tokens("imaginary").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_numbers_imaginary_float() {
    assert_eq!(tokens("imaginary_float").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_numbers_malformed_two_dots() {
    assert_lex_no_panic("lexer/numbers.py", "malformed_two_dots");
}

#[test]
fn lex_numbers_number_touching_identifier() {
    // Python tokenises `1if` as 1 then `if`.
    assert_eq!(tokens("number_touching_identifier"), Ok(vec!["Int(1)".into(), "If".into()]));
}
