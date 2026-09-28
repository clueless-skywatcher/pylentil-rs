use std::collections::HashSet;

use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType,
    ast::{
        PyAlias, PyArg, PyArguments, PyExceptHandler, PyExpr, PyKeyword, PyRefContext, PyStatement,
    },
    parser::PyParser,
};

use super::{PyBindingPower, parse_expr, parse_statement};
use crate::common::{PyArgType, expect_ident, parse_parenthesized_args};

pub(super) fn parse_stmt_if(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::If])?;
    parse_if_after_keyword(parser, start)
}

/// Parses `<test>: <block> [elif ... | else: <block>]`, i.e. everything after
/// an `if` or `elif` keyword whose position is `start`. An `elif` chain becomes
/// an `If` nested in the `orelse` of its predecessor.
fn parse_if_after_keyword(
    parser: &mut PyParser,
    start: usize,
) -> Result<PyStatement, PylentilError> {
    let test = parse_expr(parser, PyBindingPower::Default)?;
    parser.expect_type(vec![PyTokenType::Colon])?;
    let body = parse_block(parser)?;

    let orelse = match parser.peek()?.kind {
        PyTokenType::Elif => {
            let elif_start = parser.start()?;
            parser.consume()?;
            vec![parse_if_after_keyword(parser, elif_start)?]
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
        span: parser.span_from(start),
    })
}

/// Parses the body that follows a compound statement's `:`
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
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::Pass])?;

    Ok(PyStatement::Pass {
        span: parser.span_from(start),
    })
}

pub(super) fn parse_stmt_break(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::Break])?;

    Ok(PyStatement::Break {
        span: parser.span_from(start),
    })
}

pub(super) fn parse_stmt_continue(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::Continue])?;

    Ok(PyStatement::Continue {
        span: parser.span_from(start),
    })
}

pub(super) fn parse_stmt_import(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
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

    Ok(PyStatement::Import {
        names: aliases,
        span: parser.span_from(start),
    })
}

/// Parses `from [.]*[module] import (* | names | (names[,]))`.
pub(super) fn parse_stmt_import_from(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
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
            let star = parser.consume()?;
            vec![PyAlias {
                name: "*".to_string(),
                asname: None,
                span: star.span,
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
        span: parser.span_from(start),
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
    let start = parser.start()?;
    let name = parse_dotted_name(parser)?;

    let asname = if parser.peek()?.kind == PyTokenType::As {
        parser.consume()?;
        Some(expect_ident(parser)?)
    } else {
        None
    };

    Ok(PyAlias {
        name,
        asname,
        span: parser.span_from(start),
    })
}

pub(super) fn parse_stmt_funcdef(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::Def])?;

    let name = expect_ident(parser)?;
    let parsed_args: Vec<PyArgType> = parse_parenthesized_args(parser, true)?;

    parser.expect_type(vec![PyTokenType::Colon])?;
    let def_block = parse_block(parser)?;

    let mut posonlyargs: Vec<PyArg> = vec![];
    let mut args: Vec<PyArg> = vec![];
    let mut vararg: Option<PyArg> = None;
    let mut kwonlyargs: Vec<PyArg> = vec![];
    let mut kw_defaults: Vec<Option<PyExpr>> = vec![];
    let mut defaults: Vec<Option<PyExpr>> = vec![];
    let mut kwarg: Option<PyArg> = None;

    let mut posonly_marker_seen = false;
    // Set by `*args` or a bare `*`; every parameter after it is keyword-only.
    let mut star_seen = false;
    let mut bare_star_seen = false;
    let mut names: HashSet<String> = HashSet::new();

    for arg_type in parsed_args {
        // `**kwargs` must be the last parameter.
        if kwarg.is_some() {
            return Err(PylentilError::InvalidArgumentType);
        }

        match arg_type {
            PyArgType::Arg(arg) => {
                declare_parameter(&mut names, &arg)?;

                if matches!(arg.arg.as_ref(), PyExpr::Starred { .. }) {
                    if star_seen {
                        return Err(PylentilError::InvalidArgumentType);
                    }
                    star_seen = true;
                    vararg = Some(arg);
                } else if star_seen {
                    kwonlyargs.push(arg);
                    kw_defaults.push(None);
                } else {
                    // A positional parameter without a default cannot follow
                    // one with a default.
                    if !defaults.is_empty() {
                        return Err(PylentilError::InvalidArgumentType);
                    }
                    args.push(arg);
                }
            }
            PyArgType::Keyword {
                keyword:
                    PyKeyword {
                        arg: Some(kw_arg),
                        value: kw_value,
                        ..
                    },
                annotation,
                name_span,
                arg_span,
            } => {
                let arg = PyArg {
                    arg: Box::new(PyExpr::Name {
                        id: kw_arg,
                        ctx: PyRefContext::Load,
                        span: name_span,
                    }),
                    annotation,
                    type_comment: None,
                    span: arg_span,
                };
                declare_parameter(&mut names, &arg)?;

                if star_seen {
                    kwonlyargs.push(arg);
                    kw_defaults.push(Some(*kw_value));
                } else {
                    args.push(arg);
                    defaults.push(Some(*kw_value));
                }
            }
            PyArgType::Keyword {
                keyword:
                    PyKeyword {
                        arg: None,
                        value: kw_name,
                        ..
                    },
                annotation,
                arg_span,
                ..
            } => {
                let arg = PyArg {
                    arg: kw_name,
                    annotation,
                    type_comment: None,
                    span: arg_span,
                };
                declare_parameter(&mut names, &arg)?;
                kwarg = Some(arg);
            }
            PyArgType::PosOnlyMarker => {
                if posonly_marker_seen || star_seen || args.is_empty() {
                    return Err(PylentilError::InvalidArgumentType);
                }
                posonly_marker_seen = true;
                posonlyargs.append(&mut args);
            }
            PyArgType::KeywordOnlyMarker => {
                if star_seen {
                    return Err(PylentilError::InvalidArgumentType);
                }
                star_seen = true;
                bare_star_seen = true;
            }
        }
    }

    if bare_star_seen && kwonlyargs.is_empty() {
        return Err(PylentilError::InvalidArgumentType);
    }

    Ok(PyStatement::FunctionDef {
        name,
        args: Box::new(PyArguments {
            posonlyargs,
            args,
            vararg,
            kwonlyargs,
            kw_defaults,
            kwarg,
            defaults,
        }),
        body: def_block,
        decorator_list: vec![],
        returns: None,
        type_comment: None,
        type_params: vec![],
        is_async: false,
        span: parser.span_from(start),
    })
}

/// Records a parameter's name, rejecting anything that is not a name, `*name`
/// or `**name`, and any name already used in the same parameter list.
fn declare_parameter(names: &mut HashSet<String>, arg: &PyArg) -> Result<(), PylentilError> {
    let name = match arg.arg.as_ref() {
        PyExpr::Name { id, .. } => id,
        PyExpr::Starred { value, .. } => match value.as_ref() {
            PyExpr::Name { id, .. } => id,
            _ => return Err(PylentilError::InvalidArgumentType),
        },
        _ => return Err(PylentilError::InvalidArgumentType),
    };

    if !names.insert(name.clone()) {
        return Err(PylentilError::InvalidArgumentType);
    }

    Ok(())
}

pub(super) fn parse_stmt_async(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::Async])?;

    if parser.peek()?.kind == PyTokenType::Def {
        let func_def = parse_stmt_funcdef(parser)?;
        return match func_def {
            PyStatement::FunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
                type_params,
                ..
            } => Ok(PyStatement::FunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
                type_params,
                is_async: true,
                span: parser.span_from(start),
            }),
            _ => Err(PylentilError::CodePathNotImplemented),
        };
    }
    Err(PylentilError::CodePathNotImplemented)
}

pub fn parse_stmt_return(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::Return])?;

    if parser.peek()?.kind.is_eof() || parser.peek()?.kind == PyTokenType::Newline {
        return Ok(PyStatement::Return {
            value: None,
            span: parser.span_from(start),
        });
    }

    let value = parse_expr(parser, PyBindingPower::Default)?;
    Ok(PyStatement::Return {
        value: Some(Box::new(value)),
        span: parser.span_from(start),
    })
}

pub fn parse_stmt_try(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let start = parser.start()?;
    parser.expect_type(vec![PyTokenType::Try])?;
    parser.expect_type(vec![PyTokenType::Colon])?;
    // parser.skip_statement_separators()?;

    let body = parse_block(parser)?;

    let mut handlers = vec![];
    let mut orelse = vec![];
    let mut finalbody = vec![];

    if parser.peek()?.kind == PyTokenType::Except {
        handlers.append(&mut parse_except_handlers(parser)?);
    } else if parser.peek()?.kind != PyTokenType::Finally {
        return Err(PylentilError::InvalidSyntax {
            error: "Try block must have an except or a finally block".to_string(),
        });
    }

    if parser.peek()?.kind == PyTokenType::Else {
        parser.consume()?; // Consume the else
        parser.consume()?; // Consume the colon
        orelse.append(&mut parse_block(parser)?);
    }

    if parser.peek()?.kind == PyTokenType::Finally {
        parser.consume()?; // Consume the else
        parser.consume()?; // Consume the colon
        finalbody.append(&mut parse_block(parser)?);
    }

    Ok(PyStatement::Try {
        body,
        handlers,
        orelse,
        finalbody,
        span: parser.span_from(start),
    })
}

fn parse_except_handlers(parser: &mut PyParser) -> Result<Vec<PyExceptHandler>, PylentilError> {
    assert_eq!(parser.peek()?.kind, PyTokenType::Except);

    let mut handlers = vec![];

    loop {
        if parser.peek()?.kind != PyTokenType::Except {
            break;
        }
        let start = parser.start()?;
        parser.consume()?;
        let mut name: Option<String> = None;
        let mut type_: Option<Box<PyExpr>> = None;

        if parser.peek()?.kind != PyTokenType::Colon {
            // `parse_alias` would give no span for the type alone, so read
            // `Type [as name]` here.
            let type_start = parser.start()?;
            let type_name = parse_dotted_name(parser)?;
            type_ = Some(Box::new(PyExpr::Name {
                id: type_name,
                ctx: PyRefContext::Load,
                span: parser.span_from(type_start),
            }));

            if parser.peek()?.kind == PyTokenType::As {
                parser.consume()?;
                name = Some(expect_ident(parser)?);
            }
        }
        parser.consume()?; // Consuming the colon

        let body = parse_block(parser)?;

        handlers.push(PyExceptHandler {
            type_,
            name,
            body,
            span: parser.span_from(start),
        });
    }

    Ok(handlers)
}
