use pylentil_linter::rules::pylint::{self_assigning_variable::SelfAssigningVariable, unreachable::Unreachable, useless_return::UselessReturn};

use crate::rules::test_rule;

#[test]
fn test_useless_return() {
    test_rule(&UselessReturn);
}

#[test]
fn test_unreachable() {
    test_rule(&Unreachable);
}

#[test]
fn test_self_assigning_variable() {
    test_rule(&SelfAssigningVariable);
}