use pylentil_linter::rules::pycodestyle::bare_except::BareExcept;

use crate::rules::test_rule;

#[test]
fn test_bare_except() {
    test_rule(&BareExcept);
}