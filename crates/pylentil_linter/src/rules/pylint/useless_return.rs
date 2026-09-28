use std::path::{Path, PathBuf};

use pylentil_ast::ast::{PyConstant, PyExpr, PyStatement};

use crate::{lint::{Lint, LintCategory, LintSeverity}, violation::LintViolation};

#[derive(Clone)]
pub struct UselessReturn;

impl Lint for UselessReturn {
    fn code(&self) -> &'static str {
        "PYL-R1711"
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
        let mut violations: Vec<LintViolation> = vec![];
        
        match stmt {
            PyStatement::FunctionDef { body, .. } => {
                if body.len() > 1 {
                    match body.last().unwrap() {
                        PyStatement::Return { value, .. } => {
                            match value {
                                Some(value) => {
                                    if matches!(**value, PyExpr::Constant { value: PyConstant::None, .. }) {
                                        self.report(&mut violations, path);
                                    }
                                }
                                None => self.report(&mut violations, path),
                            }
                        }, 
                        _ => {}
                    }
                }
            },
            _ => {}
        }
        violations
    }
}