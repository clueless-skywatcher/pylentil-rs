use std::fs;

use pylentil_ast::PyLexer;
use pylentil_common::errors::PylentilError;

fn main() -> Result<(), PylentilError> {
    let Ok(contents) = fs::read_to_string("fixtures/somefile.py") else {
        return Err(PylentilError::IOFailed);
    };

    let Ok(lexer) = PyLexer::from_code(&contents) else {
        return Err(PylentilError::FileNotFound);
    };
    for token in &lexer.spaces_scrapped().tokens {
        println!("{token}");
    }
    Ok(())
}
