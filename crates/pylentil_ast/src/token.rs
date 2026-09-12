use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyTokenType {
    Indent,
    Dedent,
    Whitespace,
    Newline,
    EOF,

    Keyword,
    Ident,
    Int,
    Float,
    String,
    Boolean,
    NoneValue,

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

    Ampersand,
    VerticalBar,
    Caret,
}

#[derive(Debug, Clone)]
pub struct PyToken<'a> {
    pub kind: PyTokenType,
    pub value: Option<Cow<'a, str>>,
}

impl<'a> std::fmt::Display for PyToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.value)
    }
}
