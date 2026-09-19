use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyTokenType {
    Indent,
    Dedent,
    Whitespace,
    Newline,
    EOF,

    Ident,
    Int,
    Float,
    String,
    Binary,
    Hexadecimal,
    Octal,
    ENotation,
    Imaginary,

    // Keywords
    False,
    None,
    True,
    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,

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
    Ellipsis,
    Assign,
    Tilde,
    At,
    Arrow,
    Walrus,

    Plus,
    Minus,
    Star,
    DoubleStar,
    Slash,
    DoubleSlash,
    Percent,
    LShift,
    RShift,

    PlusEqual,
    MinusEqual,
    StarEqual,
    DoubleStarEqual,
    SlashEqual,
    DoubleSlashEqual,
    PercentEqual,
    AtEqual,
    AmpersandEqual,
    VerticalBarEqual,
    CaretEqual,
    LShiftEqual,
    RShiftEqual,

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
