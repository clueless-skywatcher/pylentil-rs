use pylentil_common::errors::PylentilError;

use crate::{PyToken, ast::{PyASTNode}};

pub struct PyParser;

impl PyParser {
    pub fn parse(tokens: Vec<PyToken>) -> PyASTNode {
        let body: Vec<Box<PyASTNode>> = Vec::new();

        PyASTNode::Module { body }
    }
}