use pylentil_common::span::PySpan;

use crate::common::PyArgType;

use super::expr::{PyExpr, PyExprBox};
use super::ops::PyBinaryOp;
use super::shared::{
    PyAlias, PyArguments, PyExceptHandler, PyKeyword, PyMatchCase, PyTypeParam, PyWithItem,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyFunctionDefStmt {
    pub name: String,
    pub args: Box<PyArguments>,
    pub body: PyStatement,
    pub decorator_list: Vec<PyExpr>,
    pub returns: Option<PyExprBox>,
    pub type_comment: Option<String>,
    pub type_params: Vec<PyTypeParam>,
    pub is_async: bool,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyClassDefStmt {
    pub name: String,
    pub bases: Vec<PyArgType>,
    pub keywords: Vec<PyKeyword>,
    pub body: PyStatement,
    pub decorator_list: Vec<PyExpr>,
    pub type_params: Vec<PyTypeParam>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyReturnStmt {
    pub value: Option<PyExprBox>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyDeleteStmt {
    pub targets: Vec<PyExpr>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAssignStmt {
    pub targets: Vec<PyExpr>,
    pub value: PyExprBox,
    pub type_comment: Option<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyTypeAliasStmt {
    pub name: PyExprBox,
    pub type_params: Vec<PyTypeParam>,
    pub value: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAugAssignStmt {
    pub target: PyExprBox,
    pub op: PyBinaryOp,
    pub value: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAnnAssignStmt {
    pub target: PyExprBox,
    pub annotation: PyExprBox,
    pub value: Option<PyExprBox>,
    pub simple: bool,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyForStmt {
    pub target: PyExprBox,
    pub iter: PyExprBox,
    pub body: PyStatement,
    pub orelse: Option<PyStatement>,
    pub type_comment: Option<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAsyncForStmt {
    pub target: PyExprBox,
    pub iter: PyExprBox,
    pub body: PyStatement,
    pub orelse: Option<PyStatement>,
    pub type_comment: Option<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyWhileStmt {
    pub test: PyExprBox,
    pub body: PyStatement,
    pub orelse: Option<PyStatement>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyIfStmt {
    pub test: PyExprBox,
    pub body: PyStatement,
    pub orelse: Option<PyStatement>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyWithStmt {
    pub items: Vec<PyWithItem>,
    pub body: PyStatement,
    pub type_comment: Option<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAsyncWithStmt {
    pub items: Vec<PyWithItem>,
    pub body: PyStatement,
    pub type_comment: Option<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyMatchStmt {
    pub subject: PyExprBox,
    pub cases: Vec<PyMatchCase>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyRaiseStmt {
    pub exc: Option<PyExprBox>,
    pub cause: Option<PyExprBox>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyTryStmt {
    pub body: PyStatement,
    pub handlers: Vec<PyExceptHandler>,
    pub orelse: Option<PyStatement>,
    pub finalbody: Option<PyStatement>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyTryStarStmt {
    pub body: PyStatement,
    pub handlers: Vec<PyExceptHandler>,
    pub orelse: Option<PyStatement>,
    pub finalbody: Option<PyStatement>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAssertStmt {
    pub test: PyExprBox,
    pub msg: Option<PyExprBox>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyImportStmt {
    pub names: Vec<PyAlias>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyImportFromStmt {
    pub module: Option<String>,
    pub names: Vec<PyAlias>,
    pub level: Option<i32>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyGlobalStmt {
    pub names: Vec<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyNonlocalStmt {
    pub names: Vec<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyExprStmt {
    pub value: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyPassStmt {
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyBreakStmt {
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyContinueStmt {
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PyStatement {
    FunctionDef(Box<PyFunctionDefStmt>),
    ClassDef(Box<PyClassDefStmt>),
    Return(Box<PyReturnStmt>),
    Delete(Box<PyDeleteStmt>),
    Assign(Box<PyAssignStmt>),
    TypeAlias(Box<PyTypeAliasStmt>),
    AugAssign(Box<PyAugAssignStmt>),
    AnnAssign(Box<PyAnnAssignStmt>),
    For(Box<PyForStmt>),
    AsyncFor(Box<PyAsyncForStmt>),
    While(Box<PyWhileStmt>),
    If(Box<PyIfStmt>),
    With(Box<PyWithStmt>),
    AsyncWith(Box<PyAsyncWithStmt>),
    Match(Box<PyMatchStmt>),
    Raise(Box<PyRaiseStmt>),
    Try(Box<PyTryStmt>),
    TryStar(Box<PyTryStarStmt>),
    Assert(Box<PyAssertStmt>),
    Import(Box<PyImportStmt>),
    ImportFrom(Box<PyImportFromStmt>),
    Global(Box<PyGlobalStmt>),
    Nonlocal(Box<PyNonlocalStmt>),
    Expr(Box<PyExprStmt>),
    Pass(Box<PyPassStmt>),
    Break(Box<PyBreakStmt>),
    Continue(Box<PyContinueStmt>),
    Block(Box<PyBlockStmt>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyBlockStmt {
    pub body: Vec<PyStatement>,
    pub span: PySpan,
}

impl PyBlockStmt {
    pub fn from_statements(body: Vec<PyStatement>) -> Self {
        let span = match (body.first(), body.last()) {
            (Some(first), Some(last)) => {
                PySpan::span(first.span().start, last.span().end_or_start())
            }
            _ => PySpan::default(),
        };
        Self { body, span }
    }
}

impl PyStatement {
    /// Where the statement sits in the source, from its first token to the
    /// end of its last one. A compound statement ends with its last body
    /// statement.
    pub fn span(&self) -> PySpan {
        match self {
            PyStatement::FunctionDef(stmt) => stmt.span,
            PyStatement::ClassDef(stmt) => stmt.span,
            PyStatement::Return(stmt) => stmt.span,
            PyStatement::Delete(stmt) => stmt.span,
            PyStatement::Assign(stmt) => stmt.span,
            PyStatement::TypeAlias(stmt) => stmt.span,
            PyStatement::AugAssign(stmt) => stmt.span,
            PyStatement::AnnAssign(stmt) => stmt.span,
            PyStatement::For(stmt) => stmt.span,
            PyStatement::AsyncFor(stmt) => stmt.span,
            PyStatement::While(stmt) => stmt.span,
            PyStatement::If(stmt) => stmt.span,
            PyStatement::With(stmt) => stmt.span,
            PyStatement::AsyncWith(stmt) => stmt.span,
            PyStatement::Match(stmt) => stmt.span,
            PyStatement::Raise(stmt) => stmt.span,
            PyStatement::Try(stmt) => stmt.span,
            PyStatement::TryStar(stmt) => stmt.span,
            PyStatement::Assert(stmt) => stmt.span,
            PyStatement::Import(stmt) => stmt.span,
            PyStatement::ImportFrom(stmt) => stmt.span,
            PyStatement::Global(stmt) => stmt.span,
            PyStatement::Nonlocal(stmt) => stmt.span,
            PyStatement::Expr(stmt) => stmt.span,
            PyStatement::Pass(stmt) => stmt.span,
            PyStatement::Break(stmt) => stmt.span,
            PyStatement::Continue(stmt) => stmt.span,
            PyStatement::Block(stmt) => stmt.span,
        }
    }
}
