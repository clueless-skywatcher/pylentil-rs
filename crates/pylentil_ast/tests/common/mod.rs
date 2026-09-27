//! Shared helpers for the `lexing` and `parsing` test binaries.
//!
//! Fixtures live in `<workspace>/fixtures/{lexer,parser}/*.py`, each holding many
//! snippets introduced by a `# case: <name>` line. Headers are stripped before a
//! snippet reaches the lexer, which is what lets fixtures carry documentation
//! even for syntax that is not lexable yet.
//!
//! Parser tests compare real AST values; [`build`] has the constructors for
//! the expected side.

#![allow(dead_code)]

use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;

pub mod build;

use pylentil_ast::ast::{PyModule, PyStatement};
use pylentil_ast::parser::PyParser;
use pylentil_ast::{PyLexer, PyToken, PyTokenType};
use pylentil_common::errors::PylentilError;

// ---------------------------------------------------------------- fixtures --

pub struct Case {
    pub name: String,
    pub code: String,
}

pub fn fixture_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(rel)
}

pub fn cases(rel: &str) -> Vec<Case> {
    let path = fixture_path(rel);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read fixture {}: {e}", path.display()));

    let mut out: Vec<Case> = Vec::new();
    let mut current: Option<(String, String)> = None;

    for line in text.lines() {
        match line.strip_prefix("# case:") {
            Some(name) => {
                if let Some((name, body)) = current.take() {
                    out.push(Case { name, code: trim_blank_edges(&body) });
                }
                current = Some((name.trim().to_string(), String::new()));
            }
            None => {
                if let Some((_, body)) = current.as_mut() {
                    body.push_str(line);
                    body.push('\n');
                }
            }
        }
    }
    if let Some((name, body)) = current.take() {
        out.push(Case { name, code: trim_blank_edges(&body) });
    }

    assert!(!out.is_empty(), "fixture {rel} contains no `# case:` blocks");
    out
}

/// One named case. Panics on a typo so a test cannot silently assert against
/// an empty snippet.
pub fn case(rel: &str, name: &str) -> String {
    let all = cases(rel);
    match all.iter().find(|c| c.name == name) {
        Some(c) => c.code.clone(),
        None => panic!(
            "fixture {rel} has no case `{name}`; it has: {}",
            all.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")
        ),
    }
}

fn trim_blank_edges(body: &str) -> String {
    let lines: Vec<&str> = body.lines().collect();
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
    let end = lines.iter().rposition(|l| !l.trim().is_empty()).map(|i| i + 1).unwrap_or(0);
    let mut out = lines[start..end].join("\n");
    out.push('\n');
    out
}

// ------------------------------------------------------------------ lexing --

pub fn lex(code: &str) -> Result<Vec<PyToken<'_>>, PylentilError> {
    let code = PyLexer::from_code(code);
    code.map(|lexer| lexer.tokens)
}

/// Token kinds, minus the insignificant `Whitespace` filler.
pub fn kinds(code: &str) -> Result<Vec<PyTokenType>, PylentilError> {
    Ok(lex(code)?
        .into_iter()
        .filter(|t| t.kind != PyTokenType::Whitespace)
        .map(|t| t.kind)
        .collect())
}

/// Only tokens carrying content: no whitespace, layout or end marker.
pub fn content(code: &str) -> Result<Vec<String>, PylentilError> {
    Ok(lex(code)?
        .iter()
        .filter(|t| {
            !matches!(
                t.kind,
                PyTokenType::Whitespace
                    | PyTokenType::Newline
                    | PyTokenType::Indent
                    | PyTokenType::Dedent
                    | PyTokenType::EOF
            )
        })
        .map(describe)
        .collect())
}

pub fn describe(token: &PyToken) -> String {
    match &token.value {
        Some(value) => format!("{:?}({})", token.kind, value),
        None => format!("{:?}", token.kind),
    }
}

pub fn count_kind(code: &str, kind: PyTokenType) -> Result<usize, PylentilError> {
    Ok(kinds(code)?.iter().filter(|k| **k == kind).count())
}

// ----------------------------------------------------------------- parsing --

pub fn parse_module(code: &str) -> Result<PyModule, PylentilError> {
    let tokens = lex(code)?;
    let mut parser = PyParser::new(tokens);
    parser.parse()
}

// ------------------------------------------------------- outcome capturing --

/// A result that also survives a panic, so "this input is rejected" can be
/// asserted even where the implementation reaches an `assert!` or `unwrap`.
#[derive(Debug)]
pub enum Outcome<T> {
    Ok(T),
    Err(PylentilError),
    Panic(String),
}

pub fn catching<T>(f: impl FnOnce() -> Result<T, PylentilError>) -> Outcome<T> {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(value)) => Outcome::Ok(value),
        Ok(Err(e)) => Outcome::Err(e),
        Err(payload) => {
            let message = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic>".to_string());
            Outcome::Panic(message)
        }
    }
}

pub fn lex_outcome(code: &str) -> Outcome<Vec<PyTokenType>> {
    catching(|| kinds(code))
}

pub fn parse_outcome(code: &str) -> Outcome<Vec<PyStatement>> {
    catching(|| parse_module(code).map(|module| module.body))
}
