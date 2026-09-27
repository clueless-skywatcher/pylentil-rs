use pylentil_common::errors::PylentilError;

use crate::{
    PyLexer, PyToken, PyTokenType,
    ast::{PyModule, PyStatement},
    code::{PyCode, PyCodeBuilder, PyCodeMetadata, PyCodeMetadataBuilder},
    lookups::parse_statement,
    token::describe_any_of,
};

#[derive(Debug)]
pub struct PyParser<'a> {
    pub tokens: Vec<PyToken<'a>>,
    code: &'a str,
    pos: usize,
}

impl<'a> PyParser<'a> {
    pub fn new(contents: &'a str) -> Result<Self, PylentilError> {
        match PyLexer::from_code(&contents) {
            Ok(lexer) => Ok(PyParser {
                code: contents,
                tokens: Self::remove_whitespaces(lexer.tokens),
                pos: 0usize,
            }),
            Err(e) => Err(e),
        }
    }

    fn remove_whitespaces(tokens: Vec<PyToken<'a>>) -> Vec<PyToken<'a>> {
        tokens
            .iter()
            .filter(|&token| token.kind != PyTokenType::Whitespace)
            .cloned()
            .collect()
    }

    pub fn parse(&mut self) -> Result<PyCode, PylentilError> {
        let metadata = self.extract_metadata()?;

        let mut body: Vec<PyStatement> = Vec::new();

        while self.has_tokens() {
            self.skip_newlines()?;
            if !self.has_tokens() {
                break;
            }

            match parse_statement(self) {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(e),
            };

            self.skip_statement_separators()?;
        }

        self.expect_type(vec![PyTokenType::EOF])?;

        let code = PyCodeBuilder::new()
            .with_ast(PyModule {
                body,
                type_ignores: vec![],
            })
            .with_metadata(metadata)
            .build()?;

        Ok(code)
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

    pub fn expect_type(
        &mut self,
        token_type: Vec<PyTokenType>,
    ) -> Result<PyToken<'_>, PylentilError> {
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

    fn extract_metadata(&self) -> Result<PyCodeMetadata, PylentilError> {
        let metadata = PyCodeMetadataBuilder::new();
        let mut code_lines = self
            .code
            .split(&['\r', '\n'])
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        Ok(metadata
            .add_raw_lines(&mut code_lines)
            .build()?)
    }
}
