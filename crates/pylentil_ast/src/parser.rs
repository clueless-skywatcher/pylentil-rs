use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType, ast::{PyModule, PyStatement}, lookups::parse_statement,
};

pub struct PyParser<'a> {
    pub tokens: Vec<PyToken<'a>>,
    pos: usize
}

impl <'a> PyParser<'a> {
    pub fn new(tokens: Vec<PyToken<'a>>) -> Self {
        PyParser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<PyModule, PylentilError> {
        let mut body: Vec<PyStatement> = Vec::new();

        while self.has_tokens() {
            let Ok(stmt) = parse_statement(self) else {
                return Err(PylentilError::InvalidSyntax);
            };

            body.push(stmt);
        }        

        Ok(PyModule {
            body,
            type_ignores: Vec::new(),
        })
    }

    pub fn peek(&self) -> Result<PyToken<'a>, PylentilError> {
        if self.pos >= self.tokens.len() {
            return Err(PylentilError::EndOfFileReached);
        }
        Ok(self.tokens[self.pos])
    }

    pub fn consume(&mut self) -> Result<PyToken<'a>, PylentilError> {
        let token = self.peek()?;
        self.pos += 1;
        Ok(token)
    }

    fn has_tokens(&self) -> bool {
        match self.pos < self.tokens.len() {
            false => false,
            true => match self.peek() {
                Ok(PyToken { kind: PyTokenType::EOF, .. }) => false,
                _ => true
            }
        }
    }

    pub fn expect(&mut self, token_type: PyTokenType) -> Result<(), PylentilError> {
        match self.peek()?.kind == token_type {
            true => Ok(()),
            false => Err(PylentilError::InvalidSyntax)
        }            
    }
}