use std::{
    hash::{Hash, Hasher},
    path::Path,
};

use pylentil_ast::ast::{PyExpr, PyStatement};
use pylentil_common::span::PySpan;

use crate::violation::LintViolation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintCategory {
    Pedantic,
    Correctness,
    Security,
    Complexity,
    Style,
    Formatting,
    Restriction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintSeverity {
    Warning,
    Error,
    Nitpick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintSource {
    Pylint,
    Pycodestyle,
}

impl LintSource {
    pub fn get_name(&self) -> &'static str {
        match self {
            LintSource::Pylint => "pylint",
            LintSource::Pycodestyle => "pycodestyle",
        }
    }
}

pub trait LintMeta {
    const CODE: &'static str;
    const CATEGORY: LintCategory;
    const SOURCE: LintSource;
    const SEVERITY: LintSeverity;
    const MESSAGE: &'static str;
    const POSSIBLE_FIX: Option<&'static str> = None;
}

pub trait LintRuleDetails {
    fn code(&self) -> &'static str;
    fn category(&self) -> LintCategory;
    fn message(&self) -> &'static str;
    fn severity(&self) -> LintSeverity;
    fn source(&self) -> LintSource;
    fn possible_fix(&self) -> Option<&'static str>;
}

impl<T: LintMeta> LintRuleDetails for T {
    fn code(&self) -> &'static str {
        T::CODE
    }

    fn category(&self) -> LintCategory {
        T::CATEGORY
    }

    fn message(&self) -> &'static str {
        T::MESSAGE
    }

    fn severity(&self) -> LintSeverity {
        T::SEVERITY
    }

    fn source(&self) -> LintSource {
        T::SOURCE
    }

    fn possible_fix(&self) -> Option<&'static str> {
        T::POSSIBLE_FIX
    }
}

pub trait Lint: LintRuleDetails + Send + Sync {
    fn check(&self, _path: &Path, _stmt: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_expr(&self, _path: &Path, _expr: &PyExpr) -> Vec<LintViolation> {
        vec![]
    }

    fn report(&self, violations: &mut Vec<LintViolation>, path: &Path, span: &PySpan)
    where
        Self: Sized + Clone + 'static,
    {
        violations.push(LintViolation {
            path: path.to_path_buf(),
            check_violated: Box::new(self.clone()),
            span: *span,
        });
    }
}

impl PartialEq for dyn Lint {
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

impl Eq for dyn Lint {}

impl Hash for dyn Lint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code().hash(state);
    }
}
