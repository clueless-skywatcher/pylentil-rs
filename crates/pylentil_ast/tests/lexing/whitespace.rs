use super::*;

#[test]
fn lex_whitespace_tab_then_spaces() {
    assert_eq!(
        lex_outcome(&case("lexer/whitespace.py", "tab_then_spaces")).error(),
        Some(&PylentilError::MixedSpacesAndTabs)
    );
}

#[test]
fn lex_whitespace_spaces_then_tab() {
    assert_eq!(
        lex_outcome(&case("lexer/whitespace.py", "spaces_then_tab")).error(),
        Some(&PylentilError::MixedSpacesAndTabs)
    );
}

#[test]
fn lex_whitespace_tabs_for_first_block_spaces_for_second() {
    assert_lexes("lexer/whitespace.py", "tabs_for_first_block_spaces_for_second");
}

#[test]
fn lex_whitespace_explicit_line_continuation() {
    let code = case("lexer/whitespace.py", "explicit_line_continuation");
    assert_eq!(count_kind(&code, Newline), Ok(1), "a continued line is one logical line");
}

#[test]
fn lex_whitespace_continuation_in_condition() {
    assert_lexes("lexer/whitespace.py", "continuation_in_condition");
}

#[test]
fn lex_whitespace_implicit_continuation_parens() {
    let code = case("lexer/whitespace.py", "implicit_continuation_parens");
    assert_eq!(count_kind(&code, Newline), Ok(1), "a newline inside brackets is not a statement break");
}

#[test]
fn lex_whitespace_implicit_continuation_brackets() {
    let code = case("lexer/whitespace.py", "implicit_continuation_brackets");
    assert_eq!(count_kind(&code, Newline), Ok(1), "a newline inside brackets is not a statement break");
}

#[test]
fn lex_whitespace_implicit_continuation_braces() {
    let code = case("lexer/whitespace.py", "implicit_continuation_braces");
    assert_eq!(count_kind(&code, Newline), Ok(1), "a newline inside brackets is not a statement break");
}

#[test]
fn lex_whitespace_implicit_continuation_call() {
    let code = case("lexer/whitespace.py", "implicit_continuation_call");
    assert_eq!(count_kind(&code, Newline), Ok(1), "a newline inside brackets is not a statement break");
}

#[test]
fn lex_whitespace_indented_continuation_line_looks_like_block() {
    let code = case("lexer/whitespace.py", "indented_continuation_line_looks_like_block");
    assert_eq!(count_kind(&code, Indent), Ok(0));
}

#[test]
fn lex_whitespace_multiple_spaces_between_tokens() {
    assert_eq!(
        content(&case("lexer/whitespace.py", "multiple_spaces_between_tokens")),
        content(&case("lexer/whitespace.py", "no_spaces_between_tokens"))
    );
}

#[test]
fn lex_whitespace_no_spaces_between_tokens() {
    assert_lexes("lexer/whitespace.py", "no_spaces_between_tokens");
}

#[test]
fn lex_whitespace_space_before_colon() {
    assert_lexes("lexer/whitespace.py", "space_before_colon");
}
