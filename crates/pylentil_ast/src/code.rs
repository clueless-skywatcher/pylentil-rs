use std::path::{PathBuf};

use pylentil_common::errors::PylentilError;

use crate::{ast::PyModule, lexer::PyCommentSpan};

pub struct PyCodeMetadataBuilder<'a> {
    raw_lines: Vec<String>,
    path: Option<PathBuf>,
    line_starts: Vec<usize>,
    comments: Vec<PyCommentSpan<'a>>
}

impl <'a> PyCodeMetadataBuilder<'a> {
    pub fn new() -> Self {
        PyCodeMetadataBuilder {
            raw_lines: vec![],
            path: None,
            line_starts: vec![],
            comments: vec![]
        }
    }

    pub fn add_raw_line(mut self, line: String) -> Self {
        self.raw_lines.push(line);
        self
    }

    pub fn add_raw_lines(mut self, lines: &mut Vec<String>) -> Self {
        self.raw_lines.append(lines);
        self
    }

    pub fn set_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn set_line_starts(mut self, line_starts: Vec<usize>) -> Self {
        self.line_starts = line_starts;
        self
    }

    pub fn set_comments(mut self, comments: Vec<PyCommentSpan<'a>>) -> Self {
        self.comments = comments;
        self
    }

    pub fn build(self) -> Result<PyCodeMetadata<'a>, PylentilError> {
        if self.path.is_none() {
            return Err(PylentilError::EmptyFile);
        }
        if self.raw_lines.len() == 0 {
            return Err(PylentilError::EmptyFile);
        }

        Ok(PyCodeMetadata {
            raw_lines: self.raw_lines,
            path: self.path.unwrap(),
            line_starts: self.line_starts,
            comments: self.comments
        })
    }
}

#[derive(Debug)]
pub struct PyCodeMetadata<'a> {
    pub raw_lines: Vec<String>,
    pub path: PathBuf,
    pub line_starts: Vec<usize>,
    pub comments: Vec<PyCommentSpan<'a>>,
}

pub struct PyCodeBuilder<'a> {
    ast: Option<PyModule>,
    metadata: Option<PyCodeMetadata<'a>>,
}

impl <'a> PyCodeBuilder<'a> {
    pub fn new() -> Self {
        PyCodeBuilder {
            ast: None,
            metadata: None,
        }
    }

    pub fn with_ast(mut self, ast: PyModule) -> Self {
        self.ast = Some(ast);
        self
    }

    pub fn with_metadata(mut self, metadata: PyCodeMetadata<'a>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<PyCode<'a>, PylentilError> {
        let ast = self.ast.ok_or(PylentilError::MissingAST)?;
        let metadata = self.metadata.ok_or(PylentilError::MissingMetadata)?;

        Ok(PyCode { ast, metadata })
    }
}

#[derive(Debug)]
pub struct PyCode<'a> {
    pub ast: PyModule,
    pub metadata: PyCodeMetadata<'a>,
}

impl <'a> PyCode<'a> {
    pub fn get_line_and_byte_offset(&self, pos: usize) -> (usize, usize) {
        let line_starts = self.metadata.line_starts.clone();
        let mut i = line_starts.len() - 1;
        while i > 0 {
            if line_starts[i] <= pos {
                break;
            }
            i -= 1;
        }

        (i + 1, pos - line_starts[i] + 1)
    }
}
