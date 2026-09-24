use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType,
    ast::{PyModule, PyStatement},
    lookups::parse_statement,
    token::describe_any_of,
};

#[derive(Debug)]
pub struct PyParser<'a> {
    pub tokens: Vec<PyToken<'a>>,
    pos: usize,
}

impl<'a> PyParser<'a> {
    pub fn new(tokens: Vec<PyToken<'a>>) -> Self {
        PyParser {
            tokens: Self::remove_whitespaces(tokens),
            pos: 0,
        }
    }

    fn remove_whitespaces(tokens: Vec<PyToken<'a>>) -> Vec<PyToken<'a>> {
        tokens
            .iter()
            .filter(|&token| token.kind != PyTokenType::Whitespace)
            .cloned()
            .collect()
    }

    pub fn parse(&mut self) -> Result<PyModule, PylentilError> {
        let mut body: Vec<PyStatement> = Vec::new();

        while self.has_tokens() {
            self.skip_newlines()?;
            if !self.has_tokens() {
                break;
            }

            match parse_statement(self) {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(e)
            };

            self.skip_statement_separators()?;
        }

        self.expect_type(vec![PyTokenType::EOF])?;

        Ok(PyModule {
            body,
            type_ignores: Vec::new(),
        })
    }

    pub fn peek(&self) -> Result<PyToken<'a>, PylentilError> {
        if self.pos >= self.tokens.len() {
            return Err(PylentilError::EndOfFileReached);
        }
        Ok(self.tokens[self.pos].clone())
    }

    pub fn peek_ahead(&self) -> Result<PyToken<'a>, PylentilError> {
        if self.pos >= self.tokens.len() || self.pos + 1 >= self.tokens.len() {
            return Err(PylentilError::PeekAheadFailed);
        }

        Ok(self.tokens[self.pos + 1].clone())

    }

    pub fn consume(&mut self) -> Result<PyToken<'a>, PylentilError> {
        let token = self.peek()?;
        self.pos += 1;
        Ok(token)
    }

    pub fn has_tokens(&self) -> bool {
        match self.pos < self.tokens.len() {
            false => false,
            true => match self.peek() {
                Ok(PyToken {
                    kind: PyTokenType::EOF,
                    ..
                }) => false,
                _ => true,
            },
        }
    }

    pub fn expect_type(&mut self, token_type: Vec<PyTokenType>) -> Result<PyToken<'_>, PylentilError> {
        let found = self.peek()?;
        match token_type.contains(&found.kind) {
            true => Ok(self.consume()?),
            false => Err(PylentilError::UnexpectedToken {
                expected: describe_any_of(&token_type),
                found: found.describe(),
            }),
        }
    }

    pub fn skip_any_number_of(&mut self, token_type: PyTokenType) -> Result<(), PylentilError> {
        while self.peek()?.kind == token_type {
            self.consume()?;
        }
        Ok(())
    }

    pub fn skip_newlines(&mut self) -> Result<(), PylentilError> {
        self.skip_any_number_of(PyTokenType::Newline)
    }

    /// Skips whatever separates one statement from the next: newlines and
    /// semicolons, in any combination (`a = 1; b = 2`, `a = 1;`).
    pub fn skip_statement_separators(&mut self) -> Result<(), PylentilError> {
        while matches!(
            self.peek()?.kind,
            PyTokenType::Newline | PyTokenType::Semicolon
        ) {
            self.consume()?;
        }
        Ok(())
    }

    pub fn optional_skip_one(&mut self, token_type: PyTokenType) -> Result<(), PylentilError> {
        if self.peek()?.kind == token_type {
            self.consume()?;
        }

        Ok(())
    }
}
