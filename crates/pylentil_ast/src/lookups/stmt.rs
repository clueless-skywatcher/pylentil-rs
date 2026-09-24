use pylentil_common::errors::PylentilError;

use crate::{PyTokenType, ast::PyStatement, parser::PyParser};

use super::{PyBindingPower, parse_expr, parse_statement};

pub(super) fn parse_stmt_if(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::If])?;
    parse_if_after_keyword(parser)
}

/// Parses `<test>: <block> [elif ... | else: <block>]`, i.e. everything after
/// an `if` or `elif` keyword. An `elif` chain becomes an `If` nested in the
/// `orelse` of its predecessor, which is how CPython's AST models it.
fn parse_if_after_keyword(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let test = parse_expr(parser, PyBindingPower::Default)?;
    parser.expect_type(vec![PyTokenType::Colon])?;
    let body = parse_block(parser)?;

    let orelse = match parser.peek()?.kind {
        PyTokenType::Elif => {
            parser.consume()?;
            vec![parse_if_after_keyword(parser)?]
        }
        PyTokenType::Else => {
            parser.consume()?;
            parser.expect_type(vec![PyTokenType::Colon])?;
            parse_block(parser)?
        }
        _ => Vec::new(),
    };

    Ok(PyStatement::If {
        test: Box::new(test),
        body,
        orelse,
    })
}

/// Parses the body that follows a compound statement's `:`.
///
/// Either an indented block on the following lines, or an inline body of
/// simple statements on the same line (`if a: b; c`). Trailing newlines after
/// the block are consumed so the caller sees the next keyword (`elif`, `else`)
/// or statement directly.
pub(super) fn parse_block(parser: &mut PyParser) -> Result<Vec<PyStatement>, PylentilError> {
    if parser.peek()?.kind != PyTokenType::Newline {
        return parse_inline_body(parser);
    }

    parser.skip_newlines()?;
    parser.expect_type(vec![PyTokenType::Indent])?;

    let mut body: Vec<PyStatement> = Vec::new();
    while parser.has_tokens() && parser.peek()?.kind != PyTokenType::Dedent {
        body.push(parse_statement(parser)?);
        parser.skip_statement_separators()?;
    }

    parser.expect_type(vec![PyTokenType::Dedent])?;
    parser.skip_newlines()?;

    Ok(body)
}

/// Parses one or more `;`-separated simple statements up to the end of the
/// line, for bodies written on the same line as their header.
fn parse_inline_body(parser: &mut PyParser) -> Result<Vec<PyStatement>, PylentilError> {
    let mut body = vec![parse_statement(parser)?];

    while parser.peek()?.kind == PyTokenType::Semicolon {
        parser.consume()?;

        if matches!(parser.peek()?.kind, PyTokenType::Newline | PyTokenType::EOF) {
            break;
        }

        body.push(parse_statement(parser)?);
    }

    parser.skip_newlines()?;

    Ok(body)
}

pub(super) fn parse_stmt_pass(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::Pass])?;
    
    Ok(PyStatement::Pass)
}

pub(super) fn parse_stmt_break(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::Break])?;
    
    Ok(PyStatement::Break)
}

pub(super) fn parse_stmt_continue(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::Continue])?;
    
    Ok(PyStatement::Continue)
}
