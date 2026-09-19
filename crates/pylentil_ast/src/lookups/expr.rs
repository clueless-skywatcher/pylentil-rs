use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType,
    ast::{PyBinaryOp, PyComparisonOp, PyConstant, PyExpr, PyRefContext, PyUnaryOp},
    parser::PyParser,
};

use super::{PyBindingPower, parse_expr};

fn parse_int(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::Int,
            value: Some(value),
        } => Ok(PyExpr::Constant {
            kind: None,
            value: PyConstant::Integer(value.to_string()),
        }),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn parse_float(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::Float,
            value: Some(value),
        } => Ok(PyExpr::Constant {
            kind: None,
            value: PyConstant::Float(value.to_string()),
        }),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn parse_ident(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::Ident,
            value: Some(value),
        } => Ok({
            PyExpr::Name {
                id: value.to_string(),
                ctx: PyRefContext::Load,
            }
        }),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn parse_boolean(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let bool_val = match parser.consume()?.kind {
        PyTokenType::True => true,
        PyTokenType::False => false,
        _ => return Err(PylentilError::InvalidSyntax),
    };

    Ok(PyExpr::Constant {
        value: PyConstant::Boolean(bool_val),
        kind: None,
    })
}

fn parse_none(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()?.kind {
        PyTokenType::None => Ok(PyExpr::Constant {
            value: PyConstant::None,
            kind: None,
        }),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn parse_ellipsis(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()?.kind {
        PyTokenType::Ellipsis => Ok(PyExpr::Constant {
            value: PyConstant::Ellipsis,
            kind: None,
        }),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn parse_string(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::String,
            value: Some(value),
        } => Ok({
            PyExpr::Constant {
                value: PyConstant::String(value.to_string()),
                kind: None,
            }
        }),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

pub(super) fn parse_terminal(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let token = parser.peek()?;
    match token.kind {
        PyTokenType::Int => parse_int(parser),
        PyTokenType::Float => parse_float(parser),
        PyTokenType::True | PyTokenType::False => parse_boolean(parser),
        PyTokenType::String => parse_string(parser),
        PyTokenType::Ident => parse_ident(parser),
        PyTokenType::None => parse_none(parser),
        PyTokenType::Ellipsis => parse_ellipsis(parser),
        _ => Err(PylentilError::NotATerminal),
    }
}

fn binary_op(kind: PyTokenType) -> Result<PyBinaryOp, PylentilError> {
    match kind {
        PyTokenType::Plus => Ok(PyBinaryOp::Add),
        PyTokenType::Minus => Ok(PyBinaryOp::Sub),
        PyTokenType::Star => Ok(PyBinaryOp::Mul),
        PyTokenType::Slash => Ok(PyBinaryOp::Div),
        PyTokenType::DoubleSlash => Ok(PyBinaryOp::FloorDiv),
        PyTokenType::Percent => Ok(PyBinaryOp::Mod),
        PyTokenType::At => Ok(PyBinaryOp::MatMult),
        PyTokenType::DoubleStar => Ok(PyBinaryOp::Pow),
        PyTokenType::LShift => Ok(PyBinaryOp::LShift),
        PyTokenType::RShift => Ok(PyBinaryOp::RShift),
        PyTokenType::Ampersand => Ok(PyBinaryOp::BitAnd),
        PyTokenType::Caret => Ok(PyBinaryOp::BitXor),
        PyTokenType::VerticalBar => Ok(PyBinaryOp::BitOr),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn unary_op(kind: PyTokenType) -> Result<PyUnaryOp, PylentilError> {
    match kind {
        PyTokenType::Minus => Ok(PyUnaryOp::UnarySub),
        PyTokenType::Plus => Ok(PyUnaryOp::UnaryAdd),
        PyTokenType::Tilde => Ok(PyUnaryOp::Invert),
        PyTokenType::Not => Ok(PyUnaryOp::Not),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

pub(super) fn parse_unary(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let kind = parser.consume()?.kind;
    let op = unary_op(kind)?;

    let operand_bp = match kind {
        PyTokenType::Not => PyBindingPower::Not,
        _ => PyBindingPower::Unary,
    };

    let operand = parse_expr(parser, operand_bp)?;

    Ok(PyExpr::UnaryOp {
        op,
        operand: Box::new(operand),
    })
}

fn comparison_op(parser: &mut PyParser) -> Result<PyComparisonOp, PylentilError> {
    match parser.consume()?.kind {
        PyTokenType::DoubleEqual => Ok(PyComparisonOp::Eq),
        PyTokenType::NotEqual => Ok(PyComparisonOp::NotEq),
        PyTokenType::Less => Ok(PyComparisonOp::Lt),
        PyTokenType::Greater => Ok(PyComparisonOp::Gt),
        PyTokenType::LessEqual => Ok(PyComparisonOp::Lte),
        PyTokenType::GreaterEqual => Ok(PyComparisonOp::Gte),
        PyTokenType::In => Ok(PyComparisonOp::In),
        PyTokenType::Is => match parser.peek()?.kind {
            PyTokenType::Not => {
                parser.consume()?;
                Ok(PyComparisonOp::IsNot)
            }
            _ => Ok(PyComparisonOp::Is),
        },
        PyTokenType::Not => {
            parser.expect_type(vec![PyTokenType::In])?;
            Ok(PyComparisonOp::NotIn)
        }
        _ => Err(PylentilError::InvalidSyntax),
    }
}

pub(super) fn parse_binary(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    let op = binary_op(parser.consume()?.kind)?;

    let right = parse_expr(parser, bp)?;

    Ok(PyExpr::BinOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    })
}

fn is_comparison_op(kind: PyTokenType) -> bool {
    matches!(
        kind,
        PyTokenType::DoubleEqual
            | PyTokenType::NotEqual
            | PyTokenType::Less
            | PyTokenType::Greater
            | PyTokenType::LessEqual
            | PyTokenType::GreaterEqual
            | PyTokenType::Is
            | PyTokenType::In
            | PyTokenType::Not
    )
}

pub(super) fn parse_comparison(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    let mut ops: Vec<PyComparisonOp> = Vec::new();
    let mut comparators: Vec<PyExpr> = Vec::new();

    loop {
        ops.push(comparison_op(parser)?);
        comparators.push(parse_expr(parser, bp)?);

        if !is_comparison_op(parser.peek()?.kind) {
            break;
        }
    }

    Ok(PyExpr::Compare {
        left: Box::new(left),
        ops,
        comparators,
    })
}

pub(super) fn as_target(expr: PyExpr) -> Result<PyExpr, PylentilError> {
    Ok(match expr {
        PyExpr::Name { id, .. } => PyExpr::Name {
            id,
            ctx: PyRefContext::Store,
        },
        PyExpr::Tuple {
            elts,
            parenthesized,
            ..
        } => PyExpr::Tuple {
            elts: elts
                .into_iter()
                .map(as_target)
                .collect::<Result<Vec<_>, _>>()?,
            ctx: PyRefContext::Store,
            parenthesized,
        },
        PyExpr::List { elts, .. } => PyExpr::List {
            elts: elts
                .into_iter()
                .map(as_target)
                .collect::<Result<Vec<_>, _>>()?,
            ctx: PyRefContext::Store,
        },
        PyExpr::Starred { value, .. } => PyExpr::Starred {
            value: Box::new(as_target(*value)?),
            ctx: PyRefContext::Store,
        },
        PyExpr::Attribute { value, attr, .. } => PyExpr::Attribute {
            value,
            attr,
            ctx: PyRefContext::Store,
        },
        PyExpr::Subscript { value, slice, .. } => PyExpr::Subscript {
            value,
            slice,
            ctx: PyRefContext::Store,
        },
        _ => return Err(PylentilError::InvalidSyntax),
    })
}

fn ends_expression_list(kind: PyTokenType) -> bool {
    matches!(
        kind,
        PyTokenType::RParen
            | PyTokenType::RSquare
            | PyTokenType::RBrace
            | PyTokenType::Newline
            | PyTokenType::Semicolon
            | PyTokenType::Colon
            | PyTokenType::Assign
            | PyTokenType::EOF
    )
}

pub(super) fn parse_potentially_comma_separated(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    let mut elts = vec![left];

    while parser.peek()?.kind == PyTokenType::Comma {
        parser.consume()?;

        if ends_expression_list(parser.peek()?.kind) {
            break;
        }

        elts.push(parse_expr(parser, bp)?);
    }

    Ok(PyExpr::Tuple {
        elts,
        ctx: PyRefContext::Load,
        parenthesized: false,
    })
}

pub(super) fn parse_tuple_or_expr(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::LParen])?;

    if parser.peek()?.kind == PyTokenType::RParen {
        parser.consume()?;

        return Ok(PyExpr::Tuple {
            elts: Vec::new(),
            ctx: PyRefContext::Load,
            parenthesized: true,
        });
    }

    let exprs = parse_expr(parser, PyBindingPower::Default)?;

    parser.expect_type(vec![PyTokenType::RParen])?;

    Ok(match exprs {
        PyExpr::Tuple {
            elts,
            ctx,
            parenthesized,
        } => match parenthesized {
            true => PyExpr::Tuple {
                elts,
                ctx,
                parenthesized,
            },
            false => PyExpr::Tuple {
                elts,
                ctx,
                parenthesized: true,
            },
        },
        expr => expr,
    })
}

pub(super) fn parse_star(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::Star])?;

    let expr = parse_expr(parser, PyBindingPower::Default)?;

    Ok(PyExpr::Starred {
        value: Box::new(expr),
        ctx: PyRefContext::Load,
    })
}
