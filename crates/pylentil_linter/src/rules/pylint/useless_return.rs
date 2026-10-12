use pylentil_ast::ast::PyConstant;
use pylentil_lint_macros::create_pylentil_lint;

create_pylentil_lint! {
    UselessReturn,
    "PYL-R1711",
    pylint,
    style,
    nitpick,
    "Return statement at end of function not returning anything or returning None",
    "Remove the return statement"
}

impl Lint for UselessReturn {
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
