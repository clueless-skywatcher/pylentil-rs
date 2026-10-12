use pylentil_ast::ast::PyComparisonOp;
use pylentil_lint_macros::create_pylentil_lint;

use crate::utils::is_none;

create_pylentil_lint! {
    NoneComparison,
    "PYC-E711",
    pycodestyle,
    correctness,
    warning,
    "Comparison to `None` should use `is` or `is not`, not `==` or `!=`",
    "Use `is` instead of `==`, or `is not` instead of `!=`"
}

impl Lint for NoneComparison {
    fn check_expr(&self, path: &Path, expr: &PyExpr) -> Vec<LintViolation> {
        let mut violations = vec![];
        let PyExpr::Compare(compare) = expr else {
            return violations;
        };

        let mut previous = compare.left.as_ref();
        for (op, comparator) in compare.ops.iter().zip(&compare.comparators) {
            if matches!(op, PyComparisonOp::Eq | PyComparisonOp::NotEq)
                && (is_none(previous) || is_none(comparator))
            {
                let span = if is_none(comparator) {
                    comparator.span()
                } else {
                    previous.span()
                };
                self.report(&mut violations, path, &span);
            }
            previous = comparator;
        }

        violations
    }
}
