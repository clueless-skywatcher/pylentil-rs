pub type PyConstantBox = Box<PyConstant>;

#[derive(Debug, Clone)]
pub enum PyConstant {
    Integer(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Boolean(bool),
    Complex(PyConstantBox, PyConstantBox),
    None,
    Ellipsis,
}
