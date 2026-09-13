use super::constant::PyConstant;
use super::expr::PyExpr;

pub type PyPatternBox = Box<PyPattern>;

#[derive(Debug)]
pub enum PyPattern {
    MatchValue {
        value: PyExpr,
    },
    MatchSingleton {
        value: PyConstant,
    },
    MatchSequence {
        patterns: Vec<PyPattern>,
    },
    MatchMapping {
        keys: Vec<PyExpr>,
        patterns: Vec<PyPattern>,
        rest: Option<String>,
    },
    MatchClass {
        cls: PyExpr,
        patterns: Vec<PyPattern>,
        kwd_attrs: Vec<String>,
        kwd_patterns: Vec<PyPattern>,
    },
    MatchStar {
        name: Option<String>,
    },
    MatchAs {
        pattern: Option<PyPatternBox>,
        name: Option<String>,
    },
    MatchOr {
        patterns: Vec<PyPattern>,
    },
}
