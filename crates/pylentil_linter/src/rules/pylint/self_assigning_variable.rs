use std::path::Path;

use pylentil_ast::ast::{PyExpr, PyStatement};
use pylentil_common::span::PySpan;

use crate::{
    lint::{Lint, LintCategory, LintSeverity, LintSource},
    violation::LintViolation,
};

#[derive(Debug, Clone)]
pub struct SelfAssigningVariable;

impl Lint for SelfAssigningVariable {
    fn code(&self) -> &'static str {
        "PYL-W0127"
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }

    fn message(&self) -> String {
        "Variable redundantly assigned to itself".to_string()
    }

    fn severity(&self) -> LintSeverity {
        LintSeverity::Warning
    }

    fn source(&self) -> LintSource {
        LintSource::Pylint
    }

    fn possible_fix(&self) -> Option<String> {
        Some("Remove the redundant assignment".to_string())
    }

    fn check(&self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        self.collect(&mut violations, path, stmt);
        violations
    }
}

impl SelfAssigningVariable {
    fn collect(&self, violations: &mut Vec<LintViolation>, path: &Path, stmt: &PyStatement) {
        match stmt {
            PyStatement::Assign(assign) => {
                self.check_assign(
                    violations,
                    path,
                    &assign.targets,
                    &assign.value,
                    &assign.span,
                );
            }
            PyStatement::AnnAssign(assign) => {
                if let Some(value) = &assign.value {
                    self.check_assign(
                        violations,
                        path,
                        std::slice::from_ref(assign.target.as_ref()),
                        value,
                        &assign.span,
                    );
                }
            }
            _ => {}
        }
    }

    fn check_assign(
        &self,
        violations: &mut Vec<LintViolation>,
        path: &Path,
        targets: &[PyExpr],
        value: &PyExpr,
        span: &PySpan,
    ) {
        if let [target] = targets {
            if same_expr(target, value) {
                self.report(violations, path, span);
            }
        }
    }
}

fn same_elts(left: &[PyExpr], right: &[PyExpr]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| same_expr(left, right))
}

fn same_expr(left: &PyExpr, right: &PyExpr) -> bool {
    match (left, right) {
        (PyExpr::Name(left), PyExpr::Name(right)) => left.id == right.id,
        (PyExpr::Attribute(left), PyExpr::Attribute(right)) => {
            left.attr == right.attr && same_expr(&left.value, &right.value)
        }
        (PyExpr::Subscript(left), PyExpr::Subscript(right)) => {
            same_expr(&left.value, &right.value) && same_expr(&left.slice, &right.slice)
        }
        (PyExpr::Tuple(left), PyExpr::Tuple(right)) => same_elts(&left.elts, &right.elts),
        (PyExpr::List(left), PyExpr::List(right)) => same_elts(&left.elts, &right.elts),
        (PyExpr::Starred(left), PyExpr::Starred(right)) => same_expr(&left.value, &right.value),
        _ => false,
    }
}
