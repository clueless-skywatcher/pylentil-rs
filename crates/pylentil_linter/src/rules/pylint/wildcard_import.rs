use std::path::Path;

use pylentil_ast::ast::PyStatement;

use crate::{lint::{Lint, LintCategory, LintSeverity, LintSource}, violation::LintViolation};

#[derive(Debug, Clone)]
pub struct WildcardImport;

impl Lint for WildcardImport {
    fn code(&self) -> &'static str {
        "PYL-W0401"
    }

    fn category(&self) -> LintCategory {
        LintCategory::Style
    }

    fn message(&self) -> String {
        "Wildcard imports are expensive and can cause side 
            effects to trigger, which can become difficult to debug".to_string()
    }

    fn severity(&self) -> LintSeverity {
        LintSeverity::Warning
    }

    fn source(&self) -> LintSource {
        LintSource::Pylint
    }

    fn check(&mut self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        match stmt {
            PyStatement::ImportFrom { names, .. } => {
                if let [name] = names.as_slice() {
                    if name.name == "*" {
                        self.report(&mut violations, path, &name.span);
                    }
                };
            },
            _ => {}
        }

        violations
    }
}