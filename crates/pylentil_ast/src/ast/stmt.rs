use super::expr::{PyExpr, PyExprBox};
use super::ops::PyBinaryOp;
use super::shared::{
    PyAlias, PyArguments, PyExceptHandler, PyKeyword, PyMatchCase, PyTypeParam, PyWithItem,
};

#[derive(Debug, Clone)]
pub enum PyStatement {
    FunctionDef {
        name: String,
        args: Box<PyArguments>,
        body: Vec<PyStatement>,
        decorator_list: Vec<PyExpr>,
        returns: Option<PyExprBox>,
        type_comment: Option<String>,
        type_params: Vec<PyTypeParam>,
        is_async: bool
    },
    ClassDef {
        name: String,
        bases: Vec<PyExpr>,
        keywords: Vec<PyKeyword>,
        body: Vec<PyStatement>,
        decorator_list: Vec<PyExpr>,
        type_params: Vec<PyTypeParam>,
    },
    Return {
        value: Option<PyExprBox>,
    },
    Delete {
        targets: Vec<PyExpr>,
    },
    Assign {
        targets: Vec<PyExpr>,
        value: PyExprBox,
        type_comment: Option<String>,
    },
    TypeAlias {
        name: PyExprBox,
        type_params: Vec<PyTypeParam>,
        value: PyExprBox,
    },
    AugAssign {
        target: PyExprBox,
        op: PyBinaryOp,
        value: PyExprBox,
    },
    AnnAssign {
        target: PyExprBox,
        annotation: PyExprBox,
        value: Option<PyExprBox>,
        simple: bool,
    },
    For {
        target: PyExprBox,
        iter: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        type_comment: Option<String>,
    },
    AsyncFor {
        target: PyExprBox,
        iter: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        type_comment: Option<String>,
    },
    While {
        test: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
    },
    If {
        test: PyExprBox,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
    },
    With {
        items: Vec<PyWithItem>,
        body: Vec<PyStatement>,
        type_comment: Option<String>,
    },
    AsyncWith {
        items: Vec<PyWithItem>,
        body: Vec<PyStatement>,
        type_comment: Option<String>,
    },
    Match {
        subject: PyExprBox,
        cases: Vec<PyMatchCase>,
    },
    Raise {
        exc: Option<PyExprBox>,
        cause: Option<PyExprBox>,
    },
    Try {
        body: Vec<PyStatement>,
        handlers: Vec<PyExceptHandler>,
        orelse: Vec<PyStatement>,
        finalbody: Vec<PyStatement>,
    },
    TryStar {
        body: Vec<PyStatement>,
        handlers: Vec<PyExceptHandler>,
        orelse: Vec<PyStatement>,
        finalbody: Vec<PyStatement>,
    },
    Assert {
        test: PyExprBox,
        msg: Option<PyExprBox>,
    },
    Import {
        names: Vec<PyAlias>,
    },
    ImportFrom {
        module: Option<String>,
        names: Vec<PyAlias>,
        level: Option<i32>,
    },
    Global {
        names: Vec<String>,
    },
    Nonlocal {
        names: Vec<String>,
    },
    Expr {
        value: PyExprBox,
    },
    Pass,
    Break,
    Continue,
}
