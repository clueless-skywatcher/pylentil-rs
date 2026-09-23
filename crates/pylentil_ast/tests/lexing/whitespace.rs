use super::*;

#[test]
fn mixing_tabs_and_spaces_in_one_indent_is_rejected() {
    for name in ["tab_then_spaces", "spaces_then_tab"] {
        assert_eq!(
            lex_outcome(&case("lexer/whitespace.py", name)).error(),
            Some(&PylentilError::MixedSpacesAndTabs),
            "{name} must be rejected"
        );
    }
}

#[test]
fn repeated_spaces_are_insignificant() {
    assert_eq!(
        content(&case("lexer/whitespace.py", "multiple_spaces_between_tokens")),
        content(&case("lexer/whitespace.py", "no_spaces_between_tokens"))
    );
}

#[test]
fn a_backslash_joins_the_next_line() {
    let code = case("lexer/whitespace.py", "explicit_line_continuation");
    assert_eq!(count_kind(&code, Newline), Ok(1), "a continued line is one logical line");
}

#[test]
fn a_newline_inside_brackets_does_not_break_the_statement() {
    let mut broken = Vec::new();
    for name in [
        "implicit_continuation_parens",
        "implicit_continuation_brackets",
        "implicit_continuation_braces",
        "implicit_continuation_call",
    ] {
        let code = case("lexer/whitespace.py", name);
        match count_kind(&code, Newline) {
            Ok(1) => {}
            other => broken.push(format!("  {name}: {other:?} newlines")),
        }
    }
    assert!(
        broken.is_empty(),
        "newlines inside brackets are not statement breaks:\n{}",
        broken.join("\n")
    );
}

#[test]
fn indentation_inside_brackets_does_not_open_a_block() {
    let code = case("lexer/whitespace.py", "indented_continuation_line_looks_like_block");
    assert_eq!(count_kind(&code, Indent), Ok(0));
}
