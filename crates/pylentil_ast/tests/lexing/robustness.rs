use super::*;

#[test]
fn a_file_without_a_trailing_newline_is_fine() {
    assert_eq!(content("x = 1"), Ok(vec!["Ident(x)".into(), "Assign".into(), "Int(1)".into()]));
}

#[test]
fn a_file_of_only_newlines_has_no_content() {
    assert_eq!(content("\n\n\n"), Ok(vec![]));
}

#[test]
fn a_whitespace_only_file_opens_no_block() {
    // Leading spaces on an otherwise blank line are not an indent.
    assert_eq!(count_kind("   \n\n", Indent), Ok(0));
}

#[test]
fn crlf_line_endings_are_accepted() {
    assert_eq!(kinds("x = 1\r\ny = 2\r\n").map(|k| k.len()), Ok(9));
}

#[test]
fn a_utf8_bom_is_skipped() {
    assert_eq!(content("\u{feff}x = 1\n").map(|t| t.len()), Ok(3));
}

#[test]
fn a_null_byte_is_rejected() {
    assert!(matches!(lex_outcome("x = \0\n"), Outcome::Err(_)));
}

#[test]
fn deeply_nested_brackets_do_not_blow_the_stack() {
    let code = format!("{}{}\n", "(".repeat(500), ")".repeat(500));
    assert!(!matches!(lex_outcome(&code), Outcome::Panic(_)));
}

#[test]
fn a_long_file_lexes_correctly() {
    // NB: `indent_pass` re-clones the remaining token vector per token, so
    // this is quadratic — 100 lines ~10ms, 800 lines ~600ms. Correctness
    // only here; the cost is recorded in the parser's own notes.
    let code: String = (0..200).map(|i| format!("x{i} = {i} + 1\n")).collect();
    assert_eq!(kinds(&code).map(|k| k.len()), Ok(200 * 6 + 1));
}
