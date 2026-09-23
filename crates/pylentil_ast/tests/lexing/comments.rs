use super::*;

#[test]
fn a_hash_inside_a_string_is_not_a_comment() {
    assert_eq!(
        content(&case("lexer/comments.py", "hash_in_string_not_comment")),
        Ok(vec!["Ident(x)".into(), "Assign".into(), "String(# not a comment)".into()])
    );
}

#[test]
fn comments_are_ignored_everywhere_they_may_appear() {
    let failures = lex_failures("lexer/comments.py");
    assert!(
        failures.is_empty(),
        "comments must be skipped by the lexer:\n{}",
        failures.join("\n")
    );
}

#[test]
fn a_comment_contributes_no_tokens() {
    assert_eq!(content("x = 1  # assign one\n"), content("x = 1\n"));
}

#[test]
fn a_comment_does_not_open_or_close_a_block() {
    let code = "if a:\n    # explain\n    b\n";
    assert_eq!(count_kind(code, Indent), Ok(1));
    assert_eq!(count_kind(code, Dedent), Ok(1));
}

#[test]
fn a_file_of_only_comments_lexes_to_no_content() {
    assert_eq!(content("# nothing but this\n"), Ok(vec![]));
}
