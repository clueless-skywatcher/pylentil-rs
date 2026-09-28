use std::path::PathBuf;

use pylentil_common::span::PySpan;

use crate::lint::Lint;

pub struct LintViolation {
    pub path: PathBuf,
    pub check_violated: Box<dyn Lint>,
    pub span: PySpan
}
