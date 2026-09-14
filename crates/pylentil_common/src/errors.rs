#[derive(Debug, PartialEq, Eq)]
pub enum PylentilError {
    EndOfFileReached,
    PeekAheadFailed,
    InvalidCharacter,
    FileNotFound,
    InvalidSyntax,
    IOFailed,
    NotATerminal,
    MixedSpacesAndTabs,
    ExpectedValue
}