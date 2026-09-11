use super::expr::PyExpr;
use super::ops::PyBinaryOp;
use super::shared::{
    PyAlias, PyArguments, PyExceptHandler, PyKeyword, PyMatchCase, PyTypeParam, PyWithItem,
};

pub enum PyStatement {
    FunctionDef {
        name: String,
        args: PyArguments,
        body: Vec<PyStatement>,
        decorator_list: Vec<PyExpr>,
        returns: Option<PyExpr>,
        type_comment: Option<String>,
        type_params: Vec<PyTypeParam>,
    },
    AsyncFunctionDef {
        name: String,
        args: PyArguments,
        body: Vec<PyStatement>,
        decorator_list: Vec<PyExpr>,
        returns: Option<PyExpr>,
        type_comment: Option<String>,
        type_params: Vec<PyTypeParam>,
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
        value: Option<PyExpr>,
    },
    Delete {
        targets: Vec<PyExpr>,
    },
    Assign {
        targets: Vec<PyExpr>,
        value: PyExpr,
        type_comment: Option<String>,
    },
    TypeAlias {
        name: PyExpr,
        type_params: Vec<PyTypeParam>,
        value: PyExpr,
    },
    AugAssign {
        target: PyExpr,
        op: PyBinaryOp,
        value: PyExpr,
    },
    AnnAssign {
        target: PyExpr,
        annotation: PyExpr,
        value: Option<PyExpr>,
        simple: bool,
    },
    For {
        target: PyExpr,
        iter: PyExpr,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        type_comment: Option<String>,
    },
    AsyncFor {
        target: PyExpr,
        iter: PyExpr,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
        type_comment: Option<String>,
    },
    While {
        test: PyExpr,
        body: Vec<PyStatement>,
        orelse: Vec<PyStatement>,
    },
    If {
        test: PyExpr,
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
        subject: PyExpr,
        cases: Vec<PyMatchCase>,
    },
    Raise {
        exc: Option<PyExpr>,
        cause: Option<PyExpr>,
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
        test: PyExpr,
        msg: Option<PyExpr>,
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
        value: PyExpr,
    },
    Pass,
    Break,
    Continue,
}
