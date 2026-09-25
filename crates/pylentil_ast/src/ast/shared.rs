use super::expr::{PyExpr, PyExprBox};
use super::pattern::PyPatternBox;
use super::stmt::PyStatement;

/// Keyword argument in a call (`arg=value` or `**value` when `arg` is `None`).
#[derive(Debug, Clone)]
pub struct PyKeyword {
    pub arg: Option<String>,
    pub value: PyExprBox,
}

#[derive(Debug, Clone)]
pub struct PyComprehension {
    pub target: PyExprBox,
    pub iter: PyExprBox,
    pub ifs: Vec<PyExpr>,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct PyAlias {
    pub name: String,
    pub asname: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PyArg {
    pub arg: PyExprBox,
    pub annotation: Option<PyExprBox>,
    pub type_comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PyArguments {
    pub posonlyargs: Vec<PyArg>,
    pub args: Vec<PyArg>,
    pub vararg: Option<PyArg>,
    pub kwonlyargs: Vec<PyArg>,
    pub kw_defaults: Vec<Option<PyExpr>>,
    pub kwarg: Option<PyArg>,
    pub defaults: Vec<Option<PyExpr>>,
}

#[derive(Debug, Clone)]
pub struct PyWithItem {
    pub context_expr: PyExprBox,
    pub optional_vars: Option<PyExprBox>,
}

#[derive(Debug, Clone)]
pub struct PyExceptHandler {
    pub type_: Option<PyExprBox>,
    pub name: Option<String>,
    pub body: Vec<PyStatement>,
}

#[derive(Debug, Clone)]
pub struct PyMatchCase {
    pub pattern: PyPatternBox,
    pub guard: Option<PyExprBox>,
    pub body: Vec<PyStatement>,
}

#[derive(Debug, Clone)]
pub struct PyTypeIgnore {
    pub lineno: i32,
    pub tag: String,
}

#[derive(Debug, Clone)]
pub enum PyTypeParam {
    TypeVar {
        name: String,
        bound: Option<PyExprBox>,
    },
    ParamSpec {
        name: String,
    },
    TypeVarTuple {
        name: String,
    },
}
