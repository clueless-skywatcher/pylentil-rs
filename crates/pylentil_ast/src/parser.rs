use std::path::PathBuf;

use pylentil_common::{errors::PylentilError, span::PySpan};

use crate::{
    PyLexer, PyToken, PyTokenType, ast::{PyModule, PyStatement}, code::{PyCode, PyCodeBuilder, PyCodeMetadata, PyCodeMetadataBuilder}, lookups::parse_statement, token::describe_any_of,
};

#[derive(Debug)]
pub struct PyParser<'a> {
    pub tokens: Vec<PyToken<'a>>,
    pub lexer: PyLexer<'a>,
    code: &'a str,
    path: PathBuf,
    pos: usize,
}

impl<'a> PyParser<'a> {
    pub fn new(contents: &'a str, path: PathBuf) -> Result<Self, PylentilError> {
        match PyLexer::from_code(&contents) {
            Ok(lexer) => Ok(PyParser {
                code: contents,
                lexer: lexer.clone(),
                path,
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

    pub fn parse(&mut self) -> Result<PyCode<'a>, PylentilError> {
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

    /// Where the next token starts, for recording the beginning of a node.
    pub fn start(&self) -> Result<usize, PylentilError> {
        Ok(self.peek()?.span.start)
    }

    /// The span from `start` to the end of the last consumed token. Layout
    /// tokens (newlines, semicolons, indents, dedents) are passed over, so a
    /// block ends where its last statement does.
    pub fn span_from(&self, start: usize) -> PySpan {
        let end = self.tokens[..self.pos]
            .iter()
            .rev()
            .find(|token| {
                !matches!(
                    token.kind,
                    PyTokenType::Newline
                        | PyTokenType::Semicolon
                        | PyTokenType::Indent
                        | PyTokenType::Dedent
                        | PyTokenType::EOF
                )
            })
            .map_or(start, |token| token.span.end_or_start());

        PySpan::span(start, end.max(start))
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

    fn extract_metadata(&self) -> Result<PyCodeMetadata<'a>, PylentilError> {
        let metadata = PyCodeMetadataBuilder::new();
        let mut line_starts: Vec<usize> = vec![];
        let mut pos: usize = 0;
        let mut code_lines = self
            .code
            .split(&['\n'])
            .map(|line| {
                line_starts.push(pos);
                pos += line.len() + 1;
                line.to_string()
            })
            .collect::<Vec<String>>();

        Ok(metadata
            .add_raw_lines(&mut code_lines)
            .set_path(self.path.clone())
            .set_line_starts(line_starts)
            .set_comments(self.lexer.comments.clone())
            .build()?)
    }
}
