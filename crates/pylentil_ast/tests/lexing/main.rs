#[path = "../common/mod.rs"]
mod common;

use common::{case, content, count_kind, kinds, lex_outcome, Outcome};
use pylentil_ast::PyTokenType::{Assign, Dedent, Ident, Indent, Int, Newline, EOF};
use pylentil_common::errors::PylentilError;

trait OutcomeExt {
    fn is_accepted(&self) -> bool;
    fn error(&self) -> Option<&PylentilError>;
}

impl<T> OutcomeExt for Outcome<T> {
    fn is_accepted(&self) -> bool {
        matches!(self, Outcome::Ok(_))
    }

    fn error(&self) -> Option<&PylentilError> {
        match self {
            Outcome::Err(e) => Some(e),
            _ => None,
        }
    }
}

/// Lex one named case; fail with the reason if it errors or panics.
fn assert_lexes(fixture: &str, name: &str) {
    let code = case(fixture, name);
    match lex_outcome(&code) {
        Outcome::Ok(_) => {}
        Outcome::Err(e) => panic!("rejected `{}`: {e:?}", code.trim()),
        Outcome::Panic(m) => panic!("panicked on `{}`: {m}", code.trim()),
    }
}

/// Lex one named case; fail unless the lexer returns an error.
fn assert_lex_rejected(fixture: &str, name: &str) {
    let code = case(fixture, name);
    match lex_outcome(&code) {
        Outcome::Err(_) => {}
        Outcome::Ok(t) => panic!("accepted invalid input `{}` as {t:?}", code.trim()),
        Outcome::Panic(m) => panic!("panicked instead of erroring on `{}`: {m}", code.trim()),
    }
}

/// Lex one named case; accept or reject, but never panic.
fn assert_lex_no_panic(fixture: &str, name: &str) {
    let code = case(fixture, name);
    if let Outcome::Panic(m) = lex_outcome(&code) {
        panic!("panicked on `{}`: {m}", code.trim());
    }
}

mod basics;
mod numbers;
mod strings;
mod operators;
mod identifiers;
mod comments;
mod indentation;
mod whitespace;
mod robustness;
