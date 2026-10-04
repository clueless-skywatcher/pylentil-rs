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

    fn check_block(&mut self, path: &std::path::Path, block: &[PyStatement]) -> Vec<LintViolation> {
        let mut violations: Vec<LintViolation> = vec![];

        let mut i: usize = block.len() - 1;

        while i > 0 {
            match block[i] {
                PyStatement::Raise { .. }
                | PyStatement::Return { .. }
                | PyStatement::Continue { .. }
                | PyStatement::Break { .. } => {
                    break;
                }
                _ => {}
            }

            i -= 1;
        }

        for stmt in block[i + 1..].iter() {
            self.report(&mut violations, path, &stmt.span());
        }

        violations
    }
}
