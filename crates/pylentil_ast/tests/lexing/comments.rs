use super::*;

#[test]
fn lex_comments_full_line() {
    assert_lexes("lexer/comments.py", "full_line");
}

#[test]
fn lex_comments_trailing() {
    assert_lexes("lexer/comments.py", "trailing");
}

#[test]
fn lex_comments_hash_inside_comment() {
    assert_lexes("lexer/comments.py", "hash_inside_comment");
}

#[test]
fn lex_comments_comment_between_statements() {
    assert_lexes("lexer/comments.py", "comment_between_statements");
}

#[test]
fn lex_comments_indented_comment() {
    assert_lexes("lexer/comments.py", "indented_comment");
}

#[test]
fn lex_comments_comment_at_module_indent_inside_block() {
    assert_lexes("lexer/comments.py", "comment_at_module_indent_inside_block");
}

#[test]
fn lex_comments_comment_only_file() {
    assert_lexes("lexer/comments.py", "comment_only_file");
}

#[test]
fn lex_comments_shebang() {
    assert_lexes("lexer/comments.py", "shebang");
}

#[test]
fn lex_comments_coding_declaration() {
    assert_lexes("lexer/comments.py", "coding_declaration");
}

#[test]
fn lex_comments_hash_in_string_not_comment() {
    assert_eq!(
        content(&case("lexer/comments.py", "hash_in_string_not_comment")),
        Ok(vec!["Ident(x)".into(), "Assign".into(), "String(# not a comment)".into()])
    );
}

#[test]
fn lex_comments_comment_after_colon() {
    assert_lexes("lexer/comments.py", "comment_after_colon");
}

// ------------------------------------------------ inline snippets --
// These check hand-written code, not fixture cases.

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
