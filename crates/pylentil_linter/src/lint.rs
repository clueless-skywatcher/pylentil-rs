use pylentil_ast::ast::{PyModule, PyStatement};

use crate::violation::{LintViolation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintCategory {
    Pedantic,
}

pub trait Lint: Send + Sync {
    fn code(&self) -> String;
    fn category(&self) -> LintCategory;
    fn message(&self) -> String;

    // Not every lint has to have a possible fix. Override when necessary
    fn possible_fix(&self) -> Option<String> {
        None
    }

    // Return a violation if found
    fn check(&self) -> Option<LintViolation>;
}


