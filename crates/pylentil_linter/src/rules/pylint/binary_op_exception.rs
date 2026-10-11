use std::path::Path;

use pylentil_ast::ast::{PyExceptHandler, PyExpr, PyStatement};

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

    fn check(&self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        self.collect(&mut violations, path, stmt);
        violations
    }
}

impl BinaryOpException {
    fn collect(&self, violations: &mut Vec<LintViolation>, path: &Path, stmt: &PyStatement) {
        match stmt {
            PyStatement::Try(stmt) => self.check_handlers(violations, path, &stmt.handlers),
            PyStatement::TryStar(stmt) => self.check_handlers(violations, path, &stmt.handlers),
            _ => {}
        }
    }

    fn check_handlers(
        &self,
        violations: &mut Vec<LintViolation>,
        path: &Path,
        handlers: &[PyExceptHandler],
    ) {
        for handler in handlers {
            let Some(expr) = &handler.type_ else {
                continue;
            };

            if let PyExpr::BoolOp(expr) = expr.as_ref() {
                self.report(violations, path, &expr.span);
            }
        }
    }
}
