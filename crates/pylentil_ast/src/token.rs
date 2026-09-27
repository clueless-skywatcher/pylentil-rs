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

impl PyTokenType {
    /// A noun phrase naming this kind of token, for error messages.
    pub fn describe(self) -> &'static str {
        match self {
            PyTokenType::Indent => "an indent",
            PyTokenType::Dedent => "a dedent",
            PyTokenType::Whitespace => "whitespace",
            PyTokenType::Newline => "the end of the line",
            PyTokenType::EOF => "the end of the file",

            PyTokenType::Ident => "an identifier",
            PyTokenType::Int => "an integer literal",
            PyTokenType::Float => "a float literal",
            PyTokenType::String => "a string literal",
            PyTokenType::Binary => "a binary literal",
            PyTokenType::Hexadecimal => "a hexadecimal literal",
            PyTokenType::Octal => "an octal literal",
            PyTokenType::ENotation => "a number in exponent notation",
            PyTokenType::Imaginary => "an imaginary literal",

            PyTokenType::False => "keyword `False`",
            PyTokenType::None => "keyword `None`",
            PyTokenType::True => "keyword `True`",
            PyTokenType::And => "keyword `and`",
            PyTokenType::As => "keyword `as`",
            PyTokenType::Assert => "keyword `assert`",
            PyTokenType::Async => "keyword `async`",
            PyTokenType::Await => "keyword `await`",
            PyTokenType::Break => "keyword `break`",
            PyTokenType::Class => "keyword `class`",
            PyTokenType::Continue => "keyword `continue`",
            PyTokenType::Def => "keyword `def`",
            PyTokenType::Del => "keyword `del`",
            PyTokenType::Elif => "keyword `elif`",
            PyTokenType::Else => "keyword `else`",
            PyTokenType::Except => "keyword `except`",
            PyTokenType::Finally => "keyword `finally`",
            PyTokenType::For => "keyword `for`",
            PyTokenType::From => "keyword `from`",
            PyTokenType::Global => "keyword `global`",
            PyTokenType::If => "keyword `if`",
            PyTokenType::Import => "keyword `import`",
            PyTokenType::In => "keyword `in`",
            PyTokenType::Is => "keyword `is`",
            PyTokenType::Lambda => "keyword `lambda`",
            PyTokenType::Nonlocal => "keyword `nonlocal`",
            PyTokenType::Not => "keyword `not`",
            PyTokenType::Or => "keyword `or`",
            PyTokenType::Pass => "keyword `pass`",
            PyTokenType::Raise => "keyword `raise`",
            PyTokenType::Return => "keyword `return`",
            PyTokenType::Try => "keyword `try`",
            PyTokenType::While => "keyword `while`",
            PyTokenType::With => "keyword `with`",
            PyTokenType::Yield => "keyword `yield`",

            PyTokenType::LParen => "`(`",
            PyTokenType::RParen => "`)`",
            PyTokenType::LSquare => "`[`",
            PyTokenType::RSquare => "`]`",
            PyTokenType::LBrace => "`{`",
            PyTokenType::RBrace => "`}`",

            PyTokenType::Comma => "`,`",
            PyTokenType::Colon => "`:`",
            PyTokenType::Semicolon => "`;`",
            PyTokenType::Dot => "`.`",
            PyTokenType::Ellipsis => "`...`",
            PyTokenType::Assign => "`=`",
            PyTokenType::Tilde => "`~`",
            PyTokenType::At => "`@`",
            PyTokenType::Arrow => "`->`",
            PyTokenType::Walrus => "`:=`",

            PyTokenType::Plus => "`+`",
            PyTokenType::Minus => "`-`",
            PyTokenType::Star => "`*`",
            PyTokenType::DoubleStar => "`**`",
            PyTokenType::Slash => "`/`",
            PyTokenType::DoubleSlash => "`//`",
            PyTokenType::Percent => "`%`",
            PyTokenType::LShift => "`<<`",
            PyTokenType::RShift => "`>>`",

            PyTokenType::PlusEqual => "`+=`",
            PyTokenType::MinusEqual => "`-=`",
            PyTokenType::StarEqual => "`*=`",
            PyTokenType::DoubleStarEqual => "`**=`",
            PyTokenType::SlashEqual => "`/=`",
            PyTokenType::DoubleSlashEqual => "`//=`",
            PyTokenType::PercentEqual => "`%=`",
            PyTokenType::AtEqual => "`@=`",
            PyTokenType::AmpersandEqual => "`&=`",
            PyTokenType::VerticalBarEqual => "`|=`",
            PyTokenType::CaretEqual => "`^=`",
            PyTokenType::LShiftEqual => "`<<=`",
            PyTokenType::RShiftEqual => "`>>=`",

            PyTokenType::DoubleEqual => "`==`",
            PyTokenType::NotEqual => "`!=`",
            PyTokenType::Less => "`<`",
            PyTokenType::Greater => "`>`",
            PyTokenType::LessEqual => "`<=`",
            PyTokenType::GreaterEqual => "`>=`",

            PyTokenType::Ampersand => "`&`",
            PyTokenType::VerticalBar => "`|`",
            PyTokenType::Caret => "`^`",
        }
    }

    pub fn is_eof(self) -> bool {
        self == PyTokenType::EOF
    }
}

/// Joins the kinds a parser was willing to accept into one phrase, so an
/// error can say "expected `:` or the end of the line".
pub fn describe_any_of(kinds: &[PyTokenType]) -> String {
    match kinds {
        [] => "nothing".to_string(),
        [only] => only.describe().to_string(),
        [first, second] => format!("{} or {}", first.describe(), second.describe()),
        [rest @ .., last] => format!(
            "{}, or {}",
            rest.iter()
                .map(|kind| kind.describe())
                .collect::<Vec<_>>()
                .join(", "),
            last.describe()
        ),
    }
}

impl<'a> PyToken<'a> {
    /// The token as an error message would name it, quoting its text when it
    /// carries any: `an identifier (`total`)`.
    pub fn describe(&self) -> String {
        match &self.value {
            Some(value) => format!("{} (`{}`)", self.kind.describe(), value),
            None => self.kind.describe().to_string(),
        }
    }
}
