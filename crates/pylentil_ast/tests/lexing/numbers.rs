use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/numbers.py", name))
}

#[test]
fn decimal_integers() {
    assert_eq!(tokens("decimal_integer"), Ok(vec!["Int(42)".into()]));
    assert_eq!(tokens("zero"), Ok(vec!["Int(0)".into()]));
    assert_eq!(tokens("leading_zeros"), Ok(vec!["Int(007)".into()]));
}

#[test]
fn floats() {
    assert_eq!(tokens("simple_float"), Ok(vec!["Float(3.14)".into()]));
    assert_eq!(tokens("float_trailing_dot"), Ok(vec!["Float(2.)".into()]));
}

#[test]
fn huge_integers_lex_even_though_they_overflow_i64() {
    // The lexer only carries the digits; range is the parser's problem.
    assert!(tokens("huge_integer").is_ok());
}

#[test]
fn a_number_followed_by_a_keyword_splits() {
    // Python tokenises `1if` as 1 then `if`.
    assert_eq!(tokens("number_touching_identifier"), Ok(vec!["Int(1)".into(), "If".into()]));
}

#[test]
fn leading_dot_float_is_one_number() {
    assert_eq!(tokens("float_leading_dot"), Ok(vec!["Float(.5)".into()]));
}

#[test]
fn underscore_separators_are_part_of_the_number() {
    assert_eq!(tokens("underscore_separated"), Ok(vec!["Int(1_000_000)".into()]));
}

#[test]
fn hex_octal_and_binary_literals_are_single_int_tokens() {
    assert_eq!(tokens("hexadecimal"), Ok(vec!["Hexadecimal(deadbeef)".into()]));
    assert_eq!(tokens("hexadecimal_lowercase"), Ok(vec!["Hexadecimal(1f)".into()]));
    assert_eq!(tokens("binary"), Ok(vec!["Binary(1010)".into()]));
    assert_eq!(tokens("octal"), Ok(vec!["Octal(755)".into()]));
}

#[test]
fn exponent_notation_is_a_single_token() {
    assert_eq!(tokens("exponent"), Ok(vec!["ENotation(1e10)".into()]));
    assert_eq!(tokens("exponent_capital"), Ok(vec!["ENotation(1E10)".into()]));
    assert_eq!(tokens("negative_exponent"), Ok(vec!["ENotation(1.5e-3)".into()]));
}

#[test]
fn imaginary_literals_are_a_single_token() {
    assert_eq!(tokens("imaginary").map(|t| t.len()), Ok(1));
    assert_eq!(tokens("imaginary_float").map(|t| t.len()), Ok(1));
}
