use super::constant::PyConstant;
use super::context::PyRefContext;
use super::ops::{PyBinaryOp, PyBoolOp, PyComparisonOp, PyUnaryOp};

type PyASTNodeBox = Box<PyASTNode>;

pub struct PyKeywordArg {
    pub arg: String,
    pub value: PyASTNodeBox,
}

pub struct PyComprehension {
    pub target: PyASTNodeBox,
    pub iter: PyASTNodeBox,
    pub is_async: bool,
}

pub struct PyAlias {
    pub name: String,
    pub asname: String,
}

pub enum PyASTNode {
    Module {
        body: Vec<PyASTNodeBox>,
    },

    Expr {
        value: PyASTNodeBox,
    },
    Constant {
        value: PyConstant,
    },
    List {
        elts: Vec<PyASTNodeBox>,
    },
    Tuple {
        elts: Vec<PyASTNodeBox>,
    },

    Name {
        id: String,
        ctx: PyRefContext,
    },

    Assign {
        targets: Vec<PyASTNodeBox>,
        value: PyASTNodeBox,
    },
    Delete {
        targets: Vec<PyASTNodeBox>,
    },

    Starred {
        value: PyASTNodeBox,
        ctx: PyRefContext,
    },

    UnaryOp {
        op: PyUnaryOp,
        operand: PyASTNodeBox,
    },
    BinOp {
        op: PyBinaryOp,
        left: PyASTNodeBox,
        right: PyASTNodeBox,
    },
    BoolOp {
        op: PyBoolOp,
        left: PyASTNodeBox,
        right: PyASTNodeBox,
    },
    Compare {
        op: PyComparisonOp,
        left: PyASTNodeBox,
        right: PyASTNodeBox,
    },
    Call {
        func: PyASTNodeBox,
        args: Vec<PyASTNodeBox>,
        keywords: Vec<PyKeywordArg>,
    },

    If {
        test: PyASTNodeBox,
        body: PyASTNodeBox,
        orelse: PyASTNodeBox,
    },
    For {
        target: PyASTNodeBox,
        iter: PyASTNodeBox,
        body: PyASTNodeBox,
        orelse: PyASTNodeBox,
    },
    While {
        test: PyASTNodeBox,
        body: Vec<PyASTNodeBox>,
        orelse: Vec<PyASTNodeBox>,
    },
    Break,
    Continue,

    Attribute {
        value: PyASTNodeBox,
        attr: String,
        ctx: PyRefContext,
    },

    Index {
        value: PyASTNodeBox,
        slice: PyASTNodeBox,
        ctx: PyRefContext,
    },
    Slice {
        lower: PyASTNodeBox,
        upper: PyASTNodeBox,
        step: PyASTNodeBox,
    },

    ListComprehension {
        elt: PyASTNodeBox,
        generators: Vec<PyComprehension>,
    },
    SetComprehension {
        elt: PyASTNodeBox,
        generators: Vec<PyComprehension>,
    },
    GeneratorExp {
        elt: PyASTNodeBox,
        generators: Vec<PyComprehension>,
    },
    DictComprehension {
        key: PyASTNodeBox,
        value: PyASTNodeBox,
        generators: Vec<PyComprehension>,
    },

    Annotation {
        target: PyASTNodeBox,
        annotation: PyASTNodeBox,
        value: PyASTNodeBox,
        simple: bool,
    },

    AugmentedAssign {
        target: PyASTNodeBox,
        op: PyBinaryOp,
        value: PyASTNodeBox,
    },

    Raise {
        exc: PyASTNodeBox,
        cause: PyASTNodeBox,
    },

    Pass,

    Import {
        names: Vec<PyAlias>,
    },

    ImportFrom {
        module: String,
        names: Vec<PyAlias>,
    },
}
