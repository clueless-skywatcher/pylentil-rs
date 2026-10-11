use pylentil_common::span::PySpan;

use super::constant::PyConstant;
use super::context::PyRefContext;
use super::ops::{PyBinaryOp, PyBoolOp, PyComparisonOp, PyUnaryOp};
use super::shared::{PyArguments, PyComprehension, PyKeyword};

pub type PyExprBox = Box<PyExpr>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyBoolOpExpr {
    pub op: PyBoolOp,
    pub values: Vec<PyExpr>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyNamedExprExpr {
    pub target: PyExprBox,
    pub value: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyBinOpExpr {
    pub left: PyExprBox,
    pub op: PyBinaryOp,
    pub right: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyUnaryOpExpr {
    pub op: PyUnaryOp,
    pub operand: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyLambdaExpr {
    pub args: Box<PyArguments>,
    pub body: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyIfExpExpr {
    pub test: PyExprBox,
    pub body: PyExprBox,
    pub orelse: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyDictExpr {
    pub keys: Vec<Option<PyExpr>>,
    pub values: Vec<PyExpr>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PySetExpr {
    pub elts: Vec<PyExpr>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyListCompExpr {
    pub elt: PyExprBox,
    pub generators: Vec<PyComprehension>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PySetCompExpr {
    pub elt: PyExprBox,
    pub generators: Vec<PyComprehension>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyDictCompExpr {
    pub key: PyExprBox,
    pub value: PyExprBox,
    pub generators: Vec<PyComprehension>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyGeneratorExpExpr {
    pub elt: PyExprBox,
    pub generators: Vec<PyComprehension>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAwaitExpr {
    pub value: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyYieldExpr {
    pub value: Option<PyExprBox>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyYieldFromExpr {
    pub value: PyExprBox,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyCompareExpr {
    pub left: PyExprBox,
    pub ops: Vec<PyComparisonOp>,
    pub comparators: Vec<PyExpr>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyCallExpr {
    pub func: PyExprBox,
    pub args: Vec<PyExprBox>,
    pub keywords: Vec<PyKeyword>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyFormattedValueExpr {
    pub value: PyExprBox,
    pub conversion: i32,
    pub format_spec: Option<PyExprBox>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyJoinedStrExpr {
    pub values: Vec<PyExpr>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyConstantExpr {
    pub value: PyConstant,
    pub kind: Option<String>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyAttributeExpr {
    pub value: PyExprBox,
    pub attr: String,
    pub ctx: PyRefContext,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PySubscriptExpr {
    pub value: PyExprBox,
    pub slice: PyExprBox,
    pub ctx: PyRefContext,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyStarredExpr {
    pub value: PyExprBox,
    pub ctx: PyRefContext,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyNameExpr {
    pub id: String,
    pub ctx: PyRefContext,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyListExpr {
    pub elts: Vec<PyExpr>,
    pub ctx: PyRefContext,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyTupleExpr {
    pub elts: Vec<PyExpr>,
    pub ctx: PyRefContext,
    pub parenthesized: bool,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PySliceExpr {
    pub lower: Option<PyExprBox>,
    pub upper: Option<PyExprBox>,
    pub step: Option<PyExprBox>,
    pub span: PySpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PyExpr {
    BoolOp(PyBoolOpExpr),
    NamedExpr(PyNamedExprExpr),
    BinOp(PyBinOpExpr),
    UnaryOp(PyUnaryOpExpr),
    Lambda(PyLambdaExpr),
    IfExp(PyIfExpExpr),
    Dict(PyDictExpr),
    Set(PySetExpr),
    ListComp(PyListCompExpr),
    SetComp(PySetCompExpr),
    DictComp(PyDictCompExpr),
    GeneratorExp(PyGeneratorExpExpr),
    Await(PyAwaitExpr),
    Yield(PyYieldExpr),
    YieldFrom(PyYieldFromExpr),
    Compare(PyCompareExpr),
    Call(PyCallExpr),
    FormattedValue(PyFormattedValueExpr),
    JoinedStr(PyJoinedStrExpr),
    Constant(PyConstantExpr),
    Attribute(PyAttributeExpr),
    Subscript(PySubscriptExpr),
    Starred(PyStarredExpr),
    Name(PyNameExpr),
    List(PyListExpr),
    Tuple(PyTupleExpr),
    Slice(PySliceExpr),
}

impl PyExpr {
    /// Where the expression sits in the source. A parenthesized tuple,
    /// generator or comprehension includes its brackets; any other
    /// parenthesized expression does not.
    pub fn span(&self) -> PySpan {
        match self {
            PyExpr::BoolOp(expr) => expr.span,
            PyExpr::NamedExpr(expr) => expr.span,
            PyExpr::BinOp(expr) => expr.span,
            PyExpr::UnaryOp(expr) => expr.span,
            PyExpr::Lambda(expr) => expr.span,
            PyExpr::IfExp(expr) => expr.span,
            PyExpr::Dict(expr) => expr.span,
            PyExpr::Set(expr) => expr.span,
            PyExpr::ListComp(expr) => expr.span,
            PyExpr::SetComp(expr) => expr.span,
            PyExpr::DictComp(expr) => expr.span,
            PyExpr::GeneratorExp(expr) => expr.span,
            PyExpr::Await(expr) => expr.span,
            PyExpr::Yield(expr) => expr.span,
            PyExpr::YieldFrom(expr) => expr.span,
            PyExpr::Compare(expr) => expr.span,
            PyExpr::Call(expr) => expr.span,
            PyExpr::FormattedValue(expr) => expr.span,
            PyExpr::JoinedStr(expr) => expr.span,
            PyExpr::Constant(expr) => expr.span,
            PyExpr::Attribute(expr) => expr.span,
            PyExpr::Subscript(expr) => expr.span,
            PyExpr::Starred(expr) => expr.span,
            PyExpr::Name(expr) => expr.span,
            PyExpr::List(expr) => expr.span,
            PyExpr::Tuple(expr) => expr.span,
            PyExpr::Slice(expr) => expr.span,
        }
    }

    /// A noun phrase naming this kind of expression, for error messages.
    pub fn describe(&self) -> &'static str {
        match self {
            PyExpr::BoolOp(_) => "a boolean operation",
            PyExpr::NamedExpr(_) => "a walrus expression",
            PyExpr::BinOp(_) => "a binary operation",
            PyExpr::UnaryOp(_) => "a unary operation",
            PyExpr::Lambda(_) => "a lambda",
            PyExpr::IfExp(_) => "a conditional expression",
            PyExpr::Dict(_) => "a dict literal",
            PyExpr::Set(_) => "a set literal",
            PyExpr::ListComp(_) => "a list comprehension",
            PyExpr::SetComp(_) => "a set comprehension",
            PyExpr::DictComp(_) => "a dict comprehension",
            PyExpr::GeneratorExp(_) => "a generator expression",
            PyExpr::Await(_) => "an await expression",
            PyExpr::Yield(_) => "a yield expression",
            PyExpr::YieldFrom(_) => "a yield-from expression",
            PyExpr::Compare(_) => "a comparison",
            PyExpr::Call(_) => "a function call",
            PyExpr::FormattedValue(_) => "a formatted value",
            PyExpr::JoinedStr(_) => "an f-string",
            PyExpr::Constant(_) => "a literal",
            PyExpr::Attribute(_) => "an attribute access",
            PyExpr::Subscript(_) => "a subscript",
            PyExpr::Starred(_) => "a starred expression",
            PyExpr::Name(_) => "a name",
            PyExpr::List(_) => "a list",
            PyExpr::Tuple(_) => "a tuple",
            PyExpr::Slice(_) => "a slice",
        }
    }
}
