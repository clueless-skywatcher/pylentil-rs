use std::path::{PathBuf};

use pylentil_common::errors::PylentilError;

use crate::ast::PyModule;

pub struct PyCodeMetadataBuilder {
    raw_lines: Vec<String>,
    path: Option<PathBuf>,
}

impl PyCodeMetadataBuilder {
    pub fn new() -> Self {
        PyCodeMetadataBuilder {
            raw_lines: vec![],
            path: None,
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

    pub fn build(self) -> Result<PyCodeMetadata, PylentilError> {
        if self.path.is_none() {
            return Err(PylentilError::EmptyFile);
        }
        if self.raw_lines.len() == 0 {
            return Err(PylentilError::EmptyFile);
        }

        Ok(PyCodeMetadata {
            raw_lines: self.raw_lines,
            path: self.path.unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct PyCodeMetadata {
    pub raw_lines: Vec<String>,
    pub path: PathBuf,
}

pub struct PyCodeBuilder {
    ast: Option<PyModule>,
    metadata: Option<PyCodeMetadata>,
}

impl PyCodeBuilder {
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

    pub fn with_metadata(mut self, metadata: PyCodeMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<PyCode, PylentilError> {
        let ast = self.ast.ok_or(PylentilError::MissingAST)?;
        let metadata = self.metadata.ok_or(PylentilError::MissingMetadata)?;

        Ok(PyCode { ast, metadata })
    }
}

#[derive(Debug)]
pub struct PyCode {
    pub ast: PyModule,
    pub metadata: PyCodeMetadata,
}
