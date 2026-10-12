use pylentil_ast::ast::{PyExceptHandler, PyExpr, PyStatement};
use pylentil_lint_macros::create_pylentil_lint;

create_pylentil_lint! {
    BinaryOpException,
    "PYL-W0711",
    pylint,
    correctness,
    error,
    "Binary operation involving multiple exception types detected when attempting to catch an exception",
    "If intending to catch multiple exceptions, use except (X, Y) instead"
}

impl Lint for BinaryOpException {
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
