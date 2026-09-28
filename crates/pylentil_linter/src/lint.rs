use std::{hash::{Hash, Hasher}, path::{Path, PathBuf}};

use pylentil_ast::ast::PyStatement;

use crate::violation::LintViolation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintCategory {
    Pedantic,
    Correctness,
    Security,
    Complexity,
    Style,
    Formatting,
    Restriction
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintSeverity {
    Warning,
    Error,
    Nitpick
}

pub trait Lint: Send + Sync {
    fn code(&self) -> String;
    fn category(&self) -> LintCategory;
    fn message(&self) -> String;
    fn severity(&self) -> LintSeverity;

    // Not every lint has to have a possible fix. Override when necessary
    fn possible_fix(&self) -> Option<String> {
        None
    }

    fn check(&mut self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation>;

    fn report(&self, violations: &mut Vec<LintViolation>, path: &Path)
    where 
        Self: Sized + Clone + 'static
    {
        violations.push(LintViolation { path: path.to_path_buf(), check_violated: Box::new(self.clone()) });
    }
}

impl PartialEq for dyn Lint {
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

impl Eq for dyn Lint {}

impl Hash for dyn Lint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code().hash(state);
    }
}


