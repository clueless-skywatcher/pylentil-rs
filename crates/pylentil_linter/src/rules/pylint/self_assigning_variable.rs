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

    fn check(&mut self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        match stmt {
            PyStatement::Assign {
                targets,
                value,
                span,
                ..
            } => {
                self.check_assign(&mut violations, path, targets, value, span);
            }
            PyStatement::AnnAssign {
                target,
                value: Some(value),
                span,
                ..
            } => {
                self.check_assign(
                    &mut violations,
                    path,
                    std::slice::from_ref(target),
                    value,
                    span,
                );
            }
            _ => {}
        }

        violations
    }
}

impl SelfAssigningVariable {
    fn check_assign(
        &mut self,
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

fn same_expr(left: &PyExpr, right: &PyExpr) -> bool {
    match (left, right) {
        (PyExpr::Name { id: left_id, .. }, PyExpr::Name { id: right_id, .. }) => {
            left_id == right_id
        }
        (
            PyExpr::Attribute {
                value: left_value,
                attr: left_attr,
                ..
            },
            PyExpr::Attribute {
                value: right_value,
                attr: right_attr,
                ..
            },
        ) => left_attr == right_attr && same_expr(left_value, right_value),
        (
            PyExpr::Subscript {
                value: left_value,
                slice: left_slice,
                ..
            },
            PyExpr::Subscript {
                value: right_value,
                slice: right_slice,
                ..
            },
        ) => same_expr(left_value, right_value) && same_expr(left_slice, right_slice),
        (
            PyExpr::Tuple {
                elts: left_elts, ..
            },
            PyExpr::Tuple {
                elts: right_elts, ..
            },
        )
        | (
            PyExpr::List {
                elts: left_elts, ..
            },
            PyExpr::List {
                elts: right_elts, ..
            },
        ) => {
            left_elts.len() == right_elts.len()
                && left_elts
                    .iter()
                    .zip(right_elts)
                    .all(|(left, right)| same_expr(left, right))
        }
        (
            PyExpr::Starred {
                value: left_value, ..
            },
            PyExpr::Starred {
                value: right_value, ..
            },
        ) => same_expr(left_value, right_value),
        _ => false,
    }
}
