#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PyUnaryOp {
    UnarySub,
    UnaryAdd,
    Not,
    Invert,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PyBoolOp {
    Or,
    And,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
