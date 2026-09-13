use std::collections::HashMap;

use lazy_static::lazy_static;
use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType,
    ast::{PyBinaryOp, PyComparisonOp, PyConstant, PyExpr, PyRefContext, PyStatement},
    parser::PyParser,
};

#[derive(Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
pub enum PyBindingPower {
    Default = 1,
    Comma,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    UnaryOp,
    FunctionCall,
    Attribute,
    Terminal,
}

pub type PyStatementHandler = for<'a> fn(&mut PyParser<'a>) -> Result<PyStatement, PylentilError>;
pub type PyNUDHandler = for<'a> fn(&mut PyParser<'a>) -> Result<PyExpr, PylentilError>;
pub type PyLEDHandler =
    for<'a> fn(&mut PyParser<'a>, PyExpr, PyBindingPower) -> Result<PyExpr, PylentilError>;

pub type PyStatementLookup = HashMap<PyTokenType, PyStatementHandler>;
pub type PyNUDLookup = HashMap<PyTokenType, PyNUDHandler>;
pub type PyLEDLookup = HashMap<PyTokenType, PyLEDHandler>;
pub type PyBindingPowerLookup = HashMap<PyTokenType, PyBindingPower>;

fn bp(lu: &mut PyBindingPowerLookup, kind: PyTokenType, power: PyBindingPower) {
    lu.insert(kind, power);
}

fn led(lu: &mut PyLEDLookup, kind: PyTokenType, led_handler: PyLEDHandler) {
    lu.insert(kind, led_handler);
}

fn nud(lu: &mut PyNUDLookup, kind: PyTokenType, nud_handler: PyNUDHandler) {
    lu.insert(kind, nud_handler);
}

fn stmt(lu: &mut PyStatementLookup, kind: PyTokenType, stmt_handler: PyStatementHandler) {
    lu.insert(kind, stmt_handler);
}

lazy_static! {
    static ref BP_LU: PyBindingPowerLookup = {
        let mut m = HashMap::new();

        // Atoms
        bp(&mut m, PyTokenType::Int, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::Float, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::String, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::Ident, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::True, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::False, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::None, PyBindingPower::Terminal);

        // Postfix: attr > call/subscript
        bp(&mut m, PyTokenType::Dot, PyBindingPower::Attribute);
        bp(&mut m, PyTokenType::LParen, PyBindingPower::FunctionCall);
        bp(&mut m, PyTokenType::LSquare, PyBindingPower::FunctionCall);

        // Unary
        bp(&mut m, PyTokenType::Tilde, PyBindingPower::UnaryOp);

        // Multiplicative / power
        bp(&mut m, PyTokenType::Star, PyBindingPower::Multiplicative);
        bp(&mut m, PyTokenType::Slash, PyBindingPower::Multiplicative);
        bp(&mut m, PyTokenType::Percent, PyBindingPower::Multiplicative);
        bp(&mut m, PyTokenType::DoubleStar, PyBindingPower::Multiplicative);

        // Additive / shifts
        bp(&mut m, PyTokenType::Plus, PyBindingPower::Additive);
        bp(&mut m, PyTokenType::Minus, PyBindingPower::Additive);
        bp(&mut m, PyTokenType::LShift, PyBindingPower::Additive);
        bp(&mut m, PyTokenType::RShift, PyBindingPower::Additive);

        // Bitwise
        bp(&mut m, PyTokenType::Ampersand, PyBindingPower::Logical);
        bp(&mut m, PyTokenType::Caret, PyBindingPower::Logical);
        bp(&mut m, PyTokenType::VerticalBar, PyBindingPower::Logical);

        // Comparisons
        bp(&mut m, PyTokenType::DoubleEqual, PyBindingPower::Relational);
        bp(&mut m, PyTokenType::NotEqual, PyBindingPower::Relational);
        bp(&mut m, PyTokenType::Less, PyBindingPower::Relational);
        bp(&mut m, PyTokenType::Greater, PyBindingPower::Relational);
        bp(&mut m, PyTokenType::LessEqual, PyBindingPower::Relational);
        bp(&mut m, PyTokenType::GreaterEqual, PyBindingPower::Relational);

        // Assignment / separators
        bp(&mut m, PyTokenType::Assign, PyBindingPower::Assignment);
        bp(&mut m, PyTokenType::Comma, PyBindingPower::Comma);

        m
    };
    static ref NUD_LU: PyNUDLookup = {
        let mut m = HashMap::new();

        nud(&mut m, PyTokenType::Int, parse_terminal);
        nud(&mut m, PyTokenType::Float, parse_terminal);
        nud(&mut m, PyTokenType::True, parse_terminal);
        nud(&mut m, PyTokenType::False, parse_terminal);
        nud(&mut m, PyTokenType::String, parse_terminal);
        nud(&mut m, PyTokenType::Ident, parse_terminal);
        nud(&mut m, PyTokenType::None, parse_terminal);

        m
    };
    static ref LED_LU: PyLEDLookup = {
        let mut m = HashMap::new();

        // Multiplicative / power
        led(&mut m, PyTokenType::Star, parse_binary);
        led(&mut m, PyTokenType::Slash, parse_binary);
        led(&mut m, PyTokenType::Percent, parse_binary);
        led(&mut m, PyTokenType::DoubleStar, parse_binary);

        // Additive / shifts
        led(&mut m, PyTokenType::Plus, parse_binary);
        led(&mut m, PyTokenType::Minus, parse_binary);
        led(&mut m, PyTokenType::LShift, parse_binary);
        led(&mut m, PyTokenType::RShift, parse_binary);

        // Bitwise
        led(&mut m, PyTokenType::Ampersand, parse_binary);
        led(&mut m, PyTokenType::Caret, parse_binary);
        led(&mut m, PyTokenType::VerticalBar, parse_binary);

        // Relational
        led(&mut m, PyTokenType::DoubleEqual, parse_comparison);
        led(&mut m, PyTokenType::NotEqual, parse_comparison);
        led(&mut m, PyTokenType::Less, parse_comparison);
        led(&mut m, PyTokenType::Greater, parse_comparison);
        led(&mut m, PyTokenType::LessEqual, parse_comparison);
        led(&mut m, PyTokenType::GreaterEqual, parse_comparison);

        // Comma
        led(&mut m, PyTokenType::Comma, parse_potentially_comma_separated);

        m
    };
    static ref STMT_LU: PyStatementLookup = {
        let mut m = HashMap::new();

        stmt(&mut m, PyTokenType::If, parse_stmt_if);

        m
    };
}

fn parse_int(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::Int,
            value: Some(value),
        } => {
            let int_value = value
                .parse::<i64>()
                .map_err(|_| PylentilError::InvalidSyntax)?;

            Ok(PyExpr::Constant {
                kind: None,
                value: PyConstant::Integer(int_value),
            })
        }
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn parse_float(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::Float,
            value: Some(value),
        } => {
            let float_value = value
                .parse::<f64>()
                .map_err(|_| PylentilError::InvalidSyntax)?;

            Ok(PyExpr::Constant {
                kind: None,
                value: PyConstant::Float(float_value),
            })
        }
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

fn parse_terminal(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    let token = parser.peek()?;
    match token.kind {
        PyTokenType::Int => parse_int(parser),
        PyTokenType::Float => parse_float(parser),
        PyTokenType::True | PyTokenType::False => parse_boolean(parser),
        PyTokenType::String => parse_string(parser),
        PyTokenType::Ident => parse_ident(parser),
        PyTokenType::None => parse_none(parser),
        _ => Err(PylentilError::NotATerminal),
    }
}

fn binary_op(kind: PyTokenType) -> Result<PyBinaryOp, PylentilError> {
    match kind {
        PyTokenType::Plus => Ok(PyBinaryOp::Add),
        PyTokenType::Minus => Ok(PyBinaryOp::Sub),
        PyTokenType::Star => Ok(PyBinaryOp::Mul),
        PyTokenType::Slash => Ok(PyBinaryOp::Div),
        PyTokenType::Percent => Ok(PyBinaryOp::Mod),
        PyTokenType::DoubleStar => Ok(PyBinaryOp::Pow),
        PyTokenType::LShift => Ok(PyBinaryOp::LShift),
        PyTokenType::RShift => Ok(PyBinaryOp::RShift),
        PyTokenType::Ampersand => Ok(PyBinaryOp::BitAnd),
        PyTokenType::Caret => Ok(PyBinaryOp::BitXor),
        PyTokenType::VerticalBar => Ok(PyBinaryOp::BitOr),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn comparison_op(kind: PyTokenType) -> Result<PyComparisonOp, PylentilError> {
    match kind {
        PyTokenType::DoubleEqual => Ok(PyComparisonOp::Eq),
        PyTokenType::NotEqual => Ok(PyComparisonOp::NotEq),
        PyTokenType::Less => Ok(PyComparisonOp::Lt),
        PyTokenType::Greater => Ok(PyComparisonOp::Gt),
        PyTokenType::LessEqual => Ok(PyComparisonOp::Lte),
        PyTokenType::GreaterEqual => Ok(PyComparisonOp::Gte),
        _ => Err(PylentilError::InvalidSyntax),
    }
}

fn parse_binary(
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

fn parse_expr(parser: &mut PyParser, bp: PyBindingPower) -> Result<PyExpr, PylentilError> {
    let Some(nud_fn) = NUD_LU.get(&parser.peek()?.kind) else {
        return Err(PylentilError::InvalidSyntax);
    };

    let mut left = nud_fn(parser)?;

    while BP_LU
        .get(&parser.peek()?.kind)
        .copied()
        .unwrap_or(PyBindingPower::Default)
        > bp
    {
        let token_kind = parser.peek()?.kind;
        let Some(led_fn) = LED_LU.get(&token_kind) else {
            return Err(PylentilError::InvalidSyntax);
        };

        let op_bp = BP_LU
            .get(&token_kind)
            .copied()
            .unwrap_or(PyBindingPower::Default);
        left = led_fn(parser, left, op_bp)?;
    }

    Ok(left)
}

pub fn parse_statement(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let token_kind = parser.peek()?.kind;

    match STMT_LU.get(&token_kind) {
        Some(stmt_fn) => Ok(stmt_fn(parser)?),
        None => {
            let expr = parse_expr(parser, PyBindingPower::Default)?;
            parser.skip_newlines()?;
            Ok(PyStatement::Expr { value: expr })
        }
    }
}

fn parse_stmt_if(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    parser.expect_type(vec![PyTokenType::If])?;
    let test = parse_expr(parser, PyBindingPower::Default)?;
    parser.expect_type(vec![PyTokenType::Colon])?;
    parser.skip_newlines()?;
    parser.expect_type(vec![PyTokenType::Indent])?;

    let mut body: Vec<PyStatement> = Vec::new();
    while parser.has_tokens() && parser.peek()?.kind != PyTokenType::Dedent {
        body.push(parse_statement(parser)?);
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
        }

        parser.expect_type(vec![PyTokenType::Dedent])?;
        parser.skip_newlines()?;
    }

    Ok(PyStatement::If { test, body, orelse })
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
    )
}

fn parse_comparison(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    let mut ops: Vec<PyComparisonOp> = Vec::new();
    let mut comparators: Vec<PyExpr> = Vec::new();

    loop {
        ops.push(comparison_op(parser.consume()?.kind)?);
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

fn parse_potentially_comma_separated(
    parser: &mut PyParser,
    left: PyExpr,
    bp: PyBindingPower,
) -> Result<PyExpr, PylentilError> {
    if parser.peek()?.kind == PyTokenType::Comma {
        let mut exprs = vec![left];
        parser.consume()?;
        let rest_exprs = parse_expr(parser, bp)?;
        match rest_exprs {
            PyExpr::Tuple {
                mut elts,
                parenthesized: false,
                ..
            } => {
                exprs.append(&mut elts);
            }
            _ => exprs.push(rest_exprs),
        }

        parser.optional_skip_one(PyTokenType::Comma)?;

        Ok(PyExpr::Tuple {
            elts: exprs,
            ctx: PyRefContext::Load,
            parenthesized: false,
        })
    } else {
        Ok(left)
    }
}
