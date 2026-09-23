use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/strings.py", name))
}

#[test]
fn both_quote_styles_carry_the_same_value() {
    assert_eq!(tokens("double_quoted"), Ok(vec!["String(hello)".into()]));
    assert_eq!(tokens("single_quoted"), Ok(vec!["String(hello)".into()]));
}

#[test]
fn an_empty_string_is_still_a_string_token() {
    assert_eq!(tokens("empty"), Ok(vec!["String()".into()]));
}

#[test]
fn an_escaped_quote_does_not_end_the_string() {
    assert_eq!(tokens("escaped_quote").map(|t| t.len()), Ok(1));
}

#[test]
fn an_escaped_backslash_does_not_escape_the_closing_quote() {
    assert_eq!(tokens("escaped_backslash").map(|t| t.len()), Ok(1));
}

#[test]
fn the_other_quote_style_is_ordinary_content() {
    assert_eq!(tokens("other_quote_inside"), Ok(vec!["String(it's fine)".into()]));
}

#[test]
fn non_ascii_content_survives_byte_indexing() {
    assert_eq!(tokens("non_ascii_contents"), Ok(vec!["String(héllo wörld)".into()]));
    assert_eq!(tokens("emoji_contents"), Ok(vec!["String(shipped 🚀)".into()]));
}

#[test]
fn adjacent_literals_stay_two_tokens() {
    assert_eq!(tokens("adjacent_concatenation").map(|t| t.len()), Ok(2));
}

#[test]
fn a_hash_inside_a_string_is_not_a_comment() {
    assert_eq!(tokens("hash_inside_string"), Ok(vec!["String(# not a comment)".into()]));
}

#[test]
fn an_unterminated_string_is_rejected() {
    assert!(matches!(lex_outcome(&case("lexer/strings.py", "unterminated_at_eof")), Outcome::Err(_)));
}

#[test]
fn a_trailing_escape_at_eof_is_rejected() {
    assert!(matches!(lex_outcome(&case("lexer/strings.py", "trailing_escape_at_eof")), Outcome::Err(_)));
}

#[test]
fn a_single_quoted_string_cannot_span_a_line_break() {
    // Python: unterminated string literal on line 1. The danger is silently
    // swallowing the next line and closing on a *later* quote.
    let code = "a = \"unterminated\nb = \"also\"\n";
    assert!(
        matches!(lex_outcome(code), Outcome::Err(_)),
        "a newline must terminate a single-quoted string, got {:?}",
        kinds(code)
    );
}

#[test]
fn triple_quoted_strings_are_one_token() {
    assert_eq!(tokens("triple_double"), Ok(vec!["String(triple quoted)".into()]));
    assert_eq!(tokens("triple_single"), Ok(vec!["String(triple quoted)".into()]));
}

#[test]
fn triple_quoted_strings_may_span_lines() {
    assert_eq!(tokens("triple_spanning_lines").map(|t| t.len()), Ok(1));
}

#[test]
fn string_prefixes_belong_to_the_literal() {
    // f/r/b are part of the token, not a separate identifier.
    assert_eq!(tokens("f_string").map(|t| t.len()), Ok(1));
    assert_eq!(tokens("raw_string").map(|t| t.len()), Ok(1));
    assert_eq!(tokens("bytes_literal").map(|t| t.len()), Ok(1));
}
