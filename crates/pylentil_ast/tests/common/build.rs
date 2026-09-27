//! Constructors for expected AST values.
//!
//! Each function builds a real `PyExpr` or `PyStatement`, named after the
//! variant it makes and taking that variant's fields in declaration order.
//! Fields a test rarely states are filled in the way the parser fills them:
//! names and containers are `Load`, `kind` and `type_comment` are `None`.
//! Wrap an assignment target in [`store`] to get the `Store` form.

use pylentil_ast::ast::{
    PyAlias, PyArg, PyArguments, PyBinaryOp, PyBoolOp, PyComparisonOp, PyConstant, PyExpr,
    PyKeyword, PyRefContext, PyStatement, PyUnaryOp,
};

// --------------------------------------------------------------- literals --

fn constant(value: PyConstant) -> PyExpr {
    PyExpr::Constant { value, kind: None }
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
    }
}

// -------------------------------------------------------------- operators --

pub fn bin_op(left: PyExpr, op: PyBinaryOp, right: PyExpr) -> PyExpr {
    PyExpr::BinOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    }
}

pub fn unary_op(op: PyUnaryOp, operand: PyExpr) -> PyExpr {
    PyExpr::UnaryOp {
        op,
        operand: Box::new(operand),
    }
}

pub fn bool_op(op: PyBoolOp, values: Vec<PyExpr>) -> PyExpr {
    PyExpr::BoolOp { op, values }
}

pub fn compare(left: PyExpr, ops: Vec<PyComparisonOp>, comparators: Vec<PyExpr>) -> PyExpr {
    PyExpr::Compare {
        left: Box::new(left),
        ops,
        comparators,
    }
}

pub fn if_exp(test: PyExpr, body: PyExpr, orelse: PyExpr) -> PyExpr {
    PyExpr::IfExp {
        test: Box::new(test),
        body: Box::new(body),
        orelse: Box::new(orelse),
    }
}

pub fn named_expr(target: PyExpr, value: PyExpr) -> PyExpr {
    PyExpr::NamedExpr {
        target: Box::new(target),
        value: Box::new(value),
    }
}

// ------------------------------------------------------------ collections --

pub fn tuple(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::Tuple {
        elts,
        ctx: PyRefContext::Load,
        parenthesized: false,
    }
}

pub fn parenthesized_tuple(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::Tuple {
        elts,
        ctx: PyRefContext::Load,
        parenthesized: true,
    }
}

pub fn list(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::List {
        elts,
        ctx: PyRefContext::Load,
    }
}

pub fn set(elts: Vec<PyExpr>) -> PyExpr {
    PyExpr::Set { elts }
}

/// A dict literal from `(key, value)` pairs.
pub fn dict(entries: Vec<(PyExpr, PyExpr)>) -> PyExpr {
    let (keys, values) = entries.into_iter().map(|(k, v)| (Some(k), v)).unzip();
    PyExpr::Dict { keys, values }
}

// ------------------------------------------------------- access and calls --

pub fn attribute(value: PyExpr, attr: &str) -> PyExpr {
    PyExpr::Attribute {
        value: Box::new(value),
        attr: attr.to_string(),
        ctx: PyRefContext::Load,
    }
}

pub fn subscript(value: PyExpr, slice: PyExpr) -> PyExpr {
    PyExpr::Subscript {
        value: Box::new(value),
        slice: Box::new(slice),
        ctx: PyRefContext::Load,
    }
}

pub fn slice(lower: Option<PyExpr>, upper: Option<PyExpr>, step: Option<PyExpr>) -> PyExpr {
    PyExpr::Slice {
        lower: lower.map(Box::new),
        upper: upper.map(Box::new),
        step: step.map(Box::new),
    }
}

pub fn starred(value: PyExpr) -> PyExpr {
    PyExpr::Starred {
        value: Box::new(value),
        ctx: PyRefContext::Load,
    }
}

pub fn call(func: PyExpr, args: Vec<PyExpr>, keywords: Vec<PyKeyword>) -> PyExpr {
    PyExpr::Call {
        func: Box::new(func),
        args: args.into_iter().map(Box::new).collect(),
        keywords,
    }
}

/// `arg=value`, or `**value` when `arg` is `None`.
pub fn keyword(arg: Option<&str>, value: PyExpr) -> PyKeyword {
    PyKeyword {
        arg: arg.map(str::to_string),
        value: Box::new(value),
    }
}

// ---------------------------------------------------------------- targets --

/// The `Store` form of an assignment target, as the parser makes it: names,
/// tuples, lists and starred items are converted all the way down; for an
/// attribute or subscript only the outer node is a store.
pub fn store(target: PyExpr) -> PyExpr {
    let ctx = PyRefContext::Store;
    match target {
        PyExpr::Name { id, .. } => PyExpr::Name { id, ctx },
        PyExpr::Tuple {
            elts,
            parenthesized,
            ..
        } => PyExpr::Tuple {
            elts: elts.into_iter().map(store).collect(),
            ctx,
            parenthesized,
        },
        PyExpr::List { elts, .. } => PyExpr::List {
            elts: elts.into_iter().map(store).collect(),
            ctx,
        },
        PyExpr::Starred { value, .. } => PyExpr::Starred {
            value: Box::new(store(*value)),
            ctx,
        },
        PyExpr::Attribute { value, attr, .. } => PyExpr::Attribute { value, attr, ctx },
        PyExpr::Subscript { value, slice, .. } => PyExpr::Subscript { value, slice, ctx },
        other => panic!("{} cannot be an assignment target", other.describe()),
    }
}

// ------------------------------------------------------------- statements --

pub fn expr_stmt(value: PyExpr) -> PyStatement {
    PyStatement::Expr {
        value: Box::new(value),
    }
}

pub fn assign(targets: Vec<PyExpr>, value: PyExpr) -> PyStatement {
    PyStatement::Assign {
        targets,
        value: Box::new(value),
        type_comment: None,
    }
}

pub fn aug_assign(target: PyExpr, op: PyBinaryOp, value: PyExpr) -> PyStatement {
    PyStatement::AugAssign {
        target: Box::new(target),
        op,
        value: Box::new(value),
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
    }
}

pub fn if_stmt(test: PyExpr, body: Vec<PyStatement>, orelse: Vec<PyStatement>) -> PyStatement {
    PyStatement::If {
        test: Box::new(test),
        body,
        orelse,
    }
}

pub fn while_stmt(test: PyExpr, body: Vec<PyStatement>, orelse: Vec<PyStatement>) -> PyStatement {
    PyStatement::While {
        test: Box::new(test),
        body,
        orelse,
    }
}

pub fn for_stmt(target: PyExpr, iter: PyExpr, body: Vec<PyStatement>) -> PyStatement {
    PyStatement::For {
        target: Box::new(target),
        iter: Box::new(iter),
        body,
        orelse: vec![],
        type_comment: None,
    }
}

pub fn return_stmt(value: Option<PyExpr>) -> PyStatement {
    PyStatement::Return {
        value: value.map(Box::new),
    }
}

pub fn import(names: Vec<PyAlias>) -> PyStatement {
    PyStatement::Import { names }
}

/// `from <level dots><module> import <names>`. `level` is `None` for an
/// absolute import.
pub fn import_from(module: Option<&str>, names: Vec<PyAlias>, level: Option<i32>) -> PyStatement {
    PyStatement::ImportFrom {
        module: module.map(str::to_string),
        names,
        level,
    }
}

pub fn alias(name: &str, asname: Option<&str>) -> PyAlias {
    PyAlias {
        name: name.to_string(),
        asname: asname.map(str::to_string),
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
    }
}

pub fn class_def(name: &str, body: Vec<PyStatement>) -> PyStatement {
    PyStatement::ClassDef {
        name: name.to_string(),
        bases: vec![],
        keywords: vec![],
        body,
        decorator_list: vec![],
        type_params: vec![],
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
    }
}
