//! Token positions. `==` on tokens ignores spans, so these compare offsets.

use super::*;
use pylentil_ast::PyTokenType::{self, Assign, Dedent, EOF, Ident, Indent, Int};

/// `(kind, start, end)` for every token except whitespace.
fn spans(code: &str) -> Vec<(PyTokenType, usize, Option<usize>)> {
    common::lex(code)
        .unwrap_or_else(|e| panic!("failed to lex `{}`: {e:?}", code.trim()))
        .into_iter()
        .filter(|t| t.kind != PyTokenType::Whitespace)
        .map(|t| (t.kind, t.span.start, t.span.end))
        .collect()
}

#[test]
fn lex_spans_simple_line() {
    p_assert_eq!(
        spans("x = 12"),
        vec![
            (Ident, 0, Some(1)),
            (Assign, 2, Some(3)),
            (Int, 4, Some(6)),
            (EOF, 6, None),
        ]
    );
}

#[test]
fn lex_spans_prefixed_string_starts_at_prefix() {
    p_assert_eq!(spans("rb'hi'")[0], (PyTokenType::String, 0, Some(6)));
}

#[test]
fn lex_spans_indent_and_dedent() {
    // `    y` is at 6..11 and `z` at 12.
    let tokens = spans("if x:\n    y\nz\n");

    let indent = tokens.iter().find(|t| t.0 == Indent).unwrap();
    p_assert_eq!(*indent, (Indent, 6, Some(10)));

    let dedent = tokens.iter().find(|t| t.0 == Dedent).unwrap();
    p_assert_eq!(*dedent, (Dedent, 12, None));
}
