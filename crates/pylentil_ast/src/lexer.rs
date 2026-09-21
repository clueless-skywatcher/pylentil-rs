use std::collections::HashSet;
use std::{borrow::Cow, ops::Index};

use pylentil_common::errors::PylentilError;

use crate::token::{PyToken, PyTokenType};

const BOM: &str = "\u{feff}";

const OPERATORS: &[(&str, PyTokenType)] = &[
    ("**=", PyTokenType::DoubleStarEqual),
    ("//=", PyTokenType::DoubleSlashEqual),
    ("<<=", PyTokenType::LShiftEqual),
    (">>=", PyTokenType::RShiftEqual),
    ("...", PyTokenType::Ellipsis),
    ("**", PyTokenType::DoubleStar),
    ("//", PyTokenType::DoubleSlash),
    ("<<", PyTokenType::LShift),
    (">>", PyTokenType::RShift),
    ("+=", PyTokenType::PlusEqual),
    ("-=", PyTokenType::MinusEqual),
    ("*=", PyTokenType::StarEqual),
    ("/=", PyTokenType::SlashEqual),
    ("%=", PyTokenType::PercentEqual),
    ("@=", PyTokenType::AtEqual),
    ("&=", PyTokenType::AmpersandEqual),
    ("|=", PyTokenType::VerticalBarEqual),
    ("^=", PyTokenType::CaretEqual),
    ("==", PyTokenType::DoubleEqual),
    ("!=", PyTokenType::NotEqual),
    ("<=", PyTokenType::LessEqual),
    (">=", PyTokenType::GreaterEqual),
    ("->", PyTokenType::Arrow),
    (":=", PyTokenType::Walrus),
    ("(", PyTokenType::LParen),
    (")", PyTokenType::RParen),
    ("[", PyTokenType::LSquare),
    ("]", PyTokenType::RSquare),
    ("{", PyTokenType::LBrace),
    ("}", PyTokenType::RBrace),
    (",", PyTokenType::Comma),
    (":", PyTokenType::Colon),
    (";", PyTokenType::Semicolon),
    (".", PyTokenType::Dot),
    ("+", PyTokenType::Plus),
    ("-", PyTokenType::Minus),
    ("*", PyTokenType::Star),
    ("/", PyTokenType::Slash),
    ("%", PyTokenType::Percent),
    ("@", PyTokenType::At),
    ("=", PyTokenType::Assign),
    ("<", PyTokenType::Less),
    (">", PyTokenType::Greater),
    ("&", PyTokenType::Ampersand),
    ("|", PyTokenType::VerticalBar),
    ("^", PyTokenType::Caret),
    ("~", PyTokenType::Tilde),
];

fn keyword_type(value: &str) -> Option<PyTokenType> {
    Some(match value {
        "False" => PyTokenType::False,
        "None" => PyTokenType::None,
        "True" => PyTokenType::True,
        "and" => PyTokenType::And,
        "as" => PyTokenType::As,
        "assert" => PyTokenType::Assert,
        "async" => PyTokenType::Async,
        "await" => PyTokenType::Await,
        "break" => PyTokenType::Break,
        "class" => PyTokenType::Class,
        "continue" => PyTokenType::Continue,
        "def" => PyTokenType::Def,
        "del" => PyTokenType::Del,
        "elif" => PyTokenType::Elif,
        "else" => PyTokenType::Else,
        "except" => PyTokenType::Except,
        "finally" => PyTokenType::Finally,
        "for" => PyTokenType::For,
        "from" => PyTokenType::From,
        "global" => PyTokenType::Global,
        "if" => PyTokenType::If,
        "import" => PyTokenType::Import,
        "in" => PyTokenType::In,
        "is" => PyTokenType::Is,
        "lambda" => PyTokenType::Lambda,
        "nonlocal" => PyTokenType::Nonlocal,
        "not" => PyTokenType::Not,
        "or" => PyTokenType::Or,
        "pass" => PyTokenType::Pass,
        "raise" => PyTokenType::Raise,
        "return" => PyTokenType::Return,
        "try" => PyTokenType::Try,
        "while" => PyTokenType::While,
        "with" => PyTokenType::With,
        "yield" => PyTokenType::Yield,
        _ => return None,
    })
}

#[derive(Debug)]
pub struct PyLexer<'a> {
    pub code: &'a str,
    pub tokens: Vec<PyToken<'a>>,
}

impl<'a> PyLexer<'a> {
    fn peek(code: &str, pos: usize) -> Result<u8, PylentilError> {
        if pos >= code.len() {
            Err(PylentilError::EndOfFileReached)
        } else {
            Ok(code.as_bytes()[pos])
        }
    }

    fn consume(code: &str, pos: &mut usize) -> Result<u8, PylentilError> {
        let byte = Self::peek(code, *pos)?;
        *pos += 1;
        Ok(byte)
    }

    fn consume_while(code: &'a str, pos: &mut usize, pred: impl Fn(u8) -> bool) -> &'a str {
        let start = *pos;
        while let Ok(b) = Self::peek(code, *pos) {
            if !pred(b) {
                break;
            }
            *pos += 1;
        }
        &code[start..*pos]
    }

    pub fn from_code(code: &'a str) -> Result<Self, PylentilError> {
        let mut tokens = Vec::<PyToken>::new();
        let mut i = if code.starts_with(BOM) { BOM.len() } else { 0 };
        let mut at_line_start = true;
        let mut depth = 0usize;

        while i < code.len() {
            let mut byte = Self::peek(code, i)?;

            if byte == b'#' {
                while byte != b'\n' {
                    if i >= code.len() {
                        return Ok(PyLexer { tokens, code });
                    }
                    Self::consume(code, &mut i)?;
                    byte = code.chars().nth(i).unwrap() as u8;
                    continue;
                }
            }

            if at_line_start && (byte == b' ' || byte == b'\t') {
                let value = Self::consume_while(code, &mut i, |b| b == b' ' || b == b'\t');

                if depth == 0 && Self::carries_code(code, i) {
                    if !has_all_same_chars(value) {
                        return Err(PylentilError::MixedSpacesAndTabs);
                    }

                    tokens.push(PyToken {
                        kind: PyTokenType::Indent,
                        value: Some(Cow::Borrowed(value)),
                    });
                }

                at_line_start = false;
                continue;
            }

            let token = match byte {
                b' ' | b'\t' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Whitespace,
                        value: None,
                    }
                }
                b'\n' | b'\r' => {
                    Self::skip_line_break(code, &mut i);
                    at_line_start = true;

                    if depth > 0 {
                        continue;
                    }

                    PyToken {
                        kind: PyTokenType::Newline,
                        value: None,
                    }
                }
                b'\\' if Self::is_line_break(code, i + 1) => {
                    i += 1;
                    Self::skip_line_break(code, &mut i);
                    at_line_start = false;
                    continue;
                }
                b'\'' | b'"' => Self::lex_string(code, &mut i)?,
                b'.' if Self::is_digit(code, i + 1) => Self::lex_number(code, &mut i)?,
                b if b.is_ascii_digit() => Self::lex_number(code, &mut i)?,
                b if b.is_ascii_alphabetic() || b == b'_' => {
                    let word = Self::consume_while(code, &mut i, is_ident_part);

                    if is_string_prefix(word) && matches!(Self::peek(code, i), Ok(b'\'') | Ok(b'"'))
                    {
                        Self::lex_string(code, &mut i)?
                    } else if let Some(kind) = keyword_type(word) {
                        PyToken { kind, value: None }
                    } else {
                        PyToken {
                            kind: PyTokenType::Ident,
                            value: Some(Cow::Borrowed(word)),
                        }
                    }
                }
                _ => Self::lex_operator(code, &mut i)?,
            };

            match token.kind {
                PyTokenType::LParen | PyTokenType::LSquare | PyTokenType::LBrace => depth += 1,
                PyTokenType::RParen | PyTokenType::RSquare | PyTokenType::RBrace => {
                    depth = depth.saturating_sub(1)
                }
                _ => {}
            }

            if token.kind != PyTokenType::Newline {
                at_line_start = false;
            }
            tokens.push(token);
        }

        tokens.push(PyToken {
            kind: PyTokenType::EOF,
            value: None,
        });

        PyLexer { code, tokens }.indent_pass()
    }

    fn carries_code(code: &str, pos: usize) -> bool {
        !matches!(Self::peek(code, pos), Err(_) | Ok(b'\n') | Ok(b'\r') | Ok(b'#'))
    }

    fn is_line_break(code: &str, pos: usize) -> bool {
        matches!(Self::peek(code, pos), Ok(b'\n') | Ok(b'\r'))
    }

    fn skip_line_break(code: &str, pos: &mut usize) {
        if Self::peek(code, *pos) == Ok(b'\r') {
            *pos += 1;
        }
        if Self::peek(code, *pos) == Ok(b'\n') {
            *pos += 1;
        }
    }

    fn lex_operator(code: &'a str, pos: &mut usize) -> Result<PyToken<'a>, PylentilError> {
        let rest = &code[*pos..];

        let Some((spelling, kind)) = OPERATORS
            .iter()
            .find(|(spelling, _)| rest.starts_with(spelling))
        else {
            let character = rest.chars().next().ok_or(PylentilError::EndOfFileReached)?;
            return Err(PylentilError::UnknownCharacter { character });
        };

        *pos += spelling.len();

        Ok(PyToken {
            kind: *kind,
            value: None,
        })
    }

    fn lex_string(code: &'a str, pos: &mut usize) -> Result<PyToken<'a>, PylentilError> {
        let quote = Self::peek(code, *pos)?;
        let marker = if quote == b'"' { "\"\"\"" } else { "'''" };
        let triple = code[*pos..].starts_with(marker);
        let opening = if triple { marker.len() } else { 1 };
        let opened_with = || match (triple, quote) {
            (true, b'"') => "\"\"\"".to_string(),
            (true, _) => "'''".to_string(),
            (false, b'"') => "\"".to_string(),
            (false, _) => "'".to_string(),
        };

        *pos += opening;
        let start = *pos;

        loop {
            match Self::peek(code, *pos) {
                Err(PylentilError::EndOfFileReached) => {
                    return Err(PylentilError::UnterminatedString {
                        quote: opened_with(),
                    });
                }
                Err(e) => return Err(e),
                Ok(b'\\') => {
                    *pos += 1;
                    Self::consume(code, pos).map_err(|_| PylentilError::UnterminatedString {
                        quote: opened_with(),
                    })?;
                }
                Ok(b'\n') | Ok(b'\r') if !triple => {
                    return Err(PylentilError::UnterminatedStringLine {
                        quote: opened_with(),
                    });
                }
                Ok(b) if b == quote => {
                    if !triple || code[*pos..].starts_with(marker) {
                        break;
                    }
                    *pos += 1;
                }
                Ok(_) => *pos += 1,
            }
        }

        let value = &code[start..*pos];
        *pos += opening;

        Ok(PyToken {
            kind: PyTokenType::String,
            value: Some(Cow::Borrowed(value)),
        })
    }

    fn lex_number(code: &'a str, pos: &mut usize) -> Result<PyToken<'a>, PylentilError> {
        let start = *pos;

        if Self::peek(code, *pos) == Ok(b'0') {
            let radix = match Self::peek(code, *pos + 1) {
                Ok(b'b') | Ok(b'B') => {
                    Some((PyTokenType::Binary, is_binary_digit as DigitTest, "binary"))
                }
                Ok(b'o') | Ok(b'O') => {
                    Some((PyTokenType::Octal, is_octal_digit as DigitTest, "octal"))
                }
                Ok(b'x') | Ok(b'X') => Some((
                    PyTokenType::Hexadecimal,
                    is_hex_digit as DigitTest,
                    "hexadecimal",
                )),
                _ => None,
            };

            if let Some((kind, is_digit, base)) = radix {
                *pos += 2;
                return Self::lex_radix(code, start, pos, kind, is_digit, base);
            }
        }

        let mut kind = PyTokenType::Int;
        Self::consume_while(code, pos, is_number_part);

        if Self::peek(code, *pos) == Ok(b'.') {
            *pos += 1;
            Self::consume_while(code, pos, is_number_part);
            kind = PyTokenType::Float;
        }

        if matches!(Self::peek(code, *pos), Ok(b'e') | Ok(b'E'))
            && Self::starts_exponent(code, *pos + 1)
        {
            *pos += 1;
            if matches!(Self::peek(code, *pos), Ok(b'+') | Ok(b'-')) {
                *pos += 1;
            }
            Self::consume_while(code, pos, is_number_part);
            kind = PyTokenType::ENotation;
        }

        if matches!(Self::peek(code, *pos), Ok(b'j') | Ok(b'J')) {
            *pos += 1;
            kind = PyTokenType::Imaginary;
        }

        Ok(PyToken {
            kind,
            value: Some(Cow::Borrowed(&code[start..*pos])),
        })
    }

    fn lex_radix(
        code: &'a str,
        start: usize,
        pos: &mut usize,
        kind: PyTokenType,
        is_digit: DigitTest,
        base: &'static str,
    ) -> Result<PyToken<'a>, PylentilError> {
        let digits = Self::consume_while(code, pos, |b| b == b'_' || is_digit(b));

        if digits.chars().all(|c| c == '_') {
            return Err(PylentilError::EmptyNumericLiteral {
                prefix: code[start..*pos].to_string(),
                base,
            });
        }

        let value: String = digits
            .chars()
            .filter(|c| *c != '_')
            .map(|c| c.to_ascii_lowercase())
            .collect();

        Ok(PyToken {
            kind,
            value: Some(Cow::Owned(value)),
        })
    }

    fn starts_exponent(code: &str, pos: usize) -> bool {
        let pos = match Self::peek(code, pos) {
            Ok(b'+') | Ok(b'-') => pos + 1,
            _ => pos,
        };

        Self::is_digit(code, pos)
    }

    fn is_digit(code: &str, pos: usize) -> bool {
        matches!(Self::peek(code, pos), Ok(b) if b.is_ascii_digit())
    }

    fn indent_pass(&self) -> Result<Self, PylentilError> {
        if self.tokens[0].kind == PyTokenType::Indent {
            return Err(PylentilError::UnexpectedIndent);
        }

        let mut new_tokens: Vec<PyToken> = Vec::new();
        let mut tokens = self.tokens.clone();
        let mut indents: Vec<usize> = vec![0];

        while tokens.len() > 0 {
            let token = tokens[0].clone();
            tokens = tokens[1..].to_vec();

            match token {
                PyToken {
                    kind: PyTokenType::Newline,
                    ..
                } => match tokens[0].kind {
                    PyTokenType::EOF => {
                        new_tokens.push(token);
                        Self::dedent(0, &mut indents, &mut new_tokens)?;
                        new_tokens.push(tokens[0].clone());
                        break;
                    }
                    PyTokenType::Indent if Self::is_blank_line(&tokens, 1) => {
                        new_tokens.push(token);
                        tokens = tokens[1..].to_vec();
                    }
                    PyTokenType::Indent => {
                        let indent = get_indent_size(tokens[0].value.as_deref().unwrap())?;
                        if indent > *indents.last().unwrap() {
                            indents.push(indent);
                            new_tokens.push(token);
                            new_tokens.push(PyToken {
                                kind: PyTokenType::Indent,
                                value: Some(Cow::Owned(indent.to_string())),
                            });
                        } else if indent == *indents.last().unwrap() {
                            new_tokens.push(token);
                        } else {
                            new_tokens.push(token);
                            Self::dedent(indent, &mut indents, &mut new_tokens)?;
                        }

                        tokens = tokens[1..].to_vec();
                    }
                    PyTokenType::Newline => {
                        new_tokens.push(token);
                    }
                    _ => {
                        new_tokens.push(token);
                        Self::dedent(0, &mut indents, &mut new_tokens)?;
                    }
                },
                PyToken {
                    kind: PyTokenType::EOF,
                    ..
                } => {
                    Self::dedent(0, &mut indents, &mut new_tokens)?;
                    new_tokens.push(token);
                }
                _ => {
                    new_tokens.push(token);
                }
            }
        }

        // assert!(indents.len() == 1);

        Ok(PyLexer {
            code: self.code,
            tokens: new_tokens.clone(),
        })
    }

    fn is_blank_line(tokens: &[PyToken], at: usize) -> bool {
        match tokens.get(at) {
            Some(token) => matches!(token.kind, PyTokenType::Newline | PyTokenType::EOF),
            None => true,
        }
    }

    fn dedent(
        indent: usize,
        indents: &mut Vec<usize>,
        new_tokens: &mut Vec<PyToken>,
    ) -> Result<(), PylentilError> {
        while indent < *indents.last().unwrap() {
            indents.pop();
            let val = indents.last().unwrap().to_string();
            new_tokens.push(PyToken {
                kind: PyTokenType::Dedent,
                value: Some(Cow::Owned(val)),
            });
        }

        let enclosing = *indents.last().unwrap();
        if indent != enclosing {
            Err(PylentilError::InconsistentDedent {
                found: indent,
                enclosing,
            })
        } else {
            Ok(())
        }
    }

    pub fn spaces_scrapped(&self) -> Self {
        let tokens = self
            .tokens
            .iter()
            .filter(|x| {
                x.kind != PyTokenType::Whitespace
                    && x.kind != PyTokenType::Indent
                    && x.kind != PyTokenType::Newline
            })
            .cloned()
            .collect();

        PyLexer {
            code: self.code,
            tokens,
        }
    }
}

type DigitTest = fn(u8) -> bool;

fn is_ident_part(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn is_string_prefix(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "r" | "u" | "f" | "b" | "fr" | "rf" | "br" | "rb"
    )
}

fn is_number_part(byte: u8) -> bool {
    byte.is_ascii_digit() || byte == b'_'
}

fn is_binary_digit(byte: u8) -> bool {
    matches!(byte, b'0' | b'1')
}

fn is_octal_digit(byte: u8) -> bool {
    byte.is_ascii_digit() && byte < b'8'
}

fn is_hex_digit(byte: u8) -> bool {
    byte.is_ascii_hexdigit()
}

fn has_all_same_chars(s: &str) -> bool {
    HashSet::<char>::from_iter(s.chars()).len() == 1
}

fn get_indent_size(s: &str) -> Result<usize, PylentilError> {
    let mut size = 0usize;
    for char in s.chars() {
        size += match char {
            '\t' => 4,
            ' ' => 1,
            _ => return Err(PylentilError::InvalidIndentationCharacter { character: char }),
        }
    }
    Ok(size)
}
