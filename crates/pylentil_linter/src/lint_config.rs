use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use pylentil_ast::{
    ast::{PyModule, PyStatement},
    code::PyCode,
};
use pylentil_common::errors::PylentilError;

use crate::{
    lint::Lint,
    registry::LintRegistry,
    rules::{pycodestyle::bare_except::BareExcept, pylint::useless_return::UselessReturn},
    violation::{self, LintViolation},
};

pub struct PylentilBuilder {
    paths: HashSet<PathBuf>,
    lint_codes: HashSet<String>,
}

impl PylentilBuilder {
    pub fn new() -> Self {
        PylentilBuilder {
            paths: HashSet::new(),
            lint_codes: HashSet::new(),
        }
    }

    pub fn with_path(&mut self, path: PathBuf) -> &mut Self {
        self.paths.insert(path);
        self
    }

    pub fn with_lint(&mut self, lint_code: String) -> &mut Self {
        self.lint_codes.insert(lint_code);
        self
    }

    pub fn build(&self) -> Result<Pylentil, PylentilError> {
        let mut lints: HashSet<Arc<dyn Lint>> = HashSet::new();
        let mut paths: Vec<File> = vec![];

        for code in self.lint_codes.clone() {
            let lint = LintRegistry::get_instance().get_lint(code)?;
            lints.insert(lint);
        }

        for path in self.paths.iter() {
            match File::open(path.clone()) {
                Ok(file) => paths.push(file),
                Err(_) => {
                    return Err(PylentilError::IOFailed {
                        path: path.to_string_lossy().to_string(),
                        reason: String::from("File open failed"),
                    });
                }
            }
        }

        Ok(Pylentil { lints, paths })
    }
}

pub struct Pylentil {
    pub lints: HashSet<Arc<dyn Lint>>,
    pub paths: Vec<File>,
}

impl Pylentil {
    pub fn check(&self, code: &PyCode) -> Vec<LintViolation> {
        let mut violations = vec![];
        violations.append(&mut self.check_ast(&code.metadata.path, &code.ast));
        violations
    }

    fn check_ast(&self, path: &PathBuf, ast: &PyModule) -> Vec<LintViolation> {
        let mut violations = vec![];

        for statement in ast.body.iter() {
            violations.append(&mut self.check_statement(path, statement));
        }

        violations
    }

    fn rule_enabled(&self, lint: &(dyn Lint + 'static)) -> bool {
        self.lints.contains(lint)
    }

    fn check_statement(&self, path: &PathBuf, statement: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        match statement {
            PyStatement::FunctionDef { .. } => {
                violations.append(&mut self.check_funcdef(path, statement));
            }
            PyStatement::ClassDef { .. } => {}
            PyStatement::Return { .. } => {}
            PyStatement::Delete { .. } => {}
            PyStatement::Assign { .. } => {}
            PyStatement::TypeAlias { .. } => {}
            PyStatement::AugAssign { .. } => {}
            PyStatement::AnnAssign { .. } => {}
            PyStatement::For { .. } => {}
            PyStatement::AsyncFor { .. } => {}
            PyStatement::While { .. } => {}
            PyStatement::If { .. } => {}
            PyStatement::With { .. } => {}
            PyStatement::AsyncWith { .. } => {}
            PyStatement::Match { .. } => {}
            PyStatement::Raise { .. } => {}
            PyStatement::Try { .. } => {
                violations.append(&mut self.check_try(path, statement));
            }
            PyStatement::TryStar { .. } => {}
            PyStatement::Assert { .. } => {}
            PyStatement::Import { .. } => {}
            PyStatement::ImportFrom { .. } => {}
            PyStatement::Global { .. } => {}
            PyStatement::Nonlocal { .. } => {}
            PyStatement::Expr { .. } => {}
            PyStatement::Pass { .. } => {}
            PyStatement::Break { .. } => {}
            PyStatement::Continue { .. } => {}
        }

        violations
    }

    fn check_body(&self, path: &PathBuf, body: &[PyStatement]) -> Vec<LintViolation> {
        let mut violations = vec![];

        for statement in body {
            violations.append(&mut self.check_statement(path, statement));
        }

        violations
    }

    fn check_funcdef(&self, path: &PathBuf, funcdef: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&UselessReturn) {
            violations.append(&mut UselessReturn.check(path, &funcdef));
        }

        if let PyStatement::FunctionDef { body, .. } = &funcdef {
            violations.append(&mut self.check_body(path, body));
        }

        violations
    }

    fn check_try(&self, path: &PathBuf, try_stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&BareExcept) {
            violations.append(&mut BareExcept.check(path, &try_stmt));
        }

        if let PyStatement::Try {
            body,
            handlers,
            orelse,
            finalbody,
            ..
        } = &try_stmt
        {
            violations.append(&mut self.check_body(path, body));
            for handler in handlers {
                violations.append(&mut self.check_body(path, &handler.body));
            }
            violations.append(&mut self.check_body(path, orelse));
            violations.append(&mut self.check_body(path, finalbody));
        }

        violations
    }

    pub fn get_violation_report(&self, code: &PyCode) {
        println!("--------------------------------------------------------------------");
        println!("{:^68}", code.metadata.path.display());
        println!("--------------------------------------------------------------------");
        let violations = self.check(code);
        for LintViolation {
            path,
            check_violated,
        } in violations
        {
            println!(
                "{} - {}: {}",
                path.to_str().unwrap(),
                check_violated.code(),
                check_violated.message()
            );
            println!(
                "    Possible fix: {}",
                check_violated.possible_fix().unwrap_or("None".to_string())
            );
        }
    }
}
