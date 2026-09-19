use std::collections::HashSet;
use std::{borrow::Cow, ops::Index};

use pylentil_common::errors::PylentilError;

use crate::token::{PyToken, PyTokenType};

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
        let mut i = 0;
        let mut at_line_start = true;

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

                if !has_all_same_chars(value) {
                    return Err(PylentilError::MixedSpacesAndTabs);
                }

                tokens.push(PyToken {
                    kind: PyTokenType::Indent,
                    value: Some(Cow::Borrowed(value)),
                });
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
                b'\n' => {
                    Self::consume(code, &mut i)?;
                    at_line_start = true;
                    PyToken {
                        kind: PyTokenType::Newline,
                        value: None,
                    }
                }
                b'(' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::LParen,
                        value: None,
                    }
                }
                b')' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::RParen,
                        value: None,
                    }
                }
                b'[' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::LSquare,
                        value: None,
                    }
                }
                b']' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::RSquare,
                        value: None,
                    }
                }
                b'{' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::LBrace,
                        value: None,
                    }
                }
                b'}' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::RBrace,
                        value: None,
                    }
                }
                b',' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Comma,
                        value: None,
                    }
                }
                b':' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Colon,
                        value: None,
                    }
                }
                b';' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Semicolon,
                        value: None,
                    }
                }
                b'.' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Dot,
                        value: None,
                    }
                }
                b'+' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Plus,
                        value: None,
                    }
                }
                b'-' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Minus,
                        value: None,
                    }
                }
                b'%' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Percent,
                        value: None,
                    }
                }
                b'*' => {
                    Self::consume(code, &mut i)?;
                    if Self::peek(code, i) == Ok(b'*') {
                        Self::consume(code, &mut i)?;
                        PyToken {
                            kind: PyTokenType::DoubleStar,
                            value: None,
                        }
                    } else {
                        PyToken {
                            kind: PyTokenType::Star,
                            value: None,
                        }
                    }
                }
                b'/' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Slash,
                        value: None,
                    }
                }
                b'=' => {
                    Self::consume(code, &mut i)?;
                    if Self::peek(code, i) == Ok(b'=') {
                        Self::consume(code, &mut i)?;
                        PyToken {
                            kind: PyTokenType::DoubleEqual,
                            value: None,
                        }
                    } else {
                        PyToken {
                            kind: PyTokenType::Assign,
                            value: None,
                        }
                    }
                }
                b'!' => {
                    Self::consume(code, &mut i)?;
                    if Self::peek(code, i) == Ok(b'=') {
                        Self::consume(code, &mut i)?;
                        PyToken {
                            kind: PyTokenType::NotEqual,
                            value: None,
                        }
                    } else {
                        return Err(PylentilError::InvalidCharacter);
                    }
                }
                b'~' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Tilde,
                        value: None,
                    }
                }
                b'&' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Ampersand,
                        value: None,
                    }
                }
                b'|' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::VerticalBar,
                        value: None,
                    }
                }
                b'^' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Caret,
                        value: None,
                    }
                }
                b'<' => {
                    Self::consume(code, &mut i)?;

                    match Self::peek(code, i) {
                        Ok(b'=') => {
                            Self::consume(code, &mut i)?;
                            PyToken {
                                kind: PyTokenType::LessEqual,
                                value: None,
                            }
                        }
                        Ok(b'<') => {
                            Self::consume(code, &mut i)?;
                            PyToken {
                                kind: PyTokenType::LShift,
                                value: None,
                            }
                        }
                        _ => PyToken {
                            kind: PyTokenType::Less,
                            value: None,
                        },
                    }
                }
                b'>' => {
                    Self::consume(code, &mut i)?;

                    match Self::peek(code, i) {
                        Ok(b'=') => {
                            Self::consume(code, &mut i)?;
                            PyToken {
                                kind: PyTokenType::GreaterEqual,
                                value: None,
                            }
                        }
                        Ok(b'<') => {
                            Self::consume(code, &mut i)?;
                            PyToken {
                                kind: PyTokenType::RShift,
                                value: None,
                            }
                        }
                        _ => PyToken {
                            kind: PyTokenType::Greater,
                            value: None,
                        },
                    }
                }
                b'\'' | b'"' => {
                    let quote = Self::consume(code, &mut i)?;
                    let start = i;
                    loop {
                        match Self::peek(code, i) {
                            Ok(b) if b == quote => break,
                            Ok(b'\\') => {
                                i += 1;
                                Self::consume(code, &mut i)?;
                            }
                            Ok(_) => i += 1,
                            Err(e) => return Err(e),
                        }
                    }
                    let value = &code[start..i];
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::String,
                        value: Some(Cow::Borrowed(value)),
                    }
                }
                b if b.is_ascii_digit() => {
                    if byte == b'0' {
                        Self::try_lex_number(code, &mut i)?
                    } else {
                        let start = i;
                        let mut returnable_token: PyToken;
                        Self::consume_while(code, &mut i, |b| b.is_ascii_digit());

                        if Self::peek(code, i) == Ok(b'.') {
                            Self::consume(code, &mut i)?; // Consume the '.'
                            Self::consume_while(code, &mut i, |b| b.is_ascii_digit()); // Consume fractional part
                            returnable_token = PyToken {
                                kind: PyTokenType::Float,
                                value: Some(Cow::Borrowed(&code[start..i])),
                            }
                        } else {
                            returnable_token = PyToken {
                                kind: PyTokenType::Int,
                                value: Some(Cow::Borrowed(&code[start..i])),
                            }
                        }

                        if Self::peek(code, i) == Ok(b'e') || Self::peek(code, i) == Ok(b'E') {
                            Self::consume(code, &mut i)?;
                            Self::consume_while(code, &mut i, |b| b.is_ascii_digit() || b == b'.' || b == b'-');

                            returnable_token = PyToken { 
                                kind: PyTokenType::ENotation, 
                                value: Some(Cow::Borrowed(&code[start..i]))
                            }
                        }

                        returnable_token
                    }
                }
                b if b.is_ascii_alphabetic() || b == b'_' => {
                    let value = Self::consume_while(code, &mut i, |b| {
                        b.is_ascii_alphanumeric() || b == b'_'
                    });

                    if let Some(kind) = keyword_type(value) {
                        PyToken { kind, value: None }
                    } else {
                        PyToken {
                            kind: PyTokenType::Ident,
                            value: Some(Cow::Borrowed(value)),
                        }
                    }
                }
                _ => return Err(PylentilError::InvalidCharacter),
            };

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

    fn try_lex_number(code: &'a str, pos: &mut usize) -> Result<PyToken<'a>, PylentilError> {
        // Dereference `pos` to capture the actual usize index at the start of the token
        let start = *pos;

        assert_eq!(code.as_bytes()[*pos], b'0');
        Self::consume(code, pos)?;
        let byte_type = Self::consume(code, pos)?;

        return match byte_type {
            b'b' => Self::lex_binary(code, pos),
            b'x' => Self::lex_hex(code, pos),
            b'o' => Self::lex_oct(code, pos),
            b if b.is_ascii_digit() => {
                // Pass `pos` directly instead of `&mut pos`
                Self::consume_while(code, pos, |b| b.is_ascii_digit());

                if Self::peek(code, *pos) == Ok(b'.') {
                    Self::consume(code, pos)?;
                    Self::consume_while(code, pos, |b| b.is_ascii_digit());

                    Ok(PyToken {
                        kind: PyTokenType::Float,
                        value: Some(Cow::Borrowed(&code[start..*pos])),
                    })
                } else {
                    Ok(PyToken {
                        kind: PyTokenType::Int,
                        value: Some(Cow::Borrowed(&code[start..*pos])),
                    })
                }
            },
            b' ' | b'\n' | b'\t' => Ok(PyToken { kind: PyTokenType::Int, value: Some(Cow::Borrowed("0")) }),
            _ => Err(PylentilError::InvalidCharacter),
        };
    }

    fn lex_binary(code: &'a str, pos: &mut usize) -> Result<PyToken<'a>, PylentilError> {
        let mut num: Vec<u8> = vec![];

        while *pos < code.len() {
            let char = Self::consume(code, pos)?;
            match char {
                b'0' | b'1' => num.push(char),
                _ => break,
            }
        }

        let Ok(parsed_string) = String::from_utf8(num) else {
            return Err(PylentilError::InvalidCharacter);
        };

        Ok(PyToken {
            kind: PyTokenType::Binary,
            value: Some(Cow::Owned(parsed_string)),
        })
    }

    fn lex_hex(code: &'a str, pos: &mut usize) -> Result<PyToken<'a>, PylentilError> {
        let mut num: Vec<u8> = vec![];

        while *pos < code.len() {
            let char = Self::consume(code, pos)?;
            match char {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'a'
                | b'b' | b'c' | b'd' | b'e' | b'f' => num.push(char),
                b'A' | b'B' | b'C' | b'D' | b'E' | b'F' => num.push(char.to_ascii_lowercase()),
                _ => break,
            }
        }

        let Ok(parsed_string) = String::from_utf8(num) else {
            return Err(PylentilError::InvalidCharacter);
        };

        Ok(PyToken {
            kind: PyTokenType::Hexadecimal,
            value: Some(Cow::Owned(parsed_string)),
        })
    }

    fn lex_oct(code: &'a str, pos: &mut usize) -> Result<PyToken<'a>, PylentilError> {
        let mut num: Vec<u8> = vec![];

        while *pos < code.len() {
            let char = Self::consume(code, pos)?;
            match char {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' => {
                    num.push(char.to_ascii_lowercase())
                }
                _ => break,
            }
        }

        let Ok(parsed_string) = String::from_utf8(num) else {
            return Err(PylentilError::InvalidCharacter);
        };

        Ok(PyToken {
            kind: PyTokenType::Octal,
            value: Some(Cow::Owned(parsed_string)),
        })
    }

    fn indent_pass(&self) -> Result<Self, PylentilError> {
        if self.tokens[0].kind == PyTokenType::Indent {
            return Err(PylentilError::InvalidIndentation);
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

        if indent != *indents.last().unwrap() {
            Err(PylentilError::InvalidIndentation)
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

fn has_all_same_chars(s: &str) -> bool {
    HashSet::<char>::from_iter(s.chars()).len() == 1
}

fn get_indent_size(s: &str) -> Result<usize, PylentilError> {
    let mut size = 0usize;
    for char in s.chars() {
        size += match char {
            '\t' => 4,
            ' ' => 1,
            _ => return Err(PylentilError::InvalidCharacter),
        }
    }
    Ok(size)
}
