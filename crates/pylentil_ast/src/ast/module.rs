use super::shared::PyTypeIgnore;
use super::stmt::PyStatement;

#[derive(Debug, PartialEq)]
pub struct PyModule {
    pub body: Vec<PyStatement>,
    pub type_ignores: Vec<PyTypeIgnore>,
}
