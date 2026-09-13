#[derive(Debug, Clone, Copy)]
pub enum PyUnaryOp {
    UnarySub,
    UnaryAdd,
    Not,
    Invert,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum PyBoolOp {
    Or,
    And,
}

#[derive(Debug, Clone, Copy)]
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
