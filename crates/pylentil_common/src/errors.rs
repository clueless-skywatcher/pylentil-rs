#[derive(Debug, PartialEq, Eq)]
pub enum PylentilError {
    EndOfFileReached,
    PeekAheadFailed,
    InvalidCharacter,
    InvalidIndentation,
    FileNotFound,
    InvalidSyntax,
    IOFailed,
    NotATerminal,
    MixedSpacesAndTabs,
    ExpectedValue
}