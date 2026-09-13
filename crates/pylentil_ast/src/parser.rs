use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType,
    ast::{PyModule, PyStatement},
    lookups::parse_statement,
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
            .filter(|&token| token.kind != PyTokenType::Whitespace && token.kind != PyTokenType::Newline)
            .cloned()
            .collect()
    }

    pub fn parse(&mut self) -> Result<PyModule, PylentilError> {
        let mut body: Vec<PyStatement> = Vec::new();

        while self.has_tokens() {
            match parse_statement(self) {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(e)
            };
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
        match token_type.contains(&self.peek()?.kind) {
            true => Ok(self.consume()?),
            false => Err(PylentilError::InvalidSyntax),
        }
    }
}
