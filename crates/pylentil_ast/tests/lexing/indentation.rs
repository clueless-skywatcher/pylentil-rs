use super::*;

fn layout(name: &str) -> (Result<usize, PylentilError>, Result<usize, PylentilError>) {
    let code = case("lexer/indentation.py", name);
    (count_kind(&code, Indent), count_kind(&code, Dedent))
}

#[test]
fn one_block_opens_and_closes_once() {
    assert_eq!(layout("single_level"), (Ok(1), Ok(1)));
}

#[test]
fn nested_blocks_nest_their_layout_tokens() {
    assert_eq!(layout("two_levels"), (Ok(2), Ok(2)));
}

#[test]
fn dedenting_one_level_closes_one_block() {
    assert_eq!(layout("dedent_one_level"), (Ok(2), Ok(2)));
}

#[test]
fn dedenting_to_module_level_closes_every_block() {
    assert_eq!(layout("dedent_to_module_level"), (Ok(2), Ok(2)));
}

#[test]
fn a_tab_opens_a_block_just_like_spaces() {
    assert_eq!(layout("tab_indent"), (Ok(1), Ok(1)));
}

#[test]
fn indent_width_does_not_matter() {
    assert_eq!(layout("eight_space_indent"), (Ok(1), Ok(1)));
}

#[test]
fn a_block_left_open_at_eof_is_closed() {
    assert_eq!(layout("block_at_eof"), (Ok(1), Ok(1)));
}

#[test]
fn deep_nesting_stays_balanced() {
    assert_eq!(layout("deep_nesting"), (Ok(4), Ok(4)));
}

#[test]
fn a_blank_line_does_not_close_a_block() {
    assert_eq!(layout("blank_line_inside_block"), (Ok(1), Ok(1)));
}

#[test]
fn consecutive_blocks_each_open_and_close() {
    assert_eq!(layout("reindent_after_dedent"), (Ok(2), Ok(2)));
    assert_eq!(layout("blank_line_between_blocks"), (Ok(2), Ok(2)));
}

#[test]
fn every_fixture_balances_indents_against_dedents() {
    let mut unbalanced = Vec::new();
    for c in cases("lexer/indentation.py") {
        match (count_kind(&c.code, Indent), count_kind(&c.code, Dedent)) {
            (Ok(i), Ok(d)) if i == d => {}
            (Ok(i), Ok(d)) => unbalanced.push(format!("  {}: {i} indents, {d} dedents", c.name)),
            _ => {} // rejection is covered by the dedicated tests below
        }
    }
    assert!(unbalanced.is_empty(), "layout tokens must balance:\n{}", unbalanced.join("\n"));
}

#[test]
fn an_indented_first_line_is_an_error() {
    // Python: IndentationError: unexpected indent
    assert!(
        matches!(lex_outcome(&case("lexer/indentation.py", "first_line_indented")), Outcome::Err(_)),
        "module-level code cannot start indented"
    );
}

#[test]
fn an_inconsistent_dedent_is_an_error_not_a_panic() {
    // Python: IndentationError: unindent does not match any outer level
    match lex_outcome(&case("lexer/indentation.py", "inconsistent_dedent")) {
        Outcome::Err(_) => {}
        Outcome::Ok(t) => panic!("accepted an inconsistent dedent: {t:?}"),
        Outcome::Panic(m) => panic!("panicked instead of erroring: {m}"),
    }
}

#[test]
fn a_dedent_to_an_unknown_level_is_an_error_not_a_panic() {
    match lex_outcome(&case("lexer/indentation.py", "dedent_to_unknown_level")) {
        Outcome::Err(_) => {}
        Outcome::Ok(t) => panic!("accepted a dedent to an unvisited level: {t:?}"),
        Outcome::Panic(m) => panic!("panicked instead of erroring: {m}"),
    }
}
