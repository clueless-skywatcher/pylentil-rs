use std::path::{Path};

use pylentil_ast::ast::{PyExceptHandler, PyStatement};

use crate::{
    lint::{Lint, LintCategory, LintSeverity},
    violation::LintViolation,
};

#[derive(Clone)]
pub struct BareExcept;

impl Lint for BareExcept {
    fn code(&self) -> &'static str {
        "PYC-E722"
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }

    fn message(&self) -> String {
        "Bare except is a catch-all for all exceptions, which can cause problems in debugging and handling special cases.".to_string()
    }

    fn possible_fix(&self) -> Option<String> {
        Some("Specify the type of exception to catch".to_string())
    }

    fn check(&mut self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        match stmt {
            PyStatement::Try { handlers, .. } => {
                for handler in handlers {
                    match handler {
                        PyExceptHandler {
                            type_: None,
                            name: None,
                            span,
                            ..
                        } => {
                            self.report(&mut violations, path, span)
                        },
                        _ => {}
                    }
                }

                violations
            }
            _ => violations,
        }
    }

    fn severity(&self) -> LintSeverity {
        LintSeverity::Warning
    }
}
