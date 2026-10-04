//! Constructors for expected AST values.
//!
//! Each function builds a real `PyExpr` or `PyStatement`, named after the
//! variant it makes and taking that variant's fields in declaration order.
//! Fields a test rarely states are filled in the way the parser fills them:
//! names and containers are `Load`, `kind` and `type_comment` are `None`.
//! Wrap an assignment target in [`store`] to get the `Store` form.
//!
//! Every span is `PySpan::default()`. Spans never affect `==`, so these values
//! match parsed ASTs wherever the parsed code sits.

use pylentil_ast::ast::{
    PyAlias, PyArg, PyArgType, PyArguments, PyBinaryOp, PyBoolOp, PyComparisonOp, PyConstant,
    PyExceptHandler, PyExpr, PyKeyword, PyRefContext, PyStatement, PyUnaryOp, PyWithItem,
};
use pylentil_common::span::PySpan;

const NO_SPAN: PySpan = PySpan {
    start: 0,
    end: None,
};

// --------------------------------------------------------------- literals --

fn constant(value: PyConstant) -> PyExpr {
    PyExpr::Constant {
        value,
        kind: None,
        span: NO_SPAN,
    }
}

pub fn int(value: impl ToString) -> PyExpr {
    constant(PyConstant::Integer(value.to_string()))
}

pub fn float(text: &str) -> PyExpr {
    constant(PyConstant::Float(text.to_string()))
}

/// A string literal; `text` is the source text between the quotes.
pub fn string(text: &str) -> PyExpr {
    constant(PyConstant::String(text.to_string()))
}

pub fn boolean(value: bool) -> PyExpr {
    constant(PyConstant::Boolean(value))
}

pub fn none() -> PyExpr {
    constant(PyConstant::None)
}

pub fn ellipsis() -> PyExpr {
    constant(PyConstant::Ellipsis)
}

pub fn name(id: &str) -> PyExpr {
    PyExpr::Name {
        id: id.to_string(),
        ctx: PyRefContext::Load,
        span: NO_SPAN,
    }
}

// -------------------------------------------------------------- operators --

pub fn bin_op(left: PyExpr, op: PyBinaryOp, right: PyExpr) -> PyExpr {
    PyExpr::BinOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
        span: NO_SPAN,
    }
}

pub fn unary_op(op: PyUnaryOp, operand: PyExpr) -> PyExpr {
    PyExpr::UnaryOp {
        op,
        operand: Box::new(operand),
        span: NO_SPAN,
    }
}

pub fn bool_op(op: PyBoolOp, values: Vec<PyExpr>) -> PyExpr {
    PyExpr::BoolOp {
        op,
        values,
        span: NO_SPAN,
    }
}

pub fn compare(left: PyExpr, ops: Vec<PyComparisonOp>, comparators: Vec<PyExpr>) -> PyExpr {
    PyExpr::Compare {
        left: Box::new(left),
        ops,
        comparators,
        span: NO_SPAN,
    }
}

pub fn if_exp(test: PyExpr, body: PyExpr, orelse: PyExpr) -> PyExpr {
    PyExpr::IfExp {
        test: Box::new(test),
        body: Box::new(body),
        orelse: Box::new(orelse),
        span: NO_SPAN,
    }
}

pub fn named_expr(target: PyExpr, value: PyExpr) -> PyExpr {
    PyExpr::NamedExpr {
        target: Box::new(target),
        value: Box::new(value),
        span: NO_SPAN,
    }
}

// ------------------------------------------------------------ collections --

pub fn tuple(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::Tuple {
        elts,
        ctx: PyRefContext::Load,
        parenthesized: false,
        span: NO_SPAN,
    }
}

pub fn parenthesized_tuple(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::Tuple {
        elts,
        ctx: PyRefContext::Load,
        parenthesized: true,
        span: NO_SPAN,
    }
}

pub fn list(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::List {
        elts,
        ctx: PyRefContext::Load,
        span: NO_SPAN,
    }
}

pub fn set(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::Set {
        elts,
        span: NO_SPAN,
    }
}

/// A dict literal from `(key, value)` pairs.
pub fn dict(entries: Vec<(PyExpr, PyExpr)>) -> PyExpr {
    let (keys, values) = entries.into_iter().map(|(k, v)| (Some(k), v)).unzip();
    PyExpr::Dict {
        keys,
        values,
        span: NO_SPAN,
    }
}

// ------------------------------------------------------- access and calls --

pub fn attribute(value: PyExpr, attr: &str) -> PyExpr {
    PyExpr::Attribute {
        value: Box::new(value),
        attr: attr.to_string(),
        ctx: PyRefContext::Load,
        span: NO_SPAN,
    }
}

pub fn subscript(value: PyExpr, slice: PyExpr) -> PyExpr {
    PyExpr::Subscript {
        value: Box::new(value),
        slice: Box::new(slice),
        ctx: PyRefContext::Load,
        span: NO_SPAN,
    }
}

pub fn slice(lower: Option<PyExpr>, upper: Option<PyExpr>, step: Option<PyExpr>) -> PyExpr {
    PyExpr::Slice {
        lower: lower.map(Box::new),
        upper: upper.map(Box::new),
        step: step.map(Box::new),
        span: NO_SPAN,
    }
}

pub fn yield_expr(value: Option<PyExpr>) -> PyExpr {
    PyExpr::Yield {
        value: value.map(Box::new),
        span: NO_SPAN,
    }
}

pub fn starred(value: PyExpr) -> PyExpr {
    PyExpr::Starred {
        value: Box::new(value),
        ctx: PyRefContext::Load,
        span: NO_SPAN,
    }
}

pub fn call(func: PyExpr, args: Vec<PyExpr>, keywords: Vec<PyKeyword>) -> PyExpr {
    PyExpr::Call {
        func: Box::new(func),
        args: args.into_iter().map(Box::new).collect(),
        keywords,
        span: NO_SPAN,
    }
}

/// `arg=value`, or `**value` when `arg` is `None`.
pub fn keyword(arg: Option<&str>, value: PyExpr) -> PyKeyword {
    PyKeyword {
        arg: arg.map(str::to_string),
        value: Box::new(value),
        span: NO_SPAN,
    }
}

// ---------------------------------------------------------------- targets --

/// The `Store` form of an assignment target, as the parser makes it: names,
/// tuples, lists and starred items are converted all the way down; for an
/// attribute or subscript only the outer node is a store.
pub fn store(target: PyExpr) -> PyExpr {
    let ctx = PyRefContext::Store;
    match target {
        PyExpr::Name { id, span, .. } => PyExpr::Name { id, ctx, span },
        PyExpr::Tuple {
            elts,
            parenthesized,
            span,
            ..
        } => PyExpr::Tuple {
            elts: elts.into_iter().map(store).collect(),
            ctx,
            parenthesized,
            span,
        },
        PyExpr::List { elts, span, .. } => PyExpr::List {
            elts: elts.into_iter().map(store).collect(),
            ctx,
            span,
        },
        PyExpr::Starred { value, span, .. } => PyExpr::Starred {
            value: Box::new(store(*value)),
            ctx,
            span,
        },
        PyExpr::Attribute {
            value, attr, span, ..
        } => PyExpr::Attribute {
            value,
            attr,
            ctx,
            span,
        },
        PyExpr::Subscript {
            value, slice, span, ..
        } => PyExpr::Subscript {
            value,
            slice,
            ctx,
            span,
        },
        other => panic!("{} cannot be an assignment target", other.describe()),
    }
}

// ------------------------------------------------------------- statements --

pub fn pass() -> PyStatement {
    PyStatement::Pass { span: NO_SPAN }
}

pub fn break_() -> PyStatement {
    PyStatement::Break { span: NO_SPAN }
}

pub fn continue_() -> PyStatement {
    PyStatement::Continue { span: NO_SPAN }
}

pub fn expr_stmt(value: PyExpr) -> PyStatement {
    PyStatement::Expr {
        value: Box::new(value),
        span: NO_SPAN,
    }
}

pub fn assign(targets: Vec<PyExpr>, value: PyExpr) -> PyStatement {
    PyStatement::Assign {
        targets,
        value: Box::new(value),
        type_comment: None,
        span: NO_SPAN,
    }
}

pub fn aug_assign(target: PyExpr, op: PyBinaryOp, value: PyExpr) -> PyStatement {
    PyStatement::AugAssign {
        target: Box::new(target),
        op,
        value: Box::new(value),
        span: NO_SPAN,
    }
}

/// `target: annotation [= value]`. The parser leaves the target as a load and
/// sets `simple` to `false`, so this does too.
pub fn ann_assign(target: PyExpr, annotation: PyExpr, value: Option<PyExpr>) -> PyStatement {
    PyStatement::AnnAssign {
        target: Box::new(target),
        annotation: Box::new(annotation),
        value: value.map(Box::new),
        simple: false,
        span: NO_SPAN,
    }
}

pub fn if_stmt(test: PyExpr, body: Vec<PyStatement>, orelse: Vec<PyStatement>) -> PyStatement {
    PyStatement::If {
        test: Box::new(test),
        body,
        orelse,
        span: NO_SPAN,
    }
}

pub fn while_stmt(test: PyExpr, body: Vec<PyStatement>, orelse: Vec<PyStatement>) -> PyStatement {
    PyStatement::While {
        test: Box::new(test),
        body,
        orelse,
        span: NO_SPAN,
    }
}

pub fn for_stmt(target: PyExpr, iter: PyExpr, body: Vec<PyStatement>) -> PyStatement {
    PyStatement::For {
        target: Box::new(target),
        iter: Box::new(iter),
        body,
        orelse: vec![],
        type_comment: None,
        span: NO_SPAN,
    }
}

pub fn delete(targets: Vec<PyExpr>) -> PyStatement {
    PyStatement::Delete {
        targets,
        span: NO_SPAN,
    }
}

pub fn assert_stmt(test: PyExpr, msg: Option<PyExpr>) -> PyStatement {
    PyStatement::Assert {
        test: Box::new(test),
        msg: msg.map(Box::new),
        span: NO_SPAN,
    }
}

pub fn global(names: &[&str]) -> PyStatement {
    PyStatement::Global {
        names: names.iter().map(|name| name.to_string()).collect(),
        span: NO_SPAN,
    }
}

pub fn nonlocal(names: &[&str]) -> PyStatement {
    PyStatement::Nonlocal {
        names: names.iter().map(|name| name.to_string()).collect(),
        span: NO_SPAN,
    }
}

pub fn with_item(context_expr: PyExpr, optional_vars: Option<PyExpr>) -> PyWithItem {
    PyWithItem {
        context_expr: Box::new(context_expr),
        optional_vars: optional_vars.map(Box::new),
        span: NO_SPAN,
    }
}

pub fn except_handler(
    type_: Option<PyExpr>,
    name: Option<&str>,
    body: Vec<PyStatement>,
) -> PyExceptHandler {
    PyExceptHandler {
        type_: type_.map(Box::new),
        name: name.map(str::to_string),
        body,
        span: NO_SPAN,
    }
}

pub fn try_stmt(
    body: Vec<PyStatement>,
    handlers: Vec<PyExceptHandler>,
    orelse: Vec<PyStatement>,
    finalbody: Vec<PyStatement>,
) -> PyStatement {
    PyStatement::Try {
        body,
        handlers,
        orelse,
        finalbody,
        span: NO_SPAN,
    }
}

pub fn with_stmt(items: Vec<PyWithItem>, body: Vec<PyStatement>) -> PyStatement {
    PyStatement::With {
        items,
        body,
        type_comment: None,
        span: NO_SPAN,
    }
}

pub fn return_stmt(value: Option<PyExpr>) -> PyStatement {
    PyStatement::Return {
        value: value.map(Box::new),
        span: NO_SPAN,
    }
}

pub fn import(names: Vec<PyAlias>) -> PyStatement {
    PyStatement::Import {
        names,
        span: NO_SPAN,
    }
}

/// `from <level dots><module> import <names>`. `level` is `None` for an
/// absolute import.
pub fn import_from(module: Option<&str>, names: Vec<PyAlias>, level: Option<i32>) -> PyStatement {
    PyStatement::ImportFrom {
        module: module.map(str::to_string),
        names,
        level,
        span: NO_SPAN,
    }
}

pub fn alias(name: &str, asname: Option<&str>) -> PyAlias {
    PyAlias {
        name: name.to_string(),
        asname: asname.map(str::to_string),
        span: NO_SPAN,
    }
}

/// A plain `def` with no decorators or return annotation.
pub fn function_def(name: &str, args: PyArguments, body: Vec<PyStatement>) -> PyStatement {
    PyStatement::FunctionDef {
        name: name.to_string(),
        args: Box::new(args),
        body,
        decorator_list: vec![],
        returns: None,
        type_comment: None,
        type_params: vec![],
        is_async: false,
        span: NO_SPAN,
    }
}

pub fn class_def(name: &str, body: Vec<PyStatement>) -> PyStatement {
    class_def_with(name, vec![], body, vec![])
}

pub fn class_def_with(
    name: &str,
    bases: Vec<PyArgType>,
    body: Vec<PyStatement>,
    decorator_list: Vec<PyExpr>,
) -> PyStatement {
    PyStatement::ClassDef {
        name: name.to_string(),
        bases,
        keywords: vec![],
        body,
        decorator_list,
        type_params: vec![],
        span: NO_SPAN,
    }
}

/// A positional base. Keyword bases stay in `bases` as [`PyArgType::Keyword`];
/// `ClassDef::keywords` is left empty.
pub fn class_base(expr: PyExpr) -> PyArgType {
    PyArgType::Arg(PyArg {
        arg: Box::new(expr),
        annotation: None,
        type_comment: None,
        span: NO_SPAN,
    })
}

pub fn class_keyword(arg_name: Option<&str>, value: PyExpr) -> PyArgType {
    PyArgType::Keyword {
        keyword: keyword(arg_name, value),
        annotation: None,
        name_span: NO_SPAN,
        arg_span: NO_SPAN,
    }
}

// ------------------------------------------------------------- parameters --

/// A parameter. Its name is a `Load` name, which is how the parser builds a
/// plain positional parameter.
pub fn arg(name_: &str) -> PyArg {
    PyArg {
        arg: Box::new(name(name_)),
        annotation: None,
        type_comment: None,
        span: NO_SPAN,
    }
}

pub fn annotated_arg(name_: &str, annotation: PyExpr) -> PyArg {
    PyArg {
        annotation: Some(Box::new(annotation)),
        ..arg(name_)
    }
}

/// `*args`. The parser keeps the star, so the name is wrapped in `Starred`.
pub fn vararg(name_: &str) -> PyArg {
    PyArg {
        arg: Box::new(starred(name(name_))),
        annotation: None,
        type_comment: None,
        span: NO_SPAN,
    }
}
