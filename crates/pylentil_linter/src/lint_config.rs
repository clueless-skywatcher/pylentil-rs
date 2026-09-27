use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use pylentil_ast::{ast::{PyModule, PyStatement}, code::PyCode};
use pylentil_common::errors::PylentilError;

use crate::{lint::Lint, registry::LintRegistry, violation::LintViolation};

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

    pub fn build(self) -> Result<Pylentil, PylentilError> {
        let mut lints: HashSet<Arc<dyn Lint>> = HashSet::new();
        let mut paths: Vec<File> = vec![];

        for code in self.lint_codes.clone() {
            let lint = LintRegistry::get_instance().get_lint(code)?;
            lints.insert(lint);
        }

        for path in self.paths {
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
        self.check_ast(&code.ast)
    }

    fn check_ast(&self, ast: &PyModule) -> Vec<LintViolation> {
        for statement in ast.body.iter() {
            match statement {
                PyStatement::FunctionDef {
                    name,
                    args,
                    body,
                    decorator_list,
                    returns,
                    type_comment,
                    type_params,
                    is_async,
                } => todo!(),
                PyStatement::ClassDef {
                    name,
                    bases,
                    keywords,
                    body,
                    decorator_list,
                    type_params,
                } => todo!(),
                PyStatement::Return { value } => todo!(),
                PyStatement::Delete { targets } => todo!(),
                PyStatement::Assign {
                    targets,
                    value,
                    type_comment,
                } => todo!(),
                PyStatement::TypeAlias {
                    name,
                    type_params,
                    value,
                } => todo!(),
                PyStatement::AugAssign { target, op, value } => todo!(),
                PyStatement::AnnAssign {
                    target,
                    annotation,
                    value,
                    simple,
                } => todo!(),
                PyStatement::For {
                    target,
                    iter,
                    body,
                    orelse,
                    type_comment,
                } => todo!(),
                PyStatement::AsyncFor {
                    target,
                    iter,
                    body,
                    orelse,
                    type_comment,
                } => todo!(),
                PyStatement::While { test, body, orelse } => todo!(),
                PyStatement::If { test, body, orelse } => todo!(),
                PyStatement::With {
                    items,
                    body,
                    type_comment,
                } => todo!(),
                PyStatement::AsyncWith {
                    items,
                    body,
                    type_comment,
                } => todo!(),
                PyStatement::Match { subject, cases } => todo!(),
                PyStatement::Raise { exc, cause } => todo!(),
                PyStatement::Try {
                    body,
                    handlers,
                    orelse,
                    finalbody,
                } => {
                },
                PyStatement::TryStar {
                    body,
                    handlers,
                    orelse,
                    finalbody,
                } => todo!(),
                PyStatement::Assert { test, msg } => todo!(),
                PyStatement::Import { names } => todo!(),
                PyStatement::ImportFrom {
                    module,
                    names,
                    level,
                } => todo!(),
                PyStatement::Global { names } => todo!(),
                PyStatement::Nonlocal { names } => todo!(),
                PyStatement::Expr { value } => todo!(),
                PyStatement::Pass => todo!(),
                PyStatement::Break => todo!(),
                PyStatement::Continue => todo!(),
            }
        }

        vec![]
    }
}
