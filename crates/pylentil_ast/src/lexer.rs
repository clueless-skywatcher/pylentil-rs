use pylentil_common::errors::PylentilError;

use crate::token::{PyToken, PyTokenType};

pub struct PyLexer<'a> {
    pub code: &'a str,
    pub tokens: Vec<PyToken>,
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

    fn consume_while(code: &str, pos: &mut usize, pred: impl Fn(u8) -> bool) -> String {
        let start = *pos;
        while let Ok(b) = Self::peek(code, *pos) {
            if !pred(b) {
                break;
            }
            *pos += 1;
        }
        code[start..*pos].to_string()
    }

    pub fn from_code(code: &'a str) -> Result<Self, PylentilError> {
        let mut tokens = Vec::<PyToken>::new();
        let mut i = 0;

        while i < code.len() {
            let byte = Self::peek(code, i)?;

            let token = match byte {
                b' ' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Whitespace,
                        value: None,
                    }
                }
                b'\t' => {
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::Indent,
                        value: None,
                    }
                }
                b'\n' => {
                    Self::consume(code, &mut i)?;
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
                    let value = code[start..i].to_string();
                    Self::consume(code, &mut i)?;
                    PyToken {
                        kind: PyTokenType::String,
                        value: Some(value),
                    }
                }
                b if b.is_ascii_digit() => {
                    let int_part = Self::consume_while(code, &mut i, |b| b.is_ascii_digit());
                    if Self::peek(code, i) == Ok(b'.') {
                        Self::consume(code, &mut i)?;
                        let frac = Self::consume_while(code, &mut i, |b| b.is_ascii_digit());
                        PyToken {
                            kind: PyTokenType::Float,
                            value: Some(format!("{int_part}.{frac}")),
                        }
                    } else {
                        PyToken {
                            kind: PyTokenType::Int,
                            value: Some(int_part),
                        }
                    }
                }
                b if b.is_ascii_alphabetic() || b == b'_' => {
                    let value = Self::consume_while(code, &mut i, |b| {
                        b.is_ascii_alphanumeric() || b == b'_'
                    });
                    PyToken {
                        kind: PyTokenType::KeywordOrIdent,
                        value: Some(value),
                    }
                }
                _ => return Err(PylentilError::InvalidCharacter),
            };

            tokens.push(token);
        }

        tokens.push(PyToken {
            kind: PyTokenType::EOF,
            value: None,
        });

        Ok(PyLexer { code, tokens })
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
            .map(|x| x.clone())
            .collect();

        PyLexer {
            code: self.code,
            tokens,
        }
    }
}
