use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use pylentil_common::errors::PylentilError;

use crate::{
    lint::Lint,
    linter::Pylentil,
    registry::LintRegistry,
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
                Err(e) => {
                    return Err(PylentilError::IOFailed {
                        path: path.to_string_lossy().to_string(),
                        reason: e.to_string(),
                    });
                }
            }
        }

        Ok(Pylentil { lints, paths })
    }
}
