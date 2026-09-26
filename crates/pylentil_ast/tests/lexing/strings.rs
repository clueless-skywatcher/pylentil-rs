use super::*;

fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
    content(&case("lexer/strings.py", name))
}

#[test]
fn lex_strings_double_quoted() {
    assert_eq!(tokens("double_quoted"), Ok(vec!["String(hello)".into()]));
}

#[test]
fn lex_strings_single_quoted() {
    assert_eq!(tokens("single_quoted"), Ok(vec!["String(hello)".into()]));
}

#[test]
fn lex_strings_empty() {
    assert_eq!(tokens("empty"), Ok(vec!["String()".into()]));
}

#[test]
fn lex_strings_escaped_quote() {
    assert_eq!(tokens("escaped_quote").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_strings_escaped_backslash() {
    assert_eq!(tokens("escaped_backslash").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_strings_escape_newline() {
    assert_eq!(tokens("escape_newline").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_strings_other_quote_inside() {
    assert_eq!(tokens("other_quote_inside"), Ok(vec!["String(it's fine)".into()]));
}

#[test]
fn lex_strings_triple_double() {
    assert_eq!(tokens("triple_double"), Ok(vec!["String(triple quoted)".into()]));
}

#[test]
fn lex_strings_triple_single() {
    assert_eq!(tokens("triple_single"), Ok(vec!["String(triple quoted)".into()]));
}

#[test]
fn lex_strings_triple_spanning_lines() {
    assert_eq!(tokens("triple_spanning_lines").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_strings_f_string() {
    // f/r/b are part of the token, not a separate identifier.
    assert_eq!(tokens("f_string").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_strings_raw_string() {
    assert_eq!(tokens("raw_string").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_strings_bytes_literal() {
    assert_eq!(tokens("bytes_literal").map(|t| t.len()), Ok(1));
}

#[test]
fn lex_strings_non_ascii_contents() {
    assert_eq!(tokens("non_ascii_contents"), Ok(vec!["String(héllo wörld)".into()]));
}

#[test]
fn lex_strings_emoji_contents() {
    assert_eq!(tokens("emoji_contents"), Ok(vec!["String(shipped 🚀)".into()]));
}

#[test]
fn lex_strings_adjacent_concatenation() {
    assert_eq!(tokens("adjacent_concatenation").map(|t| t.len()), Ok(2));
}

#[test]
fn lex_strings_hash_inside_string() {
    assert_eq!(tokens("hash_inside_string"), Ok(vec!["String(# not a comment)".into()]));
}

#[test]
fn lex_strings_quote_terminated_by_newline() {
    assert_lex_rejected("lexer/strings.py", "quote_terminated_by_newline");
}

#[test]
fn lex_strings_unterminated_at_eof() {
    assert!(matches!(lex_outcome(&case("lexer/strings.py", "unterminated_at_eof")), Outcome::Err(_)));
}

#[test]
fn lex_strings_trailing_escape_at_eof() {
    assert!(matches!(lex_outcome(&case("lexer/strings.py", "trailing_escape_at_eof")), Outcome::Err(_)));
}

// ------------------------------------------------ inline snippets --
// These check hand-written code, not fixture cases.

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
