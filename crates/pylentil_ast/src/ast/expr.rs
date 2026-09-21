use super::constant::PyConstant;
use super::context::PyRefContext;
use super::ops::{PyBinaryOp, PyBoolOp, PyComparisonOp, PyUnaryOp};
use super::shared::{PyArguments, PyComprehension, PyKeyword};

pub type PyExprBox = Box<PyExpr>;

#[derive(Debug, Clone)]
pub enum PyExpr {
    BoolOp {
        op: PyBoolOp,
        values: Vec<PyExpr>,
    },
    NamedExpr {
        target: PyExprBox,
        value: PyExprBox,
    },
    BinOp {
        left: PyExprBox,
        op: PyBinaryOp,
        right: PyExprBox,
    },
    UnaryOp {
        op: PyUnaryOp,
        operand: PyExprBox,
    },
    Lambda {
        args: Box<PyArguments>,
        body: PyExprBox,
    },
    IfExp {
        test: PyExprBox,
        body: PyExprBox,
        orelse: PyExprBox,
    },
    Dict {
        keys: Vec<Option<PyExpr>>,
        values: Vec<PyExpr>,
    },
    Set {
        elts: Vec<PyExpr>,
    },
    ListComp {
        elt: PyExprBox,
        generators: Vec<PyComprehension>,
    },
    SetComp {
        elt: PyExprBox,
        generators: Vec<PyComprehension>,
    },
    DictComp {
        key: PyExprBox,
        value: PyExprBox,
        generators: Vec<PyComprehension>,
    },
    GeneratorExp {
        elt: PyExprBox,
        generators: Vec<PyComprehension>,
    },
    Await {
        value: PyExprBox,
    },
    Yield {
        value: Option<PyExprBox>,
    },
    YieldFrom {
        value: PyExprBox,
    },
    Compare {
        left: PyExprBox,
        ops: Vec<PyComparisonOp>,
        comparators: Vec<PyExpr>,
    },
    Call {
        func: PyExprBox,
        args: Vec<PyExprBox>,
        keywords: Vec<PyKeyword>,
    },
    FormattedValue {
        value: PyExprBox,
        conversion: i32,
        format_spec: Option<PyExprBox>,
    },
    JoinedStr {
        values: Vec<PyExpr>,
    },
    Constant {
        value: PyConstant,
        kind: Option<String>,
    },
    Attribute {
        value: PyExprBox,
        attr: String,
        ctx: PyRefContext,
    },
    Subscript {
        value: PyExprBox,
        slice: PyExprBox,
        ctx: PyRefContext,
    },
    Starred {
        value: PyExprBox,
        ctx: PyRefContext,
    },
    Name {
        id: String,
        ctx: PyRefContext,
    },
    List {
        elts: Vec<PyExpr>,
        ctx: PyRefContext,
    },
    Tuple {
        elts: Vec<PyExpr>,
        ctx: PyRefContext,
        parenthesized: bool
    },
    Slice {
        lower: Option<PyExprBox>,
        upper: Option<PyExprBox>,
        step: Option<PyExprBox>,
    },
}

impl PyExpr {
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
