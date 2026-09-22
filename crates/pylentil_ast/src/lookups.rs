mod expr;
mod stmt;

use std::collections::HashMap;

use lazy_static::lazy_static;
use pylentil_common::errors::PylentilError;

use crate::{
    PyTokenType, ast::{PyExpr, PyStatement}, lookups::{expr::{parse_attribute_access, parse_function_call, parse_star, parse_subscript_access}, stmt::{parse_stmt_break, parse_stmt_continue, parse_stmt_pass}}, parser::PyParser,
};

use expr::{
    as_target, augmented_op, parse_binary, parse_bool_op, parse_comparison,
    parse_potentially_comma_separated, parse_terminal, parse_tuple_or_expr, parse_unary,
};
use stmt::parse_stmt_if;

#[derive(Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
pub enum PyBindingPower {
    Default = 1,
    Comma,
    Ternary,
    Or,
    And,
    Not,
    Comparison,
    BitOr,
    BitXor,
    BitAnd,
    Shift,
    Additive,
    Multiplicative,
    Unary,
    Power,
    Postfix,
    Terminal,
}

impl PyBindingPower {
    fn one_below(self) -> Self {
        match self {
            PyBindingPower::Default => PyBindingPower::Default,
            PyBindingPower::Comma => PyBindingPower::Default,
            PyBindingPower::Ternary => PyBindingPower::Comma,
            PyBindingPower::Or => PyBindingPower::Ternary,
            PyBindingPower::And => PyBindingPower::Or,
            PyBindingPower::Not => PyBindingPower::And,
            PyBindingPower::Comparison => PyBindingPower::Not,
            PyBindingPower::BitOr => PyBindingPower::Comparison,
            PyBindingPower::BitXor => PyBindingPower::BitOr,
            PyBindingPower::BitAnd => PyBindingPower::BitXor,
            PyBindingPower::Shift => PyBindingPower::BitAnd,
            PyBindingPower::Additive => PyBindingPower::Shift,
            PyBindingPower::Multiplicative => PyBindingPower::Additive,
            PyBindingPower::Unary => PyBindingPower::Multiplicative,
            PyBindingPower::Power => PyBindingPower::Unary,
            PyBindingPower::Postfix => PyBindingPower::Power,
            PyBindingPower::Terminal => PyBindingPower::Postfix,
        }
    }
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
    static ref RIGHT_ASSOC: Vec<PyTokenType> = vec![
        PyTokenType::DoubleStar
    ];

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
        bp(&mut m, PyTokenType::Ellipsis, PyBindingPower::Terminal);

        // Postfix: attr, call, subscript
        bp(&mut m, PyTokenType::Dot, PyBindingPower::Postfix);
        bp(&mut m, PyTokenType::LParen, PyBindingPower::Postfix);
        bp(&mut m, PyTokenType::LSquare, PyBindingPower::Postfix);

        // Unary
        bp(&mut m, PyTokenType::Tilde, PyBindingPower::Unary);

        // Multiplicative
        bp(&mut m, PyTokenType::Star, PyBindingPower::Multiplicative);
        bp(&mut m, PyTokenType::Slash, PyBindingPower::Multiplicative);
        bp(&mut m, PyTokenType::DoubleSlash, PyBindingPower::Multiplicative);
        bp(&mut m, PyTokenType::Percent, PyBindingPower::Multiplicative);
        bp(&mut m, PyTokenType::At, PyBindingPower::Multiplicative);

        // Exponents
        bp(&mut m, PyTokenType::DoubleStar, PyBindingPower::Power);

        // Additive / shifts
        bp(&mut m, PyTokenType::Plus, PyBindingPower::Additive);
        bp(&mut m, PyTokenType::Minus, PyBindingPower::Additive);
        bp(&mut m, PyTokenType::LShift, PyBindingPower::Shift);
        bp(&mut m, PyTokenType::RShift, PyBindingPower::Shift);

        // Bitwise
        bp(&mut m, PyTokenType::Ampersand, PyBindingPower::BitAnd);
        bp(&mut m, PyTokenType::Caret, PyBindingPower::BitXor);
        bp(&mut m, PyTokenType::VerticalBar, PyBindingPower::BitOr);

        // Boolean
        bp(&mut m, PyTokenType::Or, PyBindingPower::Or);
        bp(&mut m, PyTokenType::And, PyBindingPower::And);

        // Comparisons
        bp(&mut m, PyTokenType::DoubleEqual, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::NotEqual, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::Less, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::Greater, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::LessEqual, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::GreaterEqual, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::Is, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::In, PyBindingPower::Comparison);
        bp(&mut m, PyTokenType::Not, PyBindingPower::Comparison);

        // Assignment / separators
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
        nud(&mut m, PyTokenType::Ellipsis, parse_terminal);
        nud(&mut m, PyTokenType::LParen, parse_tuple_or_expr);

        // Unary
        nud(&mut m, PyTokenType::Minus, parse_unary);
        nud(&mut m, PyTokenType::Plus, parse_unary);
        nud(&mut m, PyTokenType::Tilde, parse_unary);
        nud(&mut m, PyTokenType::Not, parse_unary);

        // Star
        nud(&mut m, PyTokenType::Star, parse_star);

        m
    };
    static ref LED_LU: PyLEDLookup = {
        let mut m = HashMap::new();

        // Multiplicative / power
        led(&mut m, PyTokenType::Star, parse_binary);
        led(&mut m, PyTokenType::Slash, parse_binary);
        led(&mut m, PyTokenType::DoubleSlash, parse_binary);
        led(&mut m, PyTokenType::Percent, parse_binary);
        led(&mut m, PyTokenType::At, parse_binary);
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

        // Boolean
        led(&mut m, PyTokenType::Or, parse_bool_op);
        led(&mut m, PyTokenType::And, parse_bool_op);

        // Relational
        led(&mut m, PyTokenType::DoubleEqual, parse_comparison);
        led(&mut m, PyTokenType::NotEqual, parse_comparison);
        led(&mut m, PyTokenType::Less, parse_comparison);
        led(&mut m, PyTokenType::Greater, parse_comparison);
        led(&mut m, PyTokenType::LessEqual, parse_comparison);
        led(&mut m, PyTokenType::GreaterEqual, parse_comparison);
        led(&mut m, PyTokenType::Is, parse_comparison);
        led(&mut m, PyTokenType::In, parse_comparison);
        led(&mut m, PyTokenType::Not, parse_comparison);

        // Comma
        led(&mut m, PyTokenType::Comma, parse_potentially_comma_separated);

        // Attribute access
        led(&mut m, PyTokenType::Dot, parse_attribute_access);

        // Subscript access
        led(&mut m, PyTokenType::LSquare, parse_subscript_access);

        // Function calls
        led(&mut m, PyTokenType::LParen, parse_function_call);

        m
    };
    static ref STMT_LU: PyStatementLookup = {
        let mut m = HashMap::new();

        stmt(&mut m, PyTokenType::If, parse_stmt_if);
        stmt(&mut m, PyTokenType::Pass, parse_stmt_pass);
        stmt(&mut m, PyTokenType::Break, parse_stmt_break);
        stmt(&mut m, PyTokenType::Continue, parse_stmt_continue);

        m
    };
}

fn parse_expr(parser: &mut PyParser, bp: PyBindingPower) -> Result<PyExpr, PylentilError> {
    let token = parser.peek()?;
    let Some(nud_fn) = NUD_LU.get(&token.kind) else {
        return Err(PylentilError::ExpressionExpected {
            found: token.describe(),
        });
    };

    let mut left = nud_fn(parser)?;

    while BP_LU
        .get(&parser.peek()?.kind)
        .copied()
        .unwrap_or(PyBindingPower::Default)
        > bp
    {
        let token = parser.peek()?;
        let token_kind = token.kind;
        let Some(led_fn) = LED_LU.get(&token_kind) else {
            return Err(PylentilError::TokenCannotContinueExpression {
                found: token.describe(),
            });
        };

        let op_bp = BP_LU
            .get(&token_kind)
            .copied()
            .unwrap_or(PyBindingPower::Default);

        let op_bp = match RIGHT_ASSOC.contains(&token_kind) {
            true => op_bp.one_below(),
            false => op_bp,
        };

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

            Ok(match parser.peek()?.kind {
                PyTokenType::Assign => {
                    parser.consume()?;
                    let right = parse_expr(parser, PyBindingPower::Default)?;
                    let targets = match expr {
                        PyExpr::Tuple { elts, .. } => elts,
                        _ => vec![expr],
                    };
                    let targets = targets
                        .into_iter()
                        .map(as_target)
                        .collect::<Result<Vec<_>, _>>()?;

                    PyStatement::Assign {
                        targets,
                        value: Box::new(right),
                        type_comment: None,
                    }
                }
                kind if augmented_op(kind).is_some() => {
                    let token = parser.consume()?;
                    let op = augmented_op(kind).ok_or(PylentilError::UnsupportedOperator {
                        found: token.describe(),
                        context: "an augmented assignment",
                    })?;
                    let value = parse_expr(parser, PyBindingPower::Default)?;

                    PyStatement::AugAssign {
                        target: Box::new(as_target(expr)?),
                        op,
                        value: Box::new(value),
                    }
                }
                PyTokenType::Colon => {
                    parser.consume()?;

                    if !matches!(expr, PyExpr::Name { .. }) {
                        return Err(PylentilError::InvalidAnnotationTarget {
                            found: expr.describe(),
                        });
                    }

                    let annotation = parse_expr(parser, PyBindingPower::Default)?;

                    if !matches!(annotation, PyExpr::Name { .. }) {
                        return Err(PylentilError::InvalidAnnotation {
                            found: annotation.describe(),
                        });
                    }

                    if parser.peek()?.kind == PyTokenType::Assign {
                        parser.consume()?;

                        let value = parse_expr(parser, PyBindingPower::Default)?;

                        PyStatement::AnnAssign {
                            target: Box::new(expr),
                            annotation: Box::new(annotation),
                            value: Some(Box::new(value)),
                            simple: false,
                        }
                    } else {
                        PyStatement::AnnAssign {
                            target: Box::new(expr),
                            annotation: Box::new(annotation),
                            value: None,
                            simple: false,
                        }
                    }
                }

                _ => PyStatement::Expr {
                    value: Box::new(expr),
                },
            })
        }
    }
}
