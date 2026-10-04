use pylentil_linter::rules::pylint::{unreachable::Unreachable, useless_return::UselessReturn};

use crate::rules::test_rule;

#[test]
fn test_useless_return() {
    test_rule(&UselessReturn);
}

#[test]
fn test_unreachable() {
    test_rule(&Unreachable);
}