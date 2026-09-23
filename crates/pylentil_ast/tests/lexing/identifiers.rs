use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/identifiers.py", name))
}

#[test]
fn ordinary_identifiers() {
    assert_eq!(tokens("simple"), Ok(vec!["Ident(value)".into()]));
    assert_eq!(tokens("underscore_prefixed"), Ok(vec!["Ident(_private)".into()]));
    assert_eq!(tokens("dunder"), Ok(vec!["Ident(__init__)".into()]));
    assert_eq!(tokens("digits_inside"), Ok(vec!["Ident(item2)".into()]));
    assert_eq!(tokens("single_underscore"), Ok(vec!["Ident(_)".into()]));
}

#[test]
fn a_keyword_is_only_a_keyword_when_it_is_the_whole_word() {
    assert_eq!(tokens("starts_with_keyword"), Ok(vec!["Ident(ifconfig)".into()]));
    assert_eq!(tokens("keyword_with_trailing_underscore"), Ok(vec!["Ident(class_)".into()]));
    assert_eq!(tokens("keyword_embedded"), Ok(vec!["Ident(my_if_helper)".into()]));
    assert_eq!(tokens("keyword_prefix_of_identifier"), Ok(vec!["Ident(nonlocality)".into()]));
}

#[test]
fn every_keyword_lexes_to_a_keyword_token() {
    let toks = tokens("all_keywords").expect("keywords must lex");
    assert_eq!(toks.len(), 35, "expected all 35 keywords");
    let leaked: Vec<&String> = toks.iter().filter(|t| t.starts_with("Ident(")).collect();
    assert!(leaked.is_empty(), "these keywords lexed as identifiers: {leaked:?}");
}

#[test]
fn soft_keywords_are_ordinary_identifiers() {
    assert_eq!(tokens("soft_keyword_match"), Ok(vec!["Ident(match)".into()]));
    assert_eq!(tokens("soft_keyword_case"), Ok(vec!["Ident(case)".into()]));
    assert_eq!(tokens("soft_keyword_type"), Ok(vec!["Ident(type)".into()]));
}

#[test]
fn characters_that_are_not_python_are_rejected() {
    for name in ["dollar_sign", "question_mark", "backtick"] {
        assert!(
            matches!(lex_outcome(&case("lexer/identifiers.py", name)), Outcome::Err(_)),
            "{name} must be rejected"
        );
    }
}
