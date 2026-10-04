use pylentil_common::span::PySpan;

use super::constant::PyConstant;
use super::context::PyRefContext;
use super::ops::{PyBinaryOp, PyBoolOp, PyComparisonOp, PyUnaryOp};
use super::shared::{PyArguments, PyComprehension, PyKeyword};

pub type PyExprBox = Box<PyExpr>;

#[derive(Debug, Clone, PartialEq)]
pub enum PyExpr {
    BoolOp {
        op: PyBoolOp,
        values: Vec<PyExpr>,
        span: PySpan,
    },
    NamedExpr {
        target: PyExprBox,
        value: PyExprBox,
        span: PySpan,
    },
    BinOp {
        left: PyExprBox,
        op: PyBinaryOp,
        right: PyExprBox,
        span: PySpan,
    },
    UnaryOp {
        op: PyUnaryOp,
        operand: PyExprBox,
        span: PySpan,
    },
    Lambda {
        args: Box<PyArguments>,
        body: PyExprBox,
        span: PySpan,
    },
    IfExp {
        test: PyExprBox,
        body: PyExprBox,
        orelse: PyExprBox,
        span: PySpan,
    },
    Dict {
        keys: Vec<Option<PyExpr>>,
        values: Vec<PyExpr>,
        span: PySpan,
    },
    Set {
        elts: Vec<PyExpr>,
        span: PySpan,
    },
    ListComp {
        elt: PyExprBox,
        generators: Vec<PyComprehension>,
        span: PySpan,
    },
    SetComp {
        elt: PyExprBox,
        generators: Vec<PyComprehension>,
        span: PySpan,
    },
    DictComp {
        key: PyExprBox,
        value: PyExprBox,
        generators: Vec<PyComprehension>,
        span: PySpan,
    },
    GeneratorExp {
        elt: PyExprBox,
        generators: Vec<PyComprehension>,
        span: PySpan,
    },
    Await {
        value: PyExprBox,
        span: PySpan,
    },
    Yield {
        value: Option<PyExprBox>,
        span: PySpan,
    },
    YieldFrom {
        value: PyExprBox,
        span: PySpan,
    },
    Compare {
        left: PyExprBox,
        ops: Vec<PyComparisonOp>,
        comparators: Vec<PyExpr>,
        span: PySpan,
    },
    Call {
        func: PyExprBox,
        args: Vec<PyExprBox>,
        keywords: Vec<PyKeyword>,
        span: PySpan,
    },
    FormattedValue {
        value: PyExprBox,
        conversion: i32,
        format_spec: Option<PyExprBox>,
        span: PySpan,
    },
    JoinedStr {
        values: Vec<PyExpr>,
        span: PySpan,
    },
    Constant {
        value: PyConstant,
        kind: Option<String>,
        span: PySpan,
    },
    Attribute {
        value: PyExprBox,
        attr: String,
        ctx: PyRefContext,
        span: PySpan,
    },
    Subscript {
        value: PyExprBox,
        slice: PyExprBox,
        ctx: PyRefContext,
        span: PySpan,
    },
    Starred {
        value: PyExprBox,
        ctx: PyRefContext,
        span: PySpan,
    },
    Name {
        id: String,
        ctx: PyRefContext,
        span: PySpan,
    },
    List {
        elts: Vec<PyExpr>,
        ctx: PyRefContext,
        span: PySpan,
    },
    Tuple {
        elts: Vec<PyExpr>,
        ctx: PyRefContext,
        parenthesized: bool,
        span: PySpan,
    },
    Slice {
        lower: Option<PyExprBox>,
        upper: Option<PyExprBox>,
        step: Option<PyExprBox>,
        span: PySpan,
    },
}

impl PyExpr {
    /// Where the expression sits in the source. A parenthesized tuple,
    /// generator or comprehension includes its brackets; any other
    /// parenthesized expression does not.
    pub fn span(&self) -> PySpan {
        match self {
            PyExpr::BoolOp { span, .. }
            | PyExpr::NamedExpr { span, .. }
            | PyExpr::BinOp { span, .. }
            | PyExpr::UnaryOp { span, .. }
            | PyExpr::Lambda { span, .. }
            | PyExpr::IfExp { span, .. }
            | PyExpr::Dict { span, .. }
            | PyExpr::Set { span, .. }
            | PyExpr::ListComp { span, .. }
            | PyExpr::SetComp { span, .. }
            | PyExpr::DictComp { span, .. }
            | PyExpr::GeneratorExp { span, .. }
            | PyExpr::Await { span, .. }
            | PyExpr::Yield { span, .. }
            | PyExpr::YieldFrom { span, .. }
            | PyExpr::Compare { span, .. }
            | PyExpr::Call { span, .. }
            | PyExpr::FormattedValue { span, .. }
            | PyExpr::JoinedStr { span, .. }
            | PyExpr::Constant { span, .. }
            | PyExpr::Attribute { span, .. }
            | PyExpr::Subscript { span, .. }
            | PyExpr::Starred { span, .. }
            | PyExpr::Name { span, .. }
            | PyExpr::List { span, .. }
            | PyExpr::Tuple { span, .. }
            | PyExpr::Slice { span, .. } => *span,
        }
    }

    /// A noun phrase naming this kind of expression, for error messages.
    pub fn describe(&self) -> &'static str {
        match self {
            PyExpr::BoolOp { .. } => "a boolean operation",
            PyExpr::NamedExpr { .. } => "a walrus expression",
            PyExpr::BinOp { .. } => "a binary operation",
            PyExpr::UnaryOp { .. } => "a unary operation",
            PyExpr::Lambda { .. } => "a lambda",
            PyExpr::IfExp { .. } => "a conditional expression",
            PyExpr::Dict { .. } => "a dict literal",
            PyExpr::Set { .. } => "a set literal",
            PyExpr::ListComp { .. } => "a list comprehension",
            PyExpr::SetComp { .. } => "a set comprehension",
            PyExpr::DictComp { .. } => "a dict comprehension",
            PyExpr::GeneratorExp { .. } => "a generator expression",
            PyExpr::Await { .. } => "an await expression",
            PyExpr::Yield { .. } => "a yield expression",
            PyExpr::YieldFrom { .. } => "a yield-from expression",
            PyExpr::Compare { .. } => "a comparison",
            PyExpr::Call { .. } => "a function call",
            PyExpr::FormattedValue { .. } => "a formatted value",
            PyExpr::JoinedStr { .. } => "an f-string",
            PyExpr::Constant { .. } => "a literal",
            PyExpr::Attribute { .. } => "an attribute access",
            PyExpr::Subscript { .. } => "a subscript",
            PyExpr::Starred { .. } => "a starred expression",
            PyExpr::Name { .. } => "a name",
            PyExpr::List { .. } => "a list",
            PyExpr::Tuple { .. } => "a tuple",
            PyExpr::Slice { .. } => "a slice",
        }
    }
}
