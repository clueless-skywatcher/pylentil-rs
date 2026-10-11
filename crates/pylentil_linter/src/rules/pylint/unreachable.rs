use std::path::Path;

use pylentil_ast::ast::PyStatement;

use crate::{
    lint::{Lint, LintCategory, LintSeverity, LintSource},
    violation::LintViolation,
};

#[derive(Clone)]
pub struct Unreachable;

impl Lint for Unreachable {
    fn code(&self) -> &'static str {
        "PYL-W0101"
    }

    fn category(&self) -> crate::lint::LintCategory {
        LintCategory::Restriction
    }

    fn message(&self) -> String {
        "Code after return/break/continue/raise statement cannot be reached".to_string()
    }

    fn severity(&self) -> crate::lint::LintSeverity {
        LintSeverity::Warning
    }

    fn source(&self) -> crate::lint::LintSource {
        LintSource::Pylint
    }

    fn possible_fix(&self) -> Option<String> {
        Some(
            "Remove the unreachable lines, or shift the 
            terminating statement to the end of the block"
                .to_string(),
        )
    }

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

fn is_terminator(stmt: &PyStatement) -> bool {
    matches!(
        stmt,
        PyStatement::Raise(_)
            | PyStatement::Return(_)
            | PyStatement::Continue(_)
            | PyStatement::Break(_)
    )
}
