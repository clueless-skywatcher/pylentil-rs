#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyRefContext {
    Load,
    Store,
    Delete,
    Unspecified,
}
