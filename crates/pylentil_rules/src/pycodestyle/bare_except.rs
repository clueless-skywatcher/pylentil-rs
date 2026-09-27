use pylentil_ast::ast::{PyExceptHandler, PyStatement};
use pylentil_linter::{
    lint::{Lint, LintCategory, LintSeverity},
    violation::LintViolation,
};

pub struct BareExcept {}

impl Lint for BareExcept {
    fn code(&self) -> String {
        "PYC-E722".to_string()
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }

    fn message(&self) -> String {
        "Bare except is a catch-all for all exceptions, which can cause problems in debugging and handling special cases.".to_string()
    }

    fn check(&mut self, stmt: PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        match stmt {
            PyStatement::Try {
                handlers,
                ..
            } => {
                for handler in handlers {
                    if matches!(
                        handler,
                        PyExceptHandler {
                            type_: None,
                            name: None,
                            ..
                        }
                    ) {
                        self.report(
                            &mut violations,
                            LintViolation {
                                check_violated: Box::new(Self {}),
                            },
                        )
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
