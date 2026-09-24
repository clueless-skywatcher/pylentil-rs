//! Minimal scaffolding so `lexing.rs` and `parsing.rs` can run today.
//!
//! This is a placeholder, not a framework — replace it. The only part the test
//! expectations actually depend on is `expr_sexpr` / `stmt_sexpr`: the tests
//! compare rendered s-expressions like `(+ 1 (* 2 3))`, so whatever you build
//! needs to keep producing that notation (or the expectations need rewriting).
//!
//! Fixtures live in `<workspace>/fixtures/{lexer,parser}/*.py`, each holding many
//! snippets introduced by a `# case: <name>` line. Headers are stripped before a
//! snippet reaches the lexer, which is what lets fixtures carry documentation
//! even for syntax that is not lexable yet.

#![allow(dead_code)]

use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;

use pylentil_ast::ast::{
    PyBinaryOp, PyBoolOp, PyComparisonOp, PyConstant, PyExpr, PyModule, PyStatement, PyUnaryOp,
};
use pylentil_ast::parser::PyParser;
use pylentil_ast::{PyLexer, PyToken, PyTokenType};
use pylentil_common::errors::PylentilError;

// ---------------------------------------------------------------- fixtures --

pub struct Case {
    pub name: String,
    pub code: String,
}

pub fn fixture_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(rel)
}

pub fn cases(rel: &str) -> Vec<Case> {
    let path = fixture_path(rel);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read fixture {}: {e}", path.display()));

    let mut out: Vec<Case> = Vec::new();
    let mut current: Option<(String, String)> = None;

    for line in text.lines() {
        match line.strip_prefix("# case:") {
            Some(name) => {
                if let Some((name, body)) = current.take() {
                    out.push(Case { name, code: trim_blank_edges(&body) });
                }
                current = Some((name.trim().to_string(), String::new()));
            }
            None => {
                if let Some((_, body)) = current.as_mut() {
                    body.push_str(line);
                    body.push('\n');
                }
            }
        }
    }
    if let Some((name, body)) = current.take() {
        out.push(Case { name, code: trim_blank_edges(&body) });
    }

    assert!(!out.is_empty(), "fixture {rel} contains no `# case:` blocks");
    out
}

/// One named case. Panics on a typo so a test cannot silently assert against
/// an empty snippet.
pub fn case(rel: &str, name: &str) -> String {
    let all = cases(rel);
    match all.iter().find(|c| c.name == name) {
        Some(c) => c.code.clone(),
        None => panic!(
            "fixture {rel} has no case `{name}`; it has: {}",
            all.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")
        ),
    }
}

fn trim_blank_edges(body: &str) -> String {
    let lines: Vec<&str> = body.lines().collect();
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
    let end = lines.iter().rposition(|l| !l.trim().is_empty()).map(|i| i + 1).unwrap_or(0);
    let mut out = lines[start..end].join("\n");
    out.push('\n');
    out
}

// ------------------------------------------------------------------ lexing --

pub fn lex(code: &str) -> Result<Vec<PyToken<'_>>, PylentilError> {
    let code = PyLexer::from_code(code);
    code.map(|lexer| lexer.tokens)
}

/// Token kinds, minus the insignificant `Whitespace` filler.
pub fn kinds(code: &str) -> Result<Vec<PyTokenType>, PylentilError> {
    Ok(lex(code)?
        .into_iter()
        .filter(|t| t.kind != PyTokenType::Whitespace)
        .map(|t| t.kind)
        .collect())
}

/// Only tokens carrying content: no whitespace, layout or end marker.
pub fn content(code: &str) -> Result<Vec<String>, PylentilError> {
    Ok(lex(code)?
        .iter()
        .filter(|t| {
            !matches!(
                t.kind,
                PyTokenType::Whitespace
                    | PyTokenType::Newline
                    | PyTokenType::Indent
                    | PyTokenType::Dedent
                    | PyTokenType::EOF
            )
        })
        .map(describe)
        .collect())
}

pub fn describe(token: &PyToken) -> String {
    match &token.value {
        Some(value) => format!("{:?}({})", token.kind, value),
        None => format!("{:?}", token.kind),
    }
}

pub fn count_kind(code: &str, kind: PyTokenType) -> Result<usize, PylentilError> {
    Ok(kinds(code)?.iter().filter(|k| **k == kind).count())
}

// ----------------------------------------------------------------- parsing --

pub fn parse_module(code: &str) -> Result<PyModule, PylentilError> {
    let tokens = lex(code)?;
    let mut parser = PyParser::new(tokens);
    parser.parse()
}

pub fn stmts(code: &str) -> Result<Vec<String>, PylentilError> {
    Ok(parse_module(code)?.body.iter().map(stmt_sexpr).collect())
}

pub fn one_stmt(code: &str) -> Result<String, PylentilError> {
    let rendered = stmts(code)?;
    assert_eq!(
        rendered.len(),
        1,
        "expected exactly one statement in {code:?}, got {rendered:?}"
    );
    Ok(rendered.into_iter().next().unwrap())
}

/// Shorthand for a snippet that is a single expression statement.
pub fn expr(code: &str) -> Result<String, PylentilError> {
    one_stmt(code)
}

/// Every case in a fixture that fails to parse, as `name: Error` lines.
pub fn parse_failures(fixture: &str) -> Vec<String> {
    cases(fixture)
        .iter()
        .filter_map(|c| match parse_outcome(&c.code) {
            Outcome::Ok(_) => None,
            Outcome::Err(e) => Some(format!("  {}: {e:?}", c.name)),
            Outcome::Panic(m) => Some(format!("  {}: PANIC {m}", c.name)),
        })
        .collect()
}

/// Every case in a fixture that fails to lex.
pub fn lex_failures(fixture: &str) -> Vec<String> {
    cases(fixture)
        .iter()
        .filter_map(|c| match lex_outcome(&c.code) {
            Outcome::Ok(_) => None,
            Outcome::Err(e) => Some(format!("  {}: {e:?}", c.name)),
            Outcome::Panic(m) => Some(format!("  {}: PANIC {m}", c.name)),
        })
        .collect()
}

// ------------------------------------------------------- outcome capturing --

/// A result that also survives a panic, so "this input is rejected" can be
/// asserted even where the implementation reaches an `assert!` or `unwrap`.
#[derive(Debug)]
pub enum Outcome<T> {
    Ok(T),
    Err(PylentilError),
    Panic(String),
}

pub fn catching<T>(f: impl FnOnce() -> Result<T, PylentilError>) -> Outcome<T> {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(value)) => Outcome::Ok(value),
        Ok(Err(e)) => Outcome::Err(e),
        Err(payload) => {
            let message = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic>".to_string());
            Outcome::Panic(message)
        }
    }
}

pub fn lex_outcome(code: &str) -> Outcome<Vec<PyTokenType>> {
    catching(|| kinds(code))
}

pub fn parse_outcome(code: &str) -> Outcome<Vec<String>> {
    catching(|| stmts(code))
}

// ------------------------------------------------------- s-expression forms --

pub fn stmt_sexpr(stmt: &PyStatement) -> String {
    match stmt {
        PyStatement::Expr { value } => expr_sexpr(value),
        PyStatement::Assign { targets, value, .. } => {
            format!("(assign ({}) {})", exprs_sexpr(targets), expr_sexpr(value))
        }
        PyStatement::AugAssign { target, op, value } => format!(
            "(augassign {} {}= {})",
            expr_sexpr(target),
            binary_symbol(*op),
            expr_sexpr(value)
        ),
        PyStatement::AnnAssign { target, annotation, value, .. } => format!(
            "(annassign {} {} {})",
            expr_sexpr(target),
            expr_sexpr(annotation),
            value.as_ref().map(|v| expr_sexpr(v)).unwrap_or_else(|| "-".into())
        ),
        PyStatement::If { test, body, orelse } => format!(
            "(if {} ({}) ({}))",
            expr_sexpr(test),
            stmts_sexpr(body),
            stmts_sexpr(orelse)
        ),
        PyStatement::While { test, body, orelse } => format!(
            "(while {} ({}) ({}))",
            expr_sexpr(test),
            stmts_sexpr(body),
            stmts_sexpr(orelse)
        ),
        PyStatement::For { target, iter, body, .. } => format!(
            "(for {} {} ({}))",
            expr_sexpr(target),
            expr_sexpr(iter),
            stmts_sexpr(body)
        ),
        PyStatement::Return { value } => match value {
            Some(v) => format!("(return {})", expr_sexpr(v)),
            None => "(return)".to_string(),
        },
        PyStatement::Delete { targets } => format!("(del {})", exprs_sexpr(targets)),
        PyStatement::Import { names } => format!("(import {})", aliases_sexpr(names)),
        PyStatement::ImportFrom { module, names, level } => format!(
            "(from {}{} {})",
            ".".repeat(level.unwrap_or(0).max(0) as usize),
            module.as_deref().unwrap_or(""),
            aliases_sexpr(names)
        ),
        PyStatement::Global { names } => format!("(global {})", names.join(" ")),
        PyStatement::Nonlocal { names } => format!("(nonlocal {})", names.join(" ")),
        PyStatement::FunctionDef { name, body, .. } => {
            format!("(def {name} ({}))", stmts_sexpr(body))
        }
        PyStatement::ClassDef { name, body, .. } => {
            format!("(class {name} ({}))", stmts_sexpr(body))
        }
        PyStatement::Pass => "pass".to_string(),
        PyStatement::Break => "break".to_string(),
        PyStatement::Continue => "continue".to_string(),
        other => format!("<{}>", variant_name(other)),
    }
}

pub fn expr_sexpr(expr: &PyExpr) -> String {
    match expr {
        PyExpr::Constant { value, .. } => constant_sexpr(value),
        PyExpr::Name { id, .. } => id.clone(),
        PyExpr::BinOp { left, op, right } => format!(
            "({} {} {})",
            binary_symbol(*op),
            expr_sexpr(left),
            expr_sexpr(right)
        ),
        PyExpr::UnaryOp { op, operand } => {
            format!("({} {})", unary_symbol(*op), expr_sexpr(operand))
        }
        PyExpr::BoolOp { op, values } => format!("({} {})", bool_symbol(*op), exprs_sexpr(values)),
        PyExpr::Compare { left, ops, comparators } => {
            let mut out = format!("(compare {}", expr_sexpr(left));
            for (op, comparator) in ops.iter().zip(comparators.iter()) {
                out.push_str(&format!(" {} {}", comparison_symbol(*op), expr_sexpr(comparator)));
            }
            out.push(')');
            out
        }
        PyExpr::Tuple { elts, parenthesized, .. } => format!(
            "({} {})",
            if *parenthesized { "ptuple" } else { "tuple" },
            exprs_sexpr(elts)
        ),
        PyExpr::List { elts, .. } => format!("(list {})", exprs_sexpr(elts)),
        PyExpr::Set { elts } => format!("(set {})", exprs_sexpr(elts)),
        PyExpr::Dict { keys, values } => {
            let pairs: Vec<String> = keys
                .iter()
                .zip(values.iter())
                .map(|(k, v)| match k {
                    Some(k) => format!("{}: {}", expr_sexpr(k), expr_sexpr(v)),
                    None => format!("**{}", expr_sexpr(v)),
                })
                .collect();
            format!("(dict {})", pairs.join(" "))
        }
        PyExpr::Call { func, args, keywords } => {
            let mut parts = vec![expr_sexpr(func)];
            parts.extend(args.iter()
                .map(|bx| bx.as_ref())
                .map(expr_sexpr)
            );
            parts.extend(keywords.iter().map(|kw| match &kw.arg {
                Some(name) => format!("{name}={}", expr_sexpr(&kw.value)),
                None => format!("**{}", expr_sexpr(&kw.value)),
            }));
            format!("(call {})", parts.join(" "))
        }
        PyExpr::Attribute { value, attr, .. } => format!("(attr {} {attr})", expr_sexpr(value)),
        PyExpr::Subscript { value, slice, .. } => {
            format!("(subscript {} {})", expr_sexpr(value), expr_sexpr(slice))
        }
        PyExpr::Slice { lower, upper, step } => format!(
            "(slice {} {} {})",
            optional_sexpr(lower.as_deref()),
            optional_sexpr(upper.as_deref()),
            optional_sexpr(step.as_deref())
        ),
        PyExpr::Starred { value, .. } => format!("(star {})", expr_sexpr(value)),
        PyExpr::IfExp { test, body, orelse } => format!(
            "(ifexp {} {} {})",
            expr_sexpr(test),
            expr_sexpr(body),
            expr_sexpr(orelse)
        ),
        PyExpr::NamedExpr { target, value } => {
            format!("(:= {} {})", expr_sexpr(target), expr_sexpr(value))
        }
        PyExpr::Lambda { body, .. } => format!("(lambda {})", expr_sexpr(body)),
        PyExpr::Await { value } => format!("(await {})", expr_sexpr(value)),
        PyExpr::Yield { value } => match value {
            Some(v) => format!("(yield {})", expr_sexpr(v)),
            None => "(yield)".to_string(),
        },
        PyExpr::YieldFrom { value } => format!("(yieldfrom {})", expr_sexpr(value)),
        other => format!("<{}>", variant_name(other)),
    }
}

fn optional_sexpr(expr: Option<&PyExpr>) -> String {
    expr.map(expr_sexpr).unwrap_or_else(|| "-".to_string())
}

fn exprs_sexpr(exprs: &[PyExpr]) -> String {
    exprs.iter().map(expr_sexpr).collect::<Vec<_>>().join(" ")
}

/// `name` for a plain alias, `name:asname` for an aliased one.
fn aliases_sexpr(aliases: &[pylentil_ast::ast::PyAlias]) -> String {
    aliases
        .iter()
        .map(|a| match &a.asname {
            Some(asname) => format!("{}:{asname}", a.name),
            None => a.name.clone(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn stmts_sexpr(stmts: &[PyStatement]) -> String {
    stmts.iter().map(stmt_sexpr).collect::<Vec<_>>().join(" ")
}

fn constant_sexpr(constant: &PyConstant) -> String {
    match constant {
        PyConstant::Integer(i) => i.to_string(),
        PyConstant::Float(f) => f.to_string(),
        // Raw source text between the quotes, so expectations stay readable.
        PyConstant::String(s) => format!("str({s})"),
        PyConstant::Bytes(b) => format!("bytes({})", b.len()),
        PyConstant::Boolean(true) => "True".to_string(),
        PyConstant::Boolean(false) => "False".to_string(),
        PyConstant::Complex(re, im) => {
            format!("(complex {} {})", constant_sexpr(re), constant_sexpr(im))
        }
        PyConstant::None => "None".to_string(),
        PyConstant::Ellipsis => "...".to_string(),
    }
}

fn binary_symbol(op: PyBinaryOp) -> &'static str {
    match op {
        PyBinaryOp::Add => "+",
        PyBinaryOp::Sub => "-",
        PyBinaryOp::Mul => "*",
        PyBinaryOp::Div => "/",
        PyBinaryOp::FloorDiv => "//",
        PyBinaryOp::Mod => "%",
        PyBinaryOp::Pow => "**",
        PyBinaryOp::LShift => "<<",
        PyBinaryOp::RShift => ">>",
        PyBinaryOp::BitOr => "|",
        PyBinaryOp::BitXor => "^",
        PyBinaryOp::BitAnd => "&",
        PyBinaryOp::MatMult => "@",
    }
}

fn unary_symbol(op: PyUnaryOp) -> &'static str {
    match op {
        PyUnaryOp::UnarySub => "neg",
        PyUnaryOp::UnaryAdd => "pos",
        PyUnaryOp::Not => "not",
        PyUnaryOp::Invert => "~",
    }
}

fn bool_symbol(op: PyBoolOp) -> &'static str {
    match op {
        PyBoolOp::And => "and",
        PyBoolOp::Or => "or",
    }
}

fn comparison_symbol(op: PyComparisonOp) -> &'static str {
    match op {
        PyComparisonOp::Lt => "<",
        PyComparisonOp::Lte => "<=",
        PyComparisonOp::Gt => ">",
        PyComparisonOp::Gte => ">=",
        PyComparisonOp::Eq => "==",
        PyComparisonOp::NotEq => "!=",
        PyComparisonOp::Is => "is",
        PyComparisonOp::IsNot => "is-not",
        PyComparisonOp::In => "in",
        PyComparisonOp::NotIn => "not-in",
    }
}

fn variant_name<T: std::fmt::Debug>(value: &T) -> String {
    format!("{value:?}")
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}
