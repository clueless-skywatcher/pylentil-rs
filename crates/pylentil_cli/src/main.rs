use std::{fs, path::PathBuf};

use pylentil_ast::{PyLexer, parser::PyParser};
use pylentil_common::errors::PylentilError;
use pylentil_linter::lint_config::PylentilBuilder;

const SOURCE: &str = "fixtures/somefile.py";

fn main() -> Result<(), PylentilError> {
    let contents = fs::read_to_string(SOURCE).map_err(|e| PylentilError::IOFailed {
        path: SOURCE.to_string(),
        reason: e.to_string(),
    })?;

    let mut parser = PyParser::new(&contents, PathBuf::from(SOURCE))?;
    let lint = PylentilBuilder::new()
        .with_lint("PYC-E722".to_string())
        .with_path(PathBuf::from(SOURCE))
        .build()?;

    lint.get_violation_report(&parser.parse()?);

    Ok(())
}
