#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyTokenType {
    Indent,
    Dedent,
    Whitespace,
    Newline,
    EOF,

    KeywordOrIdent,
    Int,
    Float,
    String,

    LParen,
    RParen,
    LSquare,
    RSquare,
    LBrace,
    RBrace,

    Comma,
    Colon,
    Semicolon,
    Dot,
    Assign,
    Tilde,

    Plus,
    Minus,
    Star,
    DoubleStar,
    Slash,
    Percent,
    LShift,
    RShift,

    DoubleEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub struct PyToken {
    pub kind: PyTokenType,
    pub value: Option<String>
}
impl std::fmt::Display for PyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.value)
    }
}