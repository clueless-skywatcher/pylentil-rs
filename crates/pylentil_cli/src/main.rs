use std::fs;

use pylentil_ast::{PyLexer, parser::PyParser};
use pylentil_common::errors::PylentilError;

fn main() -> Result<(), PylentilError> {
    let Ok(contents) = fs::read_to_string("fixtures/somefile.py") else {
        return Err(PylentilError::IOFailed);
    };

    match PyLexer::from_code(&contents) {
        Ok(lexer) => {
            // for token in lexer.tokens.iter() {
            //     println!("{}", token);
            // }
            let mut parser = PyParser::new(lexer.tokens);
            println!("{:#?}", parser.parse());
            Ok(())
        },
        Err(e) => Err(e)
    }

}
