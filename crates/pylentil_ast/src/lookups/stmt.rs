use pylentil_common::errors::PylentilError;

use crate::{
    PyTokenType,
    ast::{PyAlias, PyStatement},
    parser::PyParser,
};

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

pub(super) fn parse_stmt_import(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::Import])?;

    let first = parse_alias(parser)?;
    let mut aliases: Vec<PyAlias> = vec![first];
    
    if parser.peek()?.kind == PyTokenType::Comma {
        loop {
            parser.consume()?;
            let import = parse_alias(parser)?;
            aliases.push(import);

            if parser.peek()?.kind.is_eof() || parser.peek()?.kind == PyTokenType::Newline {
                break;
            }    
        }
    }

    Ok(PyStatement::Import { names: aliases })
}

/// Parses `from [.]*[module] import (* | names | (names[,]))`.
pub(super) fn parse_stmt_import_from(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::From])?;

    let mut level = 0;
    loop {
        match parser.peek()?.kind {
            PyTokenType::Dot => level += 1,
            PyTokenType::Ellipsis => level += 3,
            _ => break,
        }
        parser.consume()?;
    }

    let module = if parser.peek()?.kind == PyTokenType::Import {
        if level == 0 {
            return Err(PylentilError::UnexpectedToken {
                expected: "a module name".to_string(),
                found: parser.peek()?.describe(),
            });
        }
        None
    } else {
        Some(parse_dotted_name(parser)?)
    };

    parser.expect_type(vec![PyTokenType::Import])?;

    let names = match parser.peek()?.kind {
        PyTokenType::Star => {
            parser.consume()?;
            vec![PyAlias {
                name: "*".to_string(),
                asname: None,
            }]
        }
        PyTokenType::LParen => {
            parser.consume()?;
            let names = parse_import_names(parser, true)?;
            parser.expect_type(vec![PyTokenType::RParen])?;
            names
        }
        _ => parse_import_names(parser, false)?,
    };

    Ok(PyStatement::ImportFrom {
        module,
        names,
        level: (level > 0).then_some(level),
    })
}

/// Parses `name [as alias] (, name [as alias])*`. A trailing comma is only
/// legal inside parentheses.
fn parse_import_names(
    parser: &mut PyParser,
    parenthesized: bool,
) -> Result<Vec<PyAlias>, PylentilError> {
    let mut names = vec![parse_alias(parser)?];

    while parser.peek()?.kind == PyTokenType::Comma {
        parser.consume()?;

        if parenthesized && parser.peek()?.kind == PyTokenType::RParen {
            break;
        }

        names.push(parse_alias(parser)?);
    }

    Ok(names)
}

/// Parses `name[.name...]` into a single dotted string.
fn parse_dotted_name(parser: &mut PyParser) -> Result<String, PylentilError> {
    let mut name = expect_ident(parser)?;

    while parser.peek()?.kind == PyTokenType::Dot {
        parser.consume()?;
        name.push('.');
        name.push_str(&expect_ident(parser)?);
    }

    Ok(name)
}

/// Parses `name[.name...] [as alias]`.
fn parse_alias(parser: &mut PyParser) -> Result<PyAlias, PylentilError> {
    let name = parse_dotted_name(parser)?;

    let asname = if parser.peek()?.kind == PyTokenType::As {
        parser.consume()?;
        Some(expect_ident(parser)?)
    } else {
        None
    };

    Ok(PyAlias { name, asname })
}

/// Consumes an identifier token and returns its text.
fn expect_ident(parser: &mut PyParser) -> Result<String, PylentilError> {
    let token = parser.expect_type(vec![PyTokenType::Ident])?;

    match token.value {
        Some(value) => Ok(value.to_string()),
        None => Err(PylentilError::TokenMissingValue {
            kind: "an identifier",
        }),
    }
}
