use std::{fs::File, path::PathBuf, sync::Arc};

use pylentil_common::errors::PylentilError;

use crate::{lint::Lint, registry::LintRegistry};

pub struct PylentilBuilder {
    paths: Vec<PathBuf>,
    lint_codes: Vec<String>,
}

impl PylentilBuilder {
    pub fn new() -> Self {
        PylentilBuilder {
            paths: vec![],
            lint_codes: vec![],
        }
    }

    pub fn with_path(&mut self, path: PathBuf) -> &mut Self {
        self.paths.push(path);
        self
    }

    pub fn with_lint(&mut self, lint_code: String) -> &mut Self {
        self.lint_codes.push(lint_code);
        self
    }

    pub fn build(self) -> Result<Pylentil, PylentilError> {
        let mut lints: Vec<Arc<dyn Lint>> = vec![];
        let mut paths: Vec<File> = vec![];

        for code in self.lint_codes.clone() {
            let lint = LintRegistry::get_instance().get_lint(code)?;
            lints.push(lint);
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
    pub lints: Vec<Arc<dyn Lint>>,
    pub paths: Vec<File>,
}
