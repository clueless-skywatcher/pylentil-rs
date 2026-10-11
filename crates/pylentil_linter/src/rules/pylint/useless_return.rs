use std::path::Path;

use pylentil_ast::ast::{PyConstant, PyExpr, PyStatement};

use crate::{
    lint::{Lint, LintCategory, LintSeverity, LintSource},
    violation::LintViolation,
};

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

    fn source(&self) -> LintSource {
        LintSource::Pylint
    }

    fn check(&self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        self.collect(&mut violations, path, stmt);
        violations
    }
}

impl UselessReturn {
    fn collect(&self, violations: &mut Vec<LintViolation>, path: &Path, stmt: &PyStatement) {
        if let PyStatement::FunctionDef(func) = stmt {
            if let PyStatement::Block(body) = &func.body {
                if body.body.len() > 1 {
                    if let Some(PyStatement::Return(ret)) = body.body.last() {
                        if is_useless(&ret.value) {
                            self.report(violations, path, &ret.span);
                        }
                    }
                }
            }
        }
    }
}

fn is_useless(value: &Option<Box<PyExpr>>) -> bool {
    match value {
        None => true,
        Some(value) => matches!(
            value.as_ref(),
            PyExpr::Constant(constant) if constant.value == PyConstant::None
        ),
    }
}
