#[derive(Debug, PartialEq, Eq)]
pub enum PylentilError {
    EndOfFileReached,
    InvalidCharacter,
    FileNotFound,
    InvalidSyntax,
    IOFailed
}