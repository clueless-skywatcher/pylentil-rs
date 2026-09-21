use std::fs;

use pylentil_ast::{PyLexer, parser::PyParser};
use pylentil_common::errors::PylentilError;

const SOURCE: &str = "fixtures/somefile.py";

fn main() -> Result<(), PylentilError> {
    let contents = fs::read_to_string(SOURCE).map_err(|e| PylentilError::IOFailed {
        path: SOURCE.to_string(),
        reason: e.to_string(),
    })?;

    match PyLexer::from_code(&contents) {
        Ok(lexer) => {
            let mut parser = PyParser::new(lexer.tokens);
            println!("{:#?}", parser.parse());
            Ok(())
        },
        Err(e) => Err(e)
    }

}
