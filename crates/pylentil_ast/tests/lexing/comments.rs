use super::*;
use pylentil_ast::PyLexer;

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
    p_assert_eq!(
        content(&case("lexer/comments.py", "hash_in_string_not_comment")),
        Ok(vec![
            "Ident(x)".into(),
            "Assign".into(),
            "String(# not a comment)".into()
        ])
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
    p_assert_eq!(content("x = 1  # assign one\n"), content("x = 1\n"));
}

#[test]
fn a_comment_does_not_open_or_close_a_block() {
    let code = "if a:\n    # explain\n    b\n";
    p_assert_eq!(count_kind(code, Indent), Ok(1));
    p_assert_eq!(count_kind(code, Dedent), Ok(1));
}

#[test]
fn a_file_of_only_comments_lexes_to_no_content() {
    p_assert_eq!(content("# nothing but this\n"), Ok(vec![]));
}

#[test]
fn a_comment_at_end_of_file_without_newline_lexes() {
    p_assert_eq!(content("x = 1  # assign one"), content("x = 1\n"));
}

#[test]
fn a_comment_at_end_of_file_without_newline_still_closes_blocks() {
    let code = "if a:\n    b  # last";
    p_assert_eq!(count_kind(code, Indent), Ok(1));
    p_assert_eq!(count_kind(code, Dedent), Ok(1));
    p_assert_eq!(count_kind(code, EOF), Ok(1));
}

#[test]
fn a_comment_at_end_of_file_without_newline_is_recorded() {
    let code = "x = 1  # last";
    let lexer = PyLexer::from_code(code).unwrap();
    let comments: Vec<(&str, usize, Option<usize>)> = lexer
        .comments
        .iter()
        .map(|c| (c.comment, c.span.start, c.span.end))
        .collect();
    p_assert_eq!(comments, vec![("# last", 7, Some(13))]);
}

#[test]
fn a_comment_excludes_the_carriage_return() {
    let lexer = PyLexer::from_code("x = 1  # crlf\r\n").unwrap();
    let comments: Vec<&str> = lexer.comments.iter().map(|c| c.comment).collect();
    p_assert_eq!(comments, vec!["# crlf"]);
}

#[test]
fn a_comment_with_non_ascii_text_is_recorded_whole() {
    let lexer = PyLexer::from_code("x = 1  # café ☕\ny = 2\n").unwrap();
    let comments: Vec<&str> = lexer.comments.iter().map(|c| c.comment).collect();
    p_assert_eq!(comments, vec!["# café ☕"]);
    p_assert_eq!(count_kind("x = 1  # café ☕\ny = 2\n", Newline), Ok(2));
}
