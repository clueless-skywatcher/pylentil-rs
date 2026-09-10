pub enum PyUnaryOp {
    UnarySub,
    UnaryAdd,
    Not,
    Invert,
}

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

pub enum PyBoolOp {
    Or,
    And,
}

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
