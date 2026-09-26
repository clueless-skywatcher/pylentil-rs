use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/identifiers.py", name))
}

#[test]
fn lex_identifiers_simple() {
    assert_eq!(tokens("simple"), Ok(vec!["Ident(value)".into()]));
}

#[test]
fn lex_identifiers_underscore_prefixed() {
    assert_eq!(tokens("underscore_prefixed"), Ok(vec!["Ident(_private)".into()]));
}

#[test]
fn lex_identifiers_dunder() {
    assert_eq!(tokens("dunder"), Ok(vec!["Ident(__init__)".into()]));
}

#[test]
fn lex_identifiers_digits_inside() {
    assert_eq!(tokens("digits_inside"), Ok(vec!["Ident(item2)".into()]));
}

#[test]
fn lex_identifiers_single_underscore() {
    assert_eq!(tokens("single_underscore"), Ok(vec!["Ident(_)".into()]));
}

#[test]
fn lex_identifiers_starts_with_keyword() {
    assert_eq!(tokens("starts_with_keyword"), Ok(vec!["Ident(ifconfig)".into()]));
}

#[test]
fn lex_identifiers_keyword_with_trailing_underscore() {
    assert_eq!(tokens("keyword_with_trailing_underscore"), Ok(vec!["Ident(class_)".into()]));
}

#[test]
fn lex_identifiers_keyword_embedded() {
    assert_eq!(tokens("keyword_embedded"), Ok(vec!["Ident(my_if_helper)".into()]));
}

#[test]
fn lex_identifiers_keyword_prefix_of_identifier() {
    assert_eq!(tokens("keyword_prefix_of_identifier"), Ok(vec!["Ident(nonlocality)".into()]));
}

#[test]
fn lex_identifiers_all_keywords() {
    let toks = tokens("all_keywords").expect("keywords must lex");
    assert_eq!(toks.len(), 35, "expected all 35 keywords");
    let leaked: Vec<&String> = toks.iter().filter(|t| t.starts_with("Ident(")).collect();
    assert!(leaked.is_empty(), "these keywords lexed as identifiers: {leaked:?}");
}

#[test]
fn lex_identifiers_soft_keyword_match() {
    assert_eq!(tokens("soft_keyword_match"), Ok(vec!["Ident(match)".into()]));
}

#[test]
fn lex_identifiers_soft_keyword_case() {
    assert_eq!(tokens("soft_keyword_case"), Ok(vec!["Ident(case)".into()]));
}

#[test]
fn lex_identifiers_soft_keyword_type() {
    assert_eq!(tokens("soft_keyword_type"), Ok(vec!["Ident(type)".into()]));
}

#[test]
fn lex_identifiers_unicode_identifier() {
    assert_eq!(tokens("unicode_identifier"), Ok(vec!["Ident(café)".into()]));
}

#[test]
fn lex_identifiers_cyrillic_identifier() {
    assert_eq!(tokens("cyrillic_identifier"), Ok(vec!["Ident(переменная)".into()]));
}

#[test]
fn lex_identifiers_leading_digit() {
    assert_lex_no_panic("lexer/identifiers.py", "leading_digit");
}

#[test]
fn lex_identifiers_dollar_sign() {
    assert_lex_rejected("lexer/identifiers.py", "dollar_sign");
}

#[test]
fn lex_identifiers_question_mark() {
    assert_lex_rejected("lexer/identifiers.py", "question_mark");
}

#[test]
fn lex_identifiers_backtick() {
    assert_lex_rejected("lexer/identifiers.py", "backtick");
}
