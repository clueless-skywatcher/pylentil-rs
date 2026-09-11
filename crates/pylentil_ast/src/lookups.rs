use std::collections::HashMap;

use lazy_static::lazy_static;
use pylentil_common::errors::PylentilError;

use crate::{
    PyToken, PyTokenType,
    ast::{PyBinaryOp, PyComparisonOp, PyConstant, PyExpr, PyRefContext, PyStatement},
    parser::PyParser,
};

#[derive(Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
pub enum PyBindingPower {
    Default,
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

lazy_static! {
    static ref BP_LU: PyBindingPowerLookup = {
        let mut m = HashMap::new();

        // Atoms
        bp(&mut m, PyTokenType::Int, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::Float, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::String, PyBindingPower::Terminal);
        bp(&mut m, PyTokenType::Ident, PyBindingPower::Terminal);

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
        nud(&mut m, PyTokenType::Boolean, parse_terminal);
        nud(&mut m, PyTokenType::String, parse_terminal);
        nud(&mut m, PyTokenType::Ident, parse_terminal);
        nud(&mut m, PyTokenType::NoneValue, parse_terminal);

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

        m
    };
    static ref STMT_LU: PyStatementLookup = {
        let mut m = HashMap::new();

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
    let bool_val = match parser.consume()? {
        PyToken {
            kind: PyTokenType::Boolean,
            value: Some(value),
        } => match value {
            "True" => true,
            _ => false,
        },
        _ => return Err(PylentilError::InvalidSyntax),
    };

    Ok(PyExpr::Constant {
        value: PyConstant::Boolean(bool_val),
        kind: None,
    })
}

fn parse_none(parser: &mut PyParser) -> Result<PyExpr, PylentilError> {
    match parser.consume()? {
        PyToken {
            kind: PyTokenType::NoneValue,
            value: None,
        } => Ok(PyExpr::Constant {
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
        PyTokenType::Boolean => parse_boolean(parser),
        PyTokenType::String => parse_string(parser),
        PyTokenType::Ident => parse_ident(parser),
        PyTokenType::NoneValue => parse_none(parser),
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

    while *BP_LU.get(&parser.peek()?.kind).unwrap() > bp {
        let token_kind = parser.peek()?.kind;
        let Some(led_fn) = LED_LU.get(&token_kind) else {
            return Err(PylentilError::InvalidSyntax);
        };

        left = led_fn(parser, left, bp)?;
    }

    Ok(left)
}

pub fn parse_statement(parser: &mut PyParser) -> Result<PyStatement, PylentilError> {
    let token_kind = parser.peek()?.kind;

    match STMT_LU.get(&token_kind) {
        Some(stmt_fn) => Ok(stmt_fn(parser)?),
        None => {
            let expr = parse_expr(parser, PyBindingPower::Default)?;
            parser.expect(PyTokenType::Newline)?;

            Ok(PyStatement::Expr { value: expr })
        }
    }
}
