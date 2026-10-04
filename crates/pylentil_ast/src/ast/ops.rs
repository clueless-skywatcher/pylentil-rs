#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyUnaryOp {
    UnarySub,
    UnaryAdd,
    Not,
    Invert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    MatMult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyBoolOp {
    Or,
    And,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyComparisonOp {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    NotEq,
    Is,
    IsNot,
    In,
    NotIn,
}
