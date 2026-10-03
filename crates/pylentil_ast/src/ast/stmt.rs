use pylentil_common::span::PySpan;

use crate::ast::PyArg;
use crate::common::PyArgType;

use super::expr::{PyExpr, PyExprBox};
use super::ops::PyBinaryOp;
use super::shared::{
    PyAlias, PyArguments, PyExceptHandler, PyKeyword, PyMatchCase, PyTypeParam, PyWithItem,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PyStatement {
    FunctionDef {
        name: String,
        args: Box<PyArguments>,
        body: Vec<PyStatement>,
        decorator_list: Vec<PyExpr>,
        returns: Option<PyExprBox>,
        type_comment: Option<String>,
        type_params: Vec<PyTypeParam>,
        is_async: bool,
        span: PySpan,
    },
    ClassDef {
        name: String,
        bases: Vec<PyArgType>,
        keywords: Vec<PyKeyword>,
        body: Vec<PyStatement>,
        decorator_list: Vec<PyExpr>,
        type_params: Vec<PyTypeParam>,
        span: PySpan,
    },
    Return {
        value: Option<PyExprBox>,
        span: PySpan,
    },
    Delete {
        targets: Vec<PyExpr>,
        span: PySpan,
    },
    Assign {
        targets: Vec<PyExpr>,
        value: PyExprBox,
        type_comment: Option<String>,
        span: PySpan,
    },
    TypeAlias {
        name: PyExprBox,
        type_params: Vec<PyTypeParam>,
        value: PyExprBox,
        span: PySpan,
    },
    AugAssign {
        target: PyExprBox,
        op: PyBinaryOp,
        value: PyExprBox,
        span: PySpan,
    },
    AnnAssign {
        target: PyExprBox,
        annotation: PyExprBox,
        value: Option<PyExprBox>,
        simple: bool,
        span: PySpan,
    },
    For {
        target: PyExprBox,
        iter: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        type_comment: Option<String>,
        span: PySpan,
    },
    AsyncFor {
        target: PyExprBox,
        iter: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        type_comment: Option<String>,
        span: PySpan,
    },
    While {
        test: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        span: PySpan,
    },
    If {
        test: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        span: PySpan,
    },
    With {
        items: Vec<PyWithItem>,
        body: Vec<PyStatement>,
        type_comment: Option<String>,
        span: PySpan,
    },
    AsyncWith {
        items: Vec<PyWithItem>,
        body: Vec<PyStatement>,
        type_comment: Option<String>,
        span: PySpan,
    },
    Match {
        subject: PyExprBox,
        cases: Vec<PyMatchCase>,
        span: PySpan,
    },
    Raise {
        exc: Option<PyExprBox>,
        cause: Option<PyExprBox>,
        span: PySpan,
    },
    Try {
        body: Vec<PyStatement>,
        handlers: Vec<PyExceptHandler>,
        orelse: Vec<PyStatement>,
        finalbody: Vec<PyStatement>,
        span: PySpan,
    },
    TryStar {
        body: Vec<PyStatement>,
        handlers: Vec<PyExceptHandler>,
        orelse: Vec<PyStatement>,
        finalbody: Vec<PyStatement>,
        span: PySpan,
    },
    Assert {
        test: PyExprBox,
        msg: Option<PyExprBox>,
        span: PySpan,
    },
    Import {
        names: Vec<PyAlias>,
        span: PySpan,
    },
    ImportFrom {
        module: Option<String>,
        names: Vec<PyAlias>,
        level: Option<i32>,
        span: PySpan,
    },
    Global {
        names: Vec<String>,
        span: PySpan,
    },
    Nonlocal {
        names: Vec<String>,
        span: PySpan,
    },
    Expr {
        value: PyExprBox,
        span: PySpan,
    },
    Pass {
        span: PySpan,
    },
    Break {
        span: PySpan,
    },
    Continue {
        span: PySpan,
    },
}

impl PyStatement {
    /// Where the statement sits in the source, from its first token to the
    /// end of its last one. A compound statement ends with its last body
    /// statement.
    pub fn span(&self) -> PySpan {
        match self {
            PyStatement::FunctionDef { span, .. }
            | PyStatement::ClassDef { span, .. }
            | PyStatement::Return { span, .. }
            | PyStatement::Delete { span, .. }
            | PyStatement::Assign { span, .. }
            | PyStatement::TypeAlias { span, .. }
            | PyStatement::AugAssign { span, .. }
            | PyStatement::AnnAssign { span, .. }
            | PyStatement::For { span, .. }
            | PyStatement::AsyncFor { span, .. }
            | PyStatement::While { span, .. }
            | PyStatement::If { span, .. }
            | PyStatement::With { span, .. }
            | PyStatement::AsyncWith { span, .. }
            | PyStatement::Match { span, .. }
            | PyStatement::Raise { span, .. }
            | PyStatement::Try { span, .. }
            | PyStatement::TryStar { span, .. }
            | PyStatement::Assert { span, .. }
            | PyStatement::Import { span, .. }
            | PyStatement::ImportFrom { span, .. }
            | PyStatement::Global { span, .. }
            | PyStatement::Nonlocal { span, .. }
            | PyStatement::Expr { span, .. }
            | PyStatement::Pass { span }
            | PyStatement::Break { span }
            | PyStatement::Continue { span } => *span,
        }
    }
}
