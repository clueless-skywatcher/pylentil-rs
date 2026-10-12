use pylentil_lint_macros::create_pylentil_lint;

create_pylentil_lint! {
    WildcardImport,
    "PYL-W0401",
    pylint,
    style,
    warning,
    "Wildcard imports are expensive and can cause side effects to trigger, which can become difficult to debug"
}

impl Lint for WildcardImport {
    fn check(&self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        self.collect(&mut violations, path, stmt);
        violations
    }
}

impl WildcardImport {
    fn collect(&self, violations: &mut Vec<LintViolation>, path: &Path, stmt: &PyStatement) {
        if let PyStatement::ImportFrom(import_from) = stmt {
            if let [name] = import_from.names.as_slice() {
                if name.name == "*" {
                    self.report(violations, path, &name.span);
                }
            }
        }
    }
}
