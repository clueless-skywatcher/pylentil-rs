pub enum PyConstant {
    Integer(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Boolean(bool),
    Complex(Box<PyConstant>, Box<PyConstant>),
    None,
    Ellipsis,
}
