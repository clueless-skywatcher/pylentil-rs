#[path = "../common/mod.rs"]
mod common;

use common::{case, cases, expr, one_stmt, parse_failures, parse_module, parse_outcome, stmts, Outcome};
use pylentil_ast::ast::{PyExpr, PyRefContext, PyStatement};

/// Parse one named case and render it.
fn e(fixture: &str, name: &str) -> String {
    let code = case(fixture, name);
    match expr(&code) {
        Ok(rendered) => rendered,
        Err(err) => format!("<rejected: {err:?}>"),
    }
}

fn assert_rejected(fixture: &str, name: &str) {
    let code = case(fixture, name);
    match parse_outcome(&code) {
        Outcome::Err(_) => {}
        Outcome::Ok(ast) => panic!("accepted invalid Python `{}` as {ast:?}", code.trim()),
        Outcome::Panic(m) => panic!("panicked instead of erroring on `{}`: {m}", code.trim()),
    }
}

mod basics;
mod literals;
mod arithmetic;
mod bitwise;
mod comparisons;
mod boolean;
mod tuples;
mod assignment;
mod calls;
mod subscripts;
mod collections;
mod if_statements;
mod statements;
mod functions;
mod imports;
mod modules;
mod robustness;
