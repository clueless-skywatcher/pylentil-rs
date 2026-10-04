use core::fmt;
use core::hash::{Hash, Hasher};

/// Byte offsets into the source. `end` is exclusive, and `None` marks a
/// zero-width location such as a dedent or the end of the file.
#[derive(Debug, Clone, Copy, Default)]
pub struct PySpan {
    pub start: usize,
    pub end: Option<usize>,
}

impl PySpan {
    pub fn location(pos: usize) -> Self {
        PySpan {
            start: pos,
            end: None,
        }
    }

    pub fn span(start: usize, end: usize) -> Self {
        PySpan {
            start,
            end: Some(end),
        }
    }

    /// Where the span stops; a location stops where it starts.
    pub fn end_or_start(&self) -> usize {
        self.end.unwrap_or(self.start)
    }
}

impl PartialEq for PySpan {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for PySpan {}

impl Hash for PySpan {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl fmt::Display for PySpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.end {
            Some(end) => write!(f, "{}:{}", self.start, end),
            None => write!(f, "{}", self.start)
        }
    }
}
