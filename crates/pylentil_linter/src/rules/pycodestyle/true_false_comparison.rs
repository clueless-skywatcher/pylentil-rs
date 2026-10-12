use pylentil_ast::ast::{PyCompareExpr, PyComparisonOp};
use pylentil_lint_macros::create_pylentil_lint;

use crate::utils::{is_false, is_true};

create_pylentil_lint! {
    TrueFalseComparison,
    "PYC-E712",
    pycodestyle,
    correctness,
    warning,
    "Comparison of a variable to `True` or `False` should not be used",
    "Use the bare boolean variable, e.g. if foo: print(y)"
}

impl Lint for TrueFalseComparison {
    fn check_expr(&self, path: &Path, expr: &PyExpr) -> Vec<LintViolation> {
        let mut violations = vec![];

        let PyExpr::Compare(compare_expr) = expr else {
            return violations;
        };

        let mut left = compare_expr.left.as_ref();

        for (op, operand) in compare_expr.ops.iter().zip(&compare_expr.comparators) {
            if matches!(op, PyComparisonOp::Eq | PyComparisonOp::NotEq)
                && (is_true(&operand) || is_false(&operand) || is_true(left) || is_false(left))
            {
                let span = if is_true(operand) || is_false(operand) {
                    operand.span()
                } else {
                    left.span()
                };

                self.report(&mut violations, path, &span);
            }

            left = operand;
        }

        violations
    }
}
