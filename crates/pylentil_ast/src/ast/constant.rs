pub type PyConstantBox = Box<PyConstant>;

#[derive(Debug, Clone, PartialEq)]
pub enum PyConstant {
    Integer(String),
    Float(String),
    String(String),
    Bytes(Vec<u8>),
    Boolean(bool),
    Complex(PyConstantBox, PyConstantBox),
    None,
    Ellipsis,
}
