use std::path::Path;

use pylentil_ast::ast::{PyExpr, PyStatement};

use crate::{
    lint::{Lint, LintCategory, LintSeverity, LintSource},
    violation::LintViolation,
};

#[derive(Debug, Clone)]
pub struct BinaryOpException;

impl Lint for BinaryOpException {
    fn code(&self) -> &'static str {
        "PYL-W0711"
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }

    fn message(&self) -> String {
        "Binary operation involving multiple exception types detected 
            when attempting to catch an exception"
            .to_string()
    }

    fn severity(&self) -> LintSeverity {
        LintSeverity::Error
    }

    fn source(&self) -> LintSource {
        LintSource::Pylint
    }

    fn possible_fix(&self) -> Option<String> {
        Some(
            "If intending to catch multiple exceptions, 
            use except (X, Y) instead"
                .to_string(),
        )
    }

    fn check(&mut self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        match stmt {
            PyStatement::Try { handlers, .. } => {
                for handler in handlers.iter() {
                    let Some(expr) = &handler.type_ else {
                        continue;
                    };

                    match **expr {
                        PyExpr::BoolOp {
                            span: comp_span, ..
                        } => {
                            self.report(&mut violations, path, &comp_span);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        violations
    }
}
