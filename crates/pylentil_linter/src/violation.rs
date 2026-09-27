use crate::lint::Lint;

pub struct LintViolation {
    pub check_violated: Box<dyn Lint>
}