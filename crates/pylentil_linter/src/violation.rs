use std::path::PathBuf;

use crate::lint::Lint;

pub struct LintViolation {
    pub path: PathBuf,
    pub check_violated: Box<dyn Lint>
}