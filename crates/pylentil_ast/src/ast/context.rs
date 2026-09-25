#[derive(Debug, Clone, Copy)]
pub enum PyRefContext {
    Load,
    Store,
    Delete,
    Unspecified
}
