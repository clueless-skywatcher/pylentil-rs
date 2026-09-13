use super::expr::{PyExpr, PyExprBox};
use super::pattern::PyPattern;
use super::stmt::PyStatement;

/// Keyword argument in a call (`arg=value` or `**value` when `arg` is `None`).
#[derive(Debug)]
pub struct PyKeyword {
    pub arg: Option<String>,
    pub value: PyExpr,
}

#[derive(Debug)]
pub struct PyComprehension {
    pub target: PyExpr,
    pub iter: PyExpr,
    pub ifs: Vec<PyExpr>,
    pub is_async: bool,
}

#[derive(Debug)]
pub struct PyAlias {
    pub name: String,
    pub asname: Option<String>,
}

#[derive(Debug)]
pub struct PyArg {
    pub arg: String,
    pub annotation: Option<PyExprBox>,
    pub type_comment: Option<String>,
}

#[derive(Debug)]
pub struct PyArguments {
    pub posonlyargs: Vec<PyArg>,
    pub args: Vec<PyArg>,
    pub vararg: Option<PyArg>,
    pub kwonlyargs: Vec<PyArg>,
    pub kw_defaults: Vec<Option<PyExpr>>,
    pub kwarg: Option<PyArg>,
    pub defaults: Vec<PyExpr>,
}

#[derive(Debug)]
pub struct PyWithItem {
    pub context_expr: PyExpr,
    pub optional_vars: Option<PyExpr>,
}

#[derive(Debug)]
pub struct PyExceptHandler {
    pub type_: Option<PyExpr>,
    pub name: Option<String>,
    pub body: Vec<PyStatement>,
}

#[derive(Debug)]
pub struct PyMatchCase {
    pub pattern: PyPattern,
    pub guard: Option<PyExpr>,
    pub body: Vec<PyStatement>,
}

#[derive(Debug)]
pub struct PyTypeIgnore {
    pub lineno: i32,
    pub tag: String,
}

#[derive(Debug)]
pub enum PyTypeParam {
    TypeVar {
        name: String,
        bound: Option<PyExpr>,
    },
    ParamSpec {
        name: String,
    },
    TypeVarTuple {
        name: String,
    },
}
