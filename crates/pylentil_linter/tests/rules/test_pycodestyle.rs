use pylentil_linter::rules::pycodestyle::{
    bare_except::BareExcept, none_comparison::NoneComparison,
    true_false_comparison::TrueFalseComparison,
};

use crate::rules::test_rule;

#[test]
fn test_bare_except() {
    test_rule(&BareExcept);
}

#[test]
fn test_none_comparison() {
    test_rule(&NoneComparison);
}

#[test]
fn test_true_false_comparison() {
    test_rule(&TrueFalseComparison);
}
