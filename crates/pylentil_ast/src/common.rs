//! Helpers shared by the expression and statement parsers.

use pylentil_common::{errors::PylentilError, span::PySpan};

use crate::{
    PyTokenType,
    ast::{PyArg, PyExpr, PyExprBox, PyKeyword, PyRefContext},
    parser::PyParser,
};

use crate::lookups::{PyBindingPower, parse_expr, parse_generators};

/// One entry of a parenthesised argument list, for both calls and `def`s.
#[derive(Debug, Clone, PartialEq)]
pub enum PyArgType {
    /// `value`, `*value`, or `name: annotation` when annotations are detected.
    Arg(PyArg),
    /// `name=value`, `**value`, or `name: annotation = value` when annotations
    /// are detected. Calls ignore `annotation`; definitions keep it.
    Keyword {
        keyword: PyKeyword,
        annotation: Option<PyExprBox>,
        /// Where `name` is in `name=value`; for `**value`, the whole entry.
        name_span: PySpan,
        /// `name` plus any annotation, without the default; for `**value`,
        /// the whole entry.
        arg_span: PySpan,
    },
    /// Only used for function definitions - Marker to indicate end of positional-only-arguments
    PosOnlyMarker,
    /// Only used for function definitions - Marker to indicate start of keyword-only-arguments
    KeywordOnlyMarker,
}

/// Parses `( arg, arg, ... )` including both parentheses.
///
/// Entries are returned in source order; ordering rules differ between calls
/// and definitions, so each caller checks its own. A trailing comma is
/// allowed. `optionally_detect_annotations` is passed through to [`parse_arg`].
pub(crate) fn parse_parenthesized_args(
    parser: &mut PyParser,
    optionally_detect_annotations: bool,
) -> Result<Vec<PyArgType>, PylentilError> {
    parser.expect_type(vec![PyTokenType::LParen])?;

    let mut entries: Vec<PyArgType> = vec![];

    while parser.peek()?.kind != PyTokenType::RParen {
        entries.push(parse_arg(parser, optionally_detect_annotations)?);

        if parser.peek()?.kind == PyTokenType::RParen {
            break;
        }

        parser.expect_type(vec![PyTokenType::Comma])?;
    }

    parser.expect_type(vec![PyTokenType::RParen])?;

    Ok(entries)
}

/// Parses one argument-list entry.
///
/// With `optionally_detect_annotations` set, a `name` followed by `:` reads an
/// annotation expression, as in a `def` parameter list. Calls pass `false`.
pub(crate) fn parse_arg(
    parser: &mut PyParser,
    optionally_detect_annotations: bool,
) -> Result<PyArgType, PylentilError> {
    let start = parser.start()?;

    if parser.peek()?.kind == PyTokenType::DoubleStar {
        parser.consume()?;
        let expr = parse_expr(parser, PyBindingPower::Comma)?;
        let span = parser.span_from(start);
        return Ok(PyArgType::Keyword {
            keyword: PyKeyword {
                arg: None,
                value: Box::new(expr),
                span,
            },
            annotation: None,
            name_span: span,
            arg_span: span,
        });
    }

    if parser.peek()?.kind == PyTokenType::Slash {
        parser.consume()?;
        return Ok(PyArgType::PosOnlyMarker);
    }
    // A bare `*` is followed directly by `,` or `)`; otherwise it starts `*args`.
    // The starred value is parsed at `Comma` so `*args, b` stays two entries.
    let arg = if parser.peek()?.kind == PyTokenType::Star {
        parser.consume()?;
        if matches!(
            parser.peek()?.kind,
            PyTokenType::Comma | PyTokenType::RParen
        ) {
            return Ok(PyArgType::KeywordOnlyMarker);
        }
        PyExpr::Starred {
            value: Box::new(parse_expr(parser, PyBindingPower::Comma)?),
            ctx: PyRefContext::Load,
            span: parser.span_from(start),
        }
    } else {
        parse_expr(parser, PyBindingPower::Comma)?
    };

    if parser.peek()?.kind == PyTokenType::For {
        let generators = parse_generators(parser)?;

        return Ok(PyArgType::Arg(PyArg {
            arg: Box::new(PyExpr::GeneratorExp {
                elt: Box::new(arg),
                generators,
                span: parser.span_from(start),
            }),
            annotation: None,
            type_comment: None,
            span: parser.span_from(start),
        }));
    }

    let is_parameter_name = match &arg {
        PyExpr::Name { .. } => true,
        PyExpr::Starred { value, .. } => matches!(value.as_ref(), PyExpr::Name { .. }),
        _ => false,
    };
    let annotation =
        parse_optional_annotation(parser, optionally_detect_annotations && is_parameter_name)?;
    // Everything but a default value: `*name: annotation`.
    let arg_span = parser.span_from(start);

    if parser.peek()?.kind == PyTokenType::Assign {
        parser.consume()?;
        return match arg {
            PyExpr::Name { id, span, .. } => {
                let value = parse_expr(parser, PyBindingPower::Comma)?;
                Ok(PyArgType::Keyword {
                    keyword: PyKeyword {
                        arg: Some(id),
                        value: Box::new(value),
                        span: parser.span_from(start),
                    },
                    annotation,
                    name_span: span,
                    arg_span,
                })
            }
            other => Err(PylentilError::InvalidKeywordArgumentName {
                found: other.describe(),
            }),
        };
    }

    Ok(PyArgType::Arg(PyArg {
        arg: Box::new(arg),
        annotation,
        type_comment: None,
        span: arg_span,
    }))
}

/// Parses `: annotation` if `allowed` and the next token is `:`.
fn parse_optional_annotation(
    parser: &mut PyParser,
    allowed: bool,
) -> Result<Option<PyExprBox>, PylentilError> {
    if allowed && parser.peek()?.kind == PyTokenType::Colon {
        parser.consume()?;
        Ok(Some(Box::new(parse_expr(parser, PyBindingPower::Comma)?)))
    } else {
        Ok(None)
    }
}

/// Consumes an identifier token and returns its text.
pub(crate) fn expect_ident(parser: &mut PyParser) -> Result<String, PylentilError> {
    let token = parser.expect_type(vec![PyTokenType::Ident])?;

    match token.value {
        Some(value) => Ok(value.to_string()),
        None => Err(PylentilError::TokenMissingValue {
            kind: "an identifier",
        }),
    }
}
