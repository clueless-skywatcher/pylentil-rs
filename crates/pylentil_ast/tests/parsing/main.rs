#[path = "../common/mod.rs"]
mod common;

use common::build::*;
use common::{Outcome, case, parse_module, parse_outcome};
use pylentil_ast::ast::{
    PyArg, PyArguments, PyBinaryOp, PyBoolOp, PyComparisonOp, PyExpr, PyStatement, PyUnaryOp,
};

/// The statements `code` parses to. Fails the test if it does not parse.
fn parse_body(code: &str) -> Vec<PyStatement> {
    match parse_module(code) {
        Ok(module) => module.body,
        Err(e) => panic!("failed to parse `{}`: {e:?}", code.trim()),
    }
}

/// The single statement `code` parses to.
fn parse_stmt(code: &str) -> PyStatement {
    let mut body = parse_body(code);
    assert_eq!(
        body.len(),
        1,
        "expected one statement in `{}`, got {body:#?}",
        code.trim()
    );
    body.remove(0)
}

/// The expression in `code`, which must be a single expression statement.
fn parse_expr(code: &str) -> PyExpr {
    match parse_stmt(code) {
        PyStatement::Expr { value } => *value,
        other => panic!(
            "expected an expression in `{}`, got {other:#?}",
            code.trim()
        ),
    }
}

/// The statements of one named fixture case.
fn body(fixture: &str, name: &str) -> Vec<PyStatement> {
    parse_body(&case(fixture, name))
}

/// The single statement of one named fixture case.
fn stmt(fixture: &str, name: &str) -> PyStatement {
    parse_stmt(&case(fixture, name))
}

/// The expression of one named fixture case.
fn expr(fixture: &str, name: &str) -> PyExpr {
    parse_expr(&case(fixture, name))
}

fn assert_rejected(fixture: &str, name: &str) {
    let code = case(fixture, name);
    match parse_outcome(&code) {
        Outcome::Err(_) => {}
        Outcome::Ok(ast) => panic!("accepted invalid Python `{}` as {ast:?}", code.trim()),
        Outcome::Panic(m) => panic!("panicked instead of erroring on `{}`: {m}", code.trim()),
    }
}

/// Parse one named case; fail with the reason if it errors or panics.
fn assert_parses(fixture: &str, name: &str) {
    let code = case(fixture, name);
    match parse_outcome(&code) {
        Outcome::Ok(_) => {}
        Outcome::Err(e) => panic!("rejected valid Python `{}`: {e:?}", code.trim()),
        Outcome::Panic(m) => panic!("panicked on `{}`: {m}", code.trim()),
    }
}

/// Parse one named case; accept or reject, but never panic.
fn assert_no_panic(fixture: &str, name: &str) {
    let code = case(fixture, name);
    if let Outcome::Panic(m) = parse_outcome(&code) {
        panic!("panicked on `{}`: {m}", code.trim());
    }
}

mod arithmetic;
mod assignment;
mod basics;
mod bitwise;
mod boolean;
mod calls;
mod collections;
mod comparisons;
mod functions;
mod if_statements;
mod imports;
mod literals;
mod modules;
mod robustness;
mod statements;
mod subscripts;
mod tuples;
