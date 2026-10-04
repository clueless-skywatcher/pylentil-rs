use pylentil_linter::rules::pylint::{binary_op_exception::BinaryOpException, self_assigning_variable::SelfAssigningVariable, unreachable::Unreachable, useless_return::UselessReturn, wildcard_import::WildcardImport};

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

#[test]
fn test_wildcard_import() {
    test_rule(&WildcardImport);
}

#[test]
fn test_binary_op_exception() {
    test_rule(&BinaryOpException);
}