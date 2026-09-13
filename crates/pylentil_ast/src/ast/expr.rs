use super::constant::PyConstant;
use super::context::PyRefContext;
use super::ops::{PyBinaryOp, PyBoolOp, PyComparisonOp, PyUnaryOp};
use super::shared::{PyArguments, PyComprehension, PyKeyword};

pub type PyExprBox = Box<PyExpr>;

#[derive(Debug)]
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
        args: PyArguments,
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
        args: Vec<PyExpr>,
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
