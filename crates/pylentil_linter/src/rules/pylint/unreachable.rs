use pylentil_lint_macros::create_pylentil_lint;

use crate::utils::is_terminator;

create_pylentil_lint! {
    Unreachable,
    "PYL-W0101",
    pylint,
    restriction,
    warning,
    "Code after return/break/continue/raise statement cannot be reached",
    "Remove the unreachable lines, or shift the terminating statement to the end of the block"
}

impl Lint for Unreachable {
    fn check(&self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        self.collect(&mut violations, path, stmt);
        violations
    }
}

impl Unreachable {
    fn collect(&self, violations: &mut Vec<LintViolation>, path: &Path, stmt: &PyStatement) {
        if let PyStatement::Block(block) = stmt {
            if let Some(index) = block.body.iter().rposition(is_terminator) {
                for unreachable in &block.body[index + 1..] {
                    self.report(violations, path, &unreachable.span());
                }
            }
        }
    }
}
