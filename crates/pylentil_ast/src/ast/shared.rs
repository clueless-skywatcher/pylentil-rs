use pylentil_common::span::PySpan;

use super::expr::{PyExpr, PyExprBox};
use super::pattern::PyPatternBox;
use super::stmt::PyStatement;

/// Keyword argument in a call (`arg=value` or `**value` when `arg` is `None`).
/// The span covers the whole entry, including `**`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyKeyword {
    pub arg: Option<String>,
    pub value: PyExprBox,
    pub span: PySpan,
}

/// One `for target in iter [if ...]` clause, from `for` to its last condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyComprehension {
    pub target: PyExprBox,
    pub iter: PyExprBox,
    pub ifs: Vec<PyExpr>,
    pub is_async: bool,
    pub span: PySpan,
}

/// `name [as asname]` in an import. The span covers the `as` part too.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAlias {
    pub name: String,
    pub asname: Option<String>,
    pub span: PySpan,
}

/// A parameter as written: any `*`, the name and the annotation, but not a
/// default value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyArg {
    pub arg: PyExprBox,
    pub annotation: Option<PyExprBox>,
    pub type_comment: Option<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct PyArguments {
    pub posonlyargs: Vec<PyArg>,
    pub args: Vec<PyArg>,
    pub vararg: Option<PyArg>,
    pub kwonlyargs: Vec<PyArg>,
    pub kw_defaults: Vec<Option<PyExpr>>,
    pub kwarg: Option<PyArg>,
    pub defaults: Vec<Option<PyExpr>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyWithItem {
    pub context_expr: PyExprBox,
    pub optional_vars: Option<PyExprBox>,
    pub span: PySpan,
}

/// An `except` clause, from the keyword to the end of its body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyExceptHandler {
    pub type_: Option<PyExprBox>,
    pub name: Option<String>,
    pub body: Vec<PyStatement>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyMatchCase {
    pub pattern: PyPatternBox,
    pub guard: Option<PyExprBox>,
    pub body: Vec<PyStatement>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyTypeIgnore {
    pub lineno: i32,
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PyTypeParam {
    TypeVar {
        name: String,
        bound: Option<PyExprBox>,
        span: PySpan,
    },
    ParamSpec {
        name: String,
        span: PySpan,
    },
    TypeVarTuple {
        name: String,
        span: PySpan,
    },
}

impl PyTypeParam {
    pub fn span(&self) -> PySpan {
        match self {
            PyTypeParam::TypeVar { span, .. }
            | PyTypeParam::ParamSpec { span, .. }
            | PyTypeParam::TypeVarTuple { span, .. } => *span,
        }
    }
}
