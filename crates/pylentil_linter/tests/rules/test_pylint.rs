use pylentil_linter::rules::pylint::useless_return::UselessReturn;

use crate::rules::test_rule;

#[test]
fn test_useless_return() {
    test_rule(&UselessReturn);
}