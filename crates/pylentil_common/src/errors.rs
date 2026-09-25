use std::fmt::{self};

/// Everything that can go wrong while turning Python source into an AST.
///
/// Variants carry enough context to render a sentence a user can act on.
/// They deliberately carry no positions yet: spans and line numbers are a
/// separate pass, so the payloads here describe *what* went wrong, never
/// *where*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PylentilError {
    // ------------------------------------------------------ input exhausted --
    /// The input ended while a construct still needed more of it.
    EndOfFileReached,
    /// A decision needed two tokens of lookahead but only one was left.
    PeekAheadFailed,

    // -------------------------------------------------------------- lexing --
    /// A byte that starts no operator, literal or identifier.
    UnknownCharacter { character: char },
    /// A string literal that runs to the end of the file without its
    /// closing quote.
    UnterminatedString { quote: String },
    /// A single-quoted string interrupted by a line break.
    UnterminatedStringLine { quote: String },
    /// A radix literal such as `0x` or `0b__` with no digits after the prefix.
    EmptyNumericLiteral { prefix: String, base: &'static str },
    /// One indent built from both tabs and spaces.
    MixedSpacesAndTabs,
    /// Something other than a tab or a space at the start of an indented line.
    InvalidIndentationCharacter { character: char },
    /// The very first line of the file is indented.
    UnexpectedIndent,
    /// A dedent landing between two indentation levels instead of on one that
    /// is still open.
    InconsistentDedent { found: usize, enclosing: usize },

    // ------------------------------------------------------------- parsing --
    /// A token of the wrong kind where the grammar allowed only a known set.
    UnexpectedToken { expected: String, found: String },
    /// A token that cannot begin an expression.
    ExpressionExpected { found: String },
    /// A token that cannot continue the expression parsed so far.
    TokenCannotContinueExpression { found: String },
    /// A token routed to the terminal parser that is not a literal or a name.
    NotATerminal { found: String },
    /// A literal token the lexer left without its text.
    TokenMissingValue { kind: &'static str },
    /// A token used as an operator in a position that has no such operator.
    UnsupportedOperator { found: String, context: &'static str },
    /// The left-hand side of an assignment is not something that can be
    /// assigned to.
    InvalidAssignmentTarget { found: &'static str },
    /// The left-hand side of an annotated assignment is not a plain name.
    InvalidAnnotationTarget { found: &'static str },
    /// The annotation itself is not a plain name.
    InvalidAnnotation { found: &'static str },
    /// The part after a `.` is not a name.
    InvalidAttributeName { found: &'static str },
    /// The part before the `=` of a keyword argument is not a name.
    InvalidKeywordArgumentName { found: &'static str },
    /// A positional argument after a keyword argument in the same call.
    PositionalArgumentAfterKeyword,
    /// Invalid argument type
    InvalidArgumentType,

    // -------------------------------------------------------------- driver --
    /// The source file could not be read.
    IOFailed { path: String, reason: String },

    /// The code path has not been implemented yet
    NotImplemented
}

impl fmt::Display for PylentilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PylentilError::EndOfFileReached => write!(
                f,
                "reached the end of the input while a construct was still unfinished"
            ),
            PylentilError::PeekAheadFailed => write!(
                f,
                "needed to look one token past the end of the input to decide how to parse"
            ),

            PylentilError::UnknownCharacter { character } => write!(
                f,
                "`{character}` does not start any operator, literal or identifier"
            ),
            PylentilError::UnterminatedString { quote } => write!(
                f,
                "the string opened with {quote} is never closed before the end of the file"
            ),
            PylentilError::UnterminatedStringLine { quote } => write!(
                f,
                "the string opened with {quote} is broken by a line break; close it on the same \
                 line, escape the break with `\\`, or use a triple-quoted string"
            ),
            PylentilError::EmptyNumericLiteral { prefix, base } => write!(
                f,
                "the {base} literal `{prefix}` has no digits after its prefix"
            ),
            PylentilError::MixedSpacesAndTabs => write!(
                f,
                "this indent mixes tabs and spaces; indent a line with one or the other, not both"
            ),
            PylentilError::InvalidIndentationCharacter { character } => write!(
                f,
                "indentation may only contain spaces and tabs, found `{}`",
                character.escape_debug()
            ),
            PylentilError::UnexpectedIndent => write!(
                f,
                "the first line of the file is indented, but nothing encloses it"
            ),
            PylentilError::InconsistentDedent { found, enclosing } => write!(
                f,
                "this line is dedented to width {found}, which matches no open block; the \
                 enclosing block is at width {enclosing}"
            ),

            PylentilError::UnexpectedToken { expected, found } => {
                write!(f, "expected {expected}, found {found}")
            }
            PylentilError::ExpressionExpected { found } => {
                write!(f, "expected an expression, found {found}")
            }
            PylentilError::TokenCannotContinueExpression { found } => write!(
                f,
                "{found} cannot continue the expression that comes before it"
            ),
            PylentilError::NotATerminal { found } => write!(
                f,
                "expected a literal or a name, found {found}"
            ),
            PylentilError::TokenMissingValue { kind } => write!(
                f,
                "the lexer produced {kind} without any text attached to it"
            ),
            PylentilError::UnsupportedOperator { found, context } => {
                write!(f, "{found} is not an operator allowed in {context}")
            }
            PylentilError::InvalidAssignmentTarget { found } => write!(
                f,
                "cannot assign to {found}; a target must be a name, an attribute, a subscript, or \
                 a list or tuple of those"
            ),
            PylentilError::InvalidAnnotationTarget { found } => write!(
                f,
                "cannot annotate {found}; only a plain name may carry an annotation"
            ),
            PylentilError::InvalidAnnotation { found } => {
                write!(f, "expected a type name as the annotation, found {found}")
            }
            PylentilError::InvalidAttributeName { found } => {
                write!(f, "expected an attribute name after `.`, found {found}")
            }
            PylentilError::InvalidKeywordArgumentName { found } => write!(
                f,
                "expected a parameter name before `=` in this call, found {found}"
            ),
            PylentilError::PositionalArgumentAfterKeyword => write!(
                f,
                "this positional argument follows a keyword argument; positional arguments must \
                 all come first"
            ),

            PylentilError::IOFailed { path, reason } => {
                write!(f, "could not read `{path}`: {reason}")
            },
            PylentilError::NotImplemented => {
                write!(f, "Has not been implemented")
            },
            PylentilError::InvalidArgumentType => {
                write!(f, "Invalid argument type")
            }
        }
    }
}

impl std::error::Error for PylentilError {}
