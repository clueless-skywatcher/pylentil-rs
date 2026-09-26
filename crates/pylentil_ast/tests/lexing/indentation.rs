use super::*;

fn layout(name: &str) -> (Result<usize, PylentilError>, Result<usize, PylentilError>) {
    let code = case("lexer/indentation.py", name);
    (count_kind(&code, Indent), count_kind(&code, Dedent))
}

#[test]
fn lex_indentation_single_level() {
    assert_eq!(layout("single_level"), (Ok(1), Ok(1)));
}

#[test]
fn lex_indentation_two_levels() {
    assert_eq!(layout("two_levels"), (Ok(2), Ok(2)));
}

#[test]
fn lex_indentation_dedent_one_level() {
    assert_eq!(layout("dedent_one_level"), (Ok(2), Ok(2)));
}

#[test]
fn lex_indentation_dedent_to_module_level() {
    assert_eq!(layout("dedent_to_module_level"), (Ok(2), Ok(2)));
}

#[test]
fn lex_indentation_tab_indent() {
    assert_eq!(layout("tab_indent"), (Ok(1), Ok(1)));
}

#[test]
fn lex_indentation_eight_space_indent() {
    assert_eq!(layout("eight_space_indent"), (Ok(1), Ok(1)));
}

#[test]
fn lex_indentation_blank_line_inside_block() {
    assert_eq!(layout("blank_line_inside_block"), (Ok(1), Ok(1)));
}

#[test]
fn lex_indentation_blank_line_between_blocks() {
    assert_eq!(layout("blank_line_between_blocks"), (Ok(2), Ok(2)));
}

#[test]
fn lex_indentation_indented_blank_line_inside_block() {
    let (indents, dedents) = layout("indented_blank_line_inside_block");
    assert_eq!(indents, dedents, "layout tokens must balance");
}

#[test]
fn lex_indentation_first_line_indented() {
    // Python: IndentationError: unexpected indent
    assert!(
        matches!(lex_outcome(&case("lexer/indentation.py", "first_line_indented")), Outcome::Err(_)),
        "module-level code cannot start indented"
    );
}

#[test]
fn lex_indentation_inconsistent_dedent() {
    // Python: IndentationError: unindent does not match any outer level
    match lex_outcome(&case("lexer/indentation.py", "inconsistent_dedent")) {
        Outcome::Err(_) => {}
        Outcome::Ok(t) => panic!("accepted an inconsistent dedent: {t:?}"),
        Outcome::Panic(m) => panic!("panicked instead of erroring: {m}"),
    }
}

#[test]
fn lex_indentation_dedent_to_unknown_level() {
    match lex_outcome(&case("lexer/indentation.py", "dedent_to_unknown_level")) {
        Outcome::Err(_) => {}
        Outcome::Ok(t) => panic!("accepted a dedent to an unvisited level: {t:?}"),
        Outcome::Panic(m) => panic!("panicked instead of erroring: {m}"),
    }
}

#[test]
fn lex_indentation_reindent_after_dedent() {
    assert_eq!(layout("reindent_after_dedent"), (Ok(2), Ok(2)));
}

#[test]
fn lex_indentation_block_at_eof() {
    assert_eq!(layout("block_at_eof"), (Ok(1), Ok(1)));
}

#[test]
fn lex_indentation_deep_nesting() {
    assert_eq!(layout("deep_nesting"), (Ok(4), Ok(4)));
}
