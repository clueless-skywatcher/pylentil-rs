use pylentil_ast::ast::{PyExceptHandler, PyStatement};
use pylentil_lint_macros::create_pylentil_lint;

create_pylentil_lint! {
    BareExcept,
    "PYC-E722",
    pycodestyle,
    correctness,
    warning,
    "Bare except is a catch-all for all exceptions, which can cause problems in debugging and handling special cases.",
    "Specify the type of exception to catch"
}

impl Lint for BareExcept {
    fn check(&self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        self.collect(&mut violations, path, stmt);
        violations
    }
}

impl BareExcept {
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
            if handler.type_.is_none() && handler.name.is_none() {
                self.report(violations, path, &handler.span);
            }
        }
    }
}
