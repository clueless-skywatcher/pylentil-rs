use pylentil_common::errors::PylentilError;

use crate::{PyTokenType, ast::PyStatement, parser::PyParser};

use super::{PyBindingPower, parse_expr, parse_statement};

pub(super) fn parse_stmt_if(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::If])?;
    let test = parse_expr(parser, PyBindingPower::Default)?;
    parser.expect_type(vec![PyTokenType::Colon])?;
    parser.skip_newlines()?;
    parser.expect_type(vec![PyTokenType::Indent])?;

    let mut body: Vec<PyStatement> = Vec::new();
    while parser.has_tokens() && parser.peek()?.kind != PyTokenType::Dedent {
        body.push(parse_statement(parser)?);
        parser.skip_newlines()?;
    }
    parser.expect_type(vec![PyTokenType::Dedent])?;
    parser.skip_newlines()?;

    let mut orelse: Vec<PyStatement> = Vec::new();
    if parser.peek()?.kind == PyTokenType::Else {
        parser.consume()?;
        parser.expect_type(vec![PyTokenType::Colon])?;
        parser.skip_newlines()?;
        parser.expect_type(vec![PyTokenType::Indent])?;
        while parser.has_tokens() && parser.peek()?.kind != PyTokenType::Dedent {
            orelse.push(parse_statement(parser)?);
            parser.skip_newlines()?;
        }

        parser.expect_type(vec![PyTokenType::Dedent])?;
        parser.skip_newlines()?;
    }

    Ok(PyStatement::If { test, body, orelse })
}
