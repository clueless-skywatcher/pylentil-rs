use std::path::{Path, PathBuf};

use pylentil_ast::ast::PyStatement;

use crate::{lint::{Lint, LintCategory, LintSeverity}, violation::LintViolation};

#[derive(Clone)]
pub struct UselessReturn;

impl Lint for UselessReturn {
    fn code(&self) -> String {
        "PYL-R1711".to_string()
    }

    fn category(&self) -> crate::lint::LintCategory {
        LintCategory::Style
    }

    fn message(&self) -> String {
        "Return statement at end of function not returning anything or returning None".to_string()
    }

    fn severity(&self) -> crate::lint::LintSeverity {
        LintSeverity::Nitpick
    }

    fn possible_fix(&self) -> Option<String> {
        Some("Remove the return statement".to_string())
    }

    fn check(&mut self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        match stmt {
            PyStatement::FunctionDef { body, returns, .. } => {
                
            },
            _ => {}
        }
        vec![]
    }
}