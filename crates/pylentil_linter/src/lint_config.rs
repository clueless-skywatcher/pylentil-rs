use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use pylentil_ast::{ast::{PyModule, PyStatement}, code::PyCode};
use pylentil_common::errors::PylentilError;

use crate::{lint::Lint, registry::LintRegistry, rules::{pycodestyle::bare_except::BareExcept, pylint::useless_return::UselessReturn}, violation::{self, LintViolation}};

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
            match statement {
                PyStatement::FunctionDef { .. } => {
                    if self.rule_enabled(&UselessReturn) {
                        violations.append(&mut UselessReturn.check(path, statement));
                    }
                },
                PyStatement::ClassDef { .. } => todo!(),
                PyStatement::Return { .. } => todo!(),
                PyStatement::Delete { .. } => todo!(),
                PyStatement::Assign { .. } => todo!(),
                PyStatement::TypeAlias { .. } => todo!(),
                PyStatement::AugAssign { .. } => todo!(),
                PyStatement::AnnAssign { .. } => todo!(),
                PyStatement::For { .. } => todo!(),
                PyStatement::AsyncFor { .. } => todo!(),
                PyStatement::While { .. } => todo!(),
                PyStatement::If { .. } => todo!(),
                PyStatement::With { .. } => todo!(),
                PyStatement::AsyncWith { .. } => todo!(),
                PyStatement::Match { .. } => todo!(),
                PyStatement::Raise { .. } => todo!(),
                PyStatement::Try { .. } => {
                    if self.rule_enabled(&BareExcept) {
                        violations.append(&mut BareExcept.check(path, statement));
                    }
                },
                PyStatement::TryStar { .. } => todo!(),
                PyStatement::Assert { .. } => todo!(),
                PyStatement::Import { .. } => todo!(),
                PyStatement::ImportFrom { .. } => todo!(),
                PyStatement::Global { .. } => todo!(),
                PyStatement::Nonlocal { .. } => todo!(),
                PyStatement::Expr { .. } => todo!(),
                PyStatement::Pass => todo!(),
                PyStatement::Break => todo!(),
                PyStatement::Continue => todo!(),
            }
        }

        violations
    }

    fn rule_enabled(&self, lint: &(dyn Lint + 'static)) -> bool {
        self.lints.contains(lint)
    }

    pub fn get_violation_report(&self, code: &PyCode) {
        println!("--------------------------------------------------------------------");
        println!("{:^68}", code.metadata.path.display());
        println!("--------------------------------------------------------------------");
        let violations = self.check(code);
        for LintViolation { path, check_violated } in violations {
            println!("{} - {}: {}", path.to_str().unwrap(), check_violated.code(), check_violated.message());
            println!("\tPossible fix: {}", check_violated.possible_fix().unwrap_or("None".to_string()));
        }
    } 

}
