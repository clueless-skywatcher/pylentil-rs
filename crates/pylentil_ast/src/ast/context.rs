#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PyRefContext {
    Load,
    Store,
    Delete,
    Unspecified
}
