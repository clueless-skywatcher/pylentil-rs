#[path = "../common/mod.rs"]
mod common;

use common::{case, cases, content, count_kind, kinds, lex_failures, lex_outcome, Outcome};
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

mod basics;
mod numbers;
mod strings;
mod operators;
mod identifiers;
mod comments;
mod indentation;
mod whitespace;
mod robustness;
