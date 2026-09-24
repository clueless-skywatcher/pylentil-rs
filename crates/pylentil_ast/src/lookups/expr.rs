use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType, ast::{
        PyArg, PyBinaryOp, PyBoolOp, PyComparisonOp, PyComprehension, PyConstant, PyExpr, PyExprBox, PyKeyword, PyRefContext, PyUnaryOp,
    }, parser::PyParser,
};

use super::{PyBindingPower, parse_expr};

#[derive(Debug)]
enum PyArgType {
    Arg(PyArg),
    Keyword(PyKeyword),
}

fn parse_int(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::Int,
            value: Some(value),
        } => Ok(PyExpr::Constant {
            kind: None,
            value: PyConstant::Integer(value.to_string()),
        }),
        PyToken {
            kind: PyTokenType::Int,
            value: None,
        } => Err(PylentilError::TokenMissingValue {
            kind: "an integer literal",
        }),
        other => Err(PylentilError::UnexpectedToken {
            expected: PyTokenType::Int.describe().to_string(),
            found: other.describe(),
        }),
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
        PyToken {
            kind: PyTokenType::Float,
            value: None,
        } => Err(PylentilError::TokenMissingValue {
            kind: "a float literal",
        }),
        other => Err(PylentilError::UnexpectedToken {
            expected: PyTokenType::Float.describe().to_string(),
            found: other.describe(),
        }),
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
        PyToken {
            kind: PyTokenType::Ident,
            value: None,
        } => Err(PylentilError::TokenMissingValue {
            kind: "an identifier",
        }),
        other => Err(PylentilError::UnexpectedToken {
            expected: PyTokenType::Ident.describe().to_string(),
            found: other.describe(),
        }),
    }
}

fn parse_boolean(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let token = parser.consume()?;
    let bool_val = match token.kind {
        PyTokenType::True => true,
        PyTokenType::False => false,
        _ => {
            return Err(PylentilError::UnexpectedToken {
                expected: "keyword `True` or keyword `False`".to_string(),
                found: token.describe(),
            });
        }
    };

    Ok(PyExpr::Constant {
        value: PyConstant::Boolean(bool_val),
        kind: None,
    })
}

fn parse_none(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let token = parser.consume()?;
    match token.kind {
        PyTokenType::None => Ok(PyExpr::Constant {
            value: PyConstant::None,
            kind: None,
        }),
        _ => Err(PylentilError::UnexpectedToken {
            expected: PyTokenType::None.describe().to_string(),
            found: token.describe(),
        }),
    }
}

fn parse_ellipsis(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let token = parser.consume()?;
    match token.kind {
        PyTokenType::Ellipsis => Ok(PyExpr::Constant {
            value: PyConstant::Ellipsis,
            kind: None,
        }),
        _ => Err(PylentilError::UnexpectedToken {
            expected: PyTokenType::Ellipsis.describe().to_string(),
            found: token.describe(),
        }),
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
        PyToken {
            kind: PyTokenType::String,
            value: None,
        } => Err(PylentilError::TokenMissingValue {
            kind: "a string literal",
        }),
        other => Err(PylentilError::UnexpectedToken {
            expected: PyTokenType::String.describe().to_string(),
            found: other.describe(),
        }),
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
        _ => Err(PylentilError::NotATerminal {
            found: token.describe(),
        }),
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
        _ => Err(PylentilError::UnsupportedOperator {
            found: kind.describe().to_string(),
            context: "a binary expression",
        }),
    }
}

fn bool_op(kind: PyTokenType) -> Result<PyBoolOp, PylentilError> {
    match kind {
        PyTokenType::And => Ok(PyBoolOp::And),
        PyTokenType::Or => Ok(PyBoolOp::Or),
        _ => Err(PylentilError::UnsupportedOperator {
            found: kind.describe().to_string(),
            context: "a boolean expression",
        }),
    }
}

pub(super) fn parse_bool_op(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    let kind = parser.consume()?.kind;
    let op = bool_op(kind)?;

    let mut values = vec![left, parse_expr(parser, bp)?];

    while parser.peek()?.kind == kind {
        parser.consume()?;
        values.push(parse_expr(parser, bp)?);
    }

    Ok(PyExpr::BoolOp { op, values })
}

fn unary_op(kind: PyTokenType) -> Result<PyUnaryOp, PylentilError> {
    match kind {
        PyTokenType::Minus => Ok(PyUnaryOp::UnarySub),
        PyTokenType::Plus => Ok(PyUnaryOp::UnaryAdd),
        PyTokenType::Tilde => Ok(PyUnaryOp::Invert),
        PyTokenType::Not => Ok(PyUnaryOp::Not),
        _ => Err(PylentilError::UnsupportedOperator {
            found: kind.describe().to_string(),
            context: "a unary expression",
        }),
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

pub(super) fn augmented_op(kind: PyTokenType) -> Option<PyBinaryOp> {
    Some(match kind {
        PyTokenType::PlusEqual => PyBinaryOp::Add,
        PyTokenType::MinusEqual => PyBinaryOp::Sub,
        PyTokenType::StarEqual => PyBinaryOp::Mul,
        PyTokenType::SlashEqual => PyBinaryOp::Div,
        PyTokenType::DoubleSlashEqual => PyBinaryOp::FloorDiv,
        PyTokenType::PercentEqual => PyBinaryOp::Mod,
        PyTokenType::DoubleStarEqual => PyBinaryOp::Pow,
        PyTokenType::AtEqual => PyBinaryOp::MatMult,
        PyTokenType::LShiftEqual => PyBinaryOp::LShift,
        PyTokenType::RShiftEqual => PyBinaryOp::RShift,
        PyTokenType::AmpersandEqual => PyBinaryOp::BitAnd,
        PyTokenType::VerticalBarEqual => PyBinaryOp::BitOr,
        PyTokenType::CaretEqual => PyBinaryOp::BitXor,
        _ => return None,
    })
}

fn comparison_op(parser: &mut PyParser) -> Result<PyComparisonOp, PylentilError> {
    let token = parser.consume()?;
    match token.kind {
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
        _ => Err(PylentilError::UnsupportedOperator {
            found: token.describe(),
            context: "a comparison",
        }),
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
        other => {
            return Err(PylentilError::InvalidAssignmentTarget {
                found: other.describe(),
            });
        }
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

pub(super) fn parse_walrus_tuple_or_expr(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
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

    if parser.peek()?.kind == PyTokenType::Walrus {
        parser.consume()?;

        if !matches!(
            exprs,
            PyExpr::Name { .. }
                | PyExpr::Tuple { .. }
                | PyExpr::Attribute { .. }
                | PyExpr::Subscript { .. }
        ) {
            return Err(PylentilError::InvalidAssignmentTarget {
                found: exprs.describe(),
            });
        }

        let value = parse_expr(parser, PyBindingPower::Default)?;
        parser.expect_type(vec![PyTokenType::RParen])?;

        return Ok(PyExpr::NamedExpr {
            target: Box::new(exprs),
            value: Box::new(value),
        });
    }

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

pub(super) fn parse_attribute_access(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::Dot])?;

    let attribute = parse_expr(parser, bp)?;

    match attribute {
        PyExpr::Name { id, ctx } => Ok(PyExpr::Attribute {
            value: Box::new(left),
            attr: id,
            ctx,
        }),
        other => Err(PylentilError::InvalidAttributeName {
            found: other.describe(),
        }),
    }
}

pub(super) fn parse_subscript_access(
    parser: &mut PyParser,
    left: PyExpr,
    _bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::LSquare])?;

    let slice = parse_slice(parser)?;

    parser.expect_type(vec![PyTokenType::RSquare])?;

    Ok(PyExpr::Subscript {
        value: Box::new(left),
        slice: Box::new(slice),
        ctx: PyRefContext::Load,
    })
}

pub(super) fn parse_function_call(
    parser: &mut PyParser,
    left: PyExpr,
    _bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::LParen])?;
    let mut args: Vec<PyExprBox> = vec![];
    let mut keywords: Vec<PyKeyword> = vec![];
    let mut kw_phase_started = false;

    loop {
        if parser.peek()?.kind == PyTokenType::RParen {
            break;
        }

        match parse_arg(parser)? {
            PyArgType::Arg(arg_expr) => {
                if kw_phase_started {
                    return Err(PylentilError::PositionalArgumentAfterKeyword);
                }
                args.push(arg_expr.arg)
            }
            PyArgType::Keyword(kw) => {
                if !kw_phase_started {
                    kw_phase_started = true;
                }
                keywords.push(kw);
            }
        }

        if parser.peek()?.kind == PyTokenType::RParen {
            break;
        }

        parser.expect_type(vec![PyTokenType::Comma])?;
    }
    parser.expect_type(vec![PyTokenType::RParen])?;

    Ok(PyExpr::Call {
        func: Box::new(left),
        args,
        keywords,
    })
}

fn parse_arg(parser: &mut PyParser) -> Result<PyArgType, PylentilError> {
    if parser.peek()?.kind == PyTokenType::DoubleStar {
        parser.consume()?;
        let expr = parse_expr(parser, PyBindingPower::Comma)?;
        return Ok(PyArgType::Keyword(PyKeyword {
            arg: None,
            value: Box::new(expr),
        }));
    }

    let arg = parse_expr(parser, PyBindingPower::Comma)?;

    if parser.peek()?.kind == PyTokenType::Assign {
        parser.consume()?;
        match arg {
            PyExpr::Name { id, .. } => {
                let val = parse_expr(parser, PyBindingPower::Comma)?;
                return Ok(PyArgType::Keyword(PyKeyword {
                    arg: Some(id),
                    value: Box::new(val),
                }));
            }
            other => {
                return Err(PylentilError::InvalidKeywordArgumentName {
                    found: other.describe(),
                });
            }
        }
    }

    Ok(PyArgType::Arg(PyArg {
        arg: Box::new(arg),
        annotation: None,
        type_comment: None,
    }))
}

fn parse_slice(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let mut lower: Option<PyExprBox> = None;
    let mut upper: Option<PyExprBox> = None;
    let mut step: Option<PyExprBox> = None;

    if parser.peek()?.kind != PyTokenType::Colon {
        lower = Some(Box::new(parse_expr(parser, PyBindingPower::Default)?));
    }
    if parser.peek()?.kind == PyTokenType::RSquare {
        return Ok(*lower.unwrap());
    } else {
        parser.expect_type(vec![PyTokenType::Colon])?;
    }

    if !matches!(
        parser.peek()?.kind,
        PyTokenType::RSquare | PyTokenType::Colon
    ) {
        upper = Some(Box::new(parse_expr(parser, PyBindingPower::Default)?));
    }

    if parser.peek()?.kind != PyTokenType::RSquare {
        parser.expect_type(vec![PyTokenType::Colon])?;
    }

    if parser.peek()?.kind != PyTokenType::RSquare {
        step = Some(Box::new(parse_expr(parser, PyBindingPower::Default)?));
    }

    Ok(PyExpr::Slice { lower, upper, step })
}

pub(super) fn parse_if(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::If])?;

    let test = parse_expr(parser, bp)?;

    parser.expect_type(vec![PyTokenType::Else])?;
    let orelse = parse_expr(parser, PyBindingPower::Default)?;

    Ok(PyExpr::IfExp {
        test: Box::new(test),
        body: Box::new(left),
        orelse: Box::new(orelse),
    })
}

pub(super) fn parse_dict_or_set_or_comprehension(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::LBrace])?;

    if parser.peek()?.kind == PyTokenType::RBrace {
        parser.consume()?;
        return Ok(PyExpr::Dict { keys: vec![], values: vec![] });
    }

    if parser.peek()?.kind == PyTokenType::DoubleStar {
        parser.consume()?;
        let value = parse_expr(parser, PyBindingPower::Comma)?;
        return parse_dict(parser, None, value);
    }

    let first = parse_expr(parser, PyBindingPower::Comma)?;

    if parser.peek()?.kind == PyTokenType::Colon {
        parser.consume()?;
        let value = parse_expr(parser, PyBindingPower::Comma)?;
        return parse_dict(parser, Some(first), value);
    }

    parse_set(parser, first)
}

pub(super) fn parse_list_or_comprehension(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    parser.expect_type(vec![PyTokenType::LSquare])?;

    let mut elts: Vec<PyExpr> = vec![];

    loop {
        if parser.peek()?.kind == PyTokenType::RSquare {
            break;
        }

        let expr = parse_expr(parser, PyBindingPower::Comma)?;

        if parser.peek()?.kind == PyTokenType::For {
            let mut generators: Vec<PyComprehension> = vec![];
            loop {
                generators.push(parse_comprehension(parser)?);
                if parser.peek()?.kind == PyTokenType::RSquare {
                    break;
                }
            }

            parser.expect_type(vec![PyTokenType::RSquare])?;
            return Ok(PyExpr::ListComp { elt: Box::new(expr), generators })
        }

        elts.push(expr);
        if parser.peek()?.kind == PyTokenType::RSquare {
            break;
        }

        parser.expect_type(vec![PyTokenType::Comma])?;
    }

    parser.expect_type(vec![PyTokenType::RSquare])?;

    Ok(PyExpr::List {
        elts,
        ctx: PyRefContext::Load,
    })
}

fn parse_dict(parser: &mut PyParser, key: Option<PyExpr>, value: PyExpr) -> Result<PyExpr, PylentilError> {
    let mut keys = vec![key];
    let mut values = vec![value];

    loop {
        if parser.peek()?.kind == PyTokenType::RBrace {
            break;
        }

        parser.expect_type(vec![PyTokenType::Comma])?;

        if parser.peek()?.kind == PyTokenType::RBrace {
            break;
        }

        if parser.peek()?.kind == PyTokenType::DoubleStar {
            parser.consume()?;
            keys.push(None);
            values.push(parse_expr(parser, PyBindingPower::Comma)?);
        } else {
            let key = parse_expr(parser, PyBindingPower::Comma)?;
            parser.expect_type(vec![PyTokenType::Colon])?;
            keys.push(Some(key));
            values.push(parse_expr(parser, PyBindingPower::Comma)?);
        }
    }
    parser.expect_type(vec![PyTokenType::RBrace])?;
    Ok(PyExpr::Dict { keys, values })
}

fn parse_set(parser: &mut PyParser, first: PyExpr) -> Result<PyExpr, PylentilError> {
    let mut elts = vec![first];

    loop {
        if parser.peek()?.kind == PyTokenType::RBrace {
            break;
        }
        parser.expect_type(vec![PyTokenType::Comma])?;
        if parser.peek()?.kind == PyTokenType::RBrace {
            break;
        }

        elts.push(parse_expr(parser, PyBindingPower::Comma)?);
    }
    parser.expect_type(vec![PyTokenType::RBrace])?;

    Ok(PyExpr::Set { elts })
}

fn parse_comprehension(parser: &mut PyParser) -> Result<PyComprehension, PylentilError> {
    parser.expect_type(vec![PyTokenType::For])?;
    let target = parse_expr(parser, PyBindingPower::Ternary)?;
    parser.expect_type(vec![PyTokenType::In])?;
    let iter = parse_expr(parser, PyBindingPower::Default)?;

    let mut ifs: Vec<PyExpr> = vec![];

    if parser.peek()?.kind == PyTokenType::If {
        loop {
            parser.consume()?;
            ifs.push(parse_expr(parser, PyBindingPower::Default)?);

            if parser.peek()?.kind != PyTokenType::If {
                break;
            }
        }
    }

    Ok(PyComprehension { target: Box::new(target), iter: Box::new(iter), ifs, is_async: false })
}
