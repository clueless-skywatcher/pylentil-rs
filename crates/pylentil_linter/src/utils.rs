use pylentil_ast::ast::{
    PyArgType, PyClassDefStmt, PyConstant, PyConstantExpr, PyExpr, PyNameExpr, PyStatement,
};

pub fn classdef_bases(classdef: PyClassDefStmt) -> Vec<String> {
    let PyClassDefStmt { bases, .. } = classdef;

    bases
        .iter()
        .map(|arg_type| match arg_type.clone() {
            PyArgType::Arg(arg) => match *arg.arg {
                PyExpr::Name(PyNameExpr { id, .. }) => id,
                _ => "".to_string(),
            },
            _ => "".to_string(),
        })
        .filter(|str| str.len() > 0)
        .collect()
}

pub fn is_terminator(stmt: &PyStatement) -> bool {
    matches!(
        stmt,
        PyStatement::Raise(_)
            | PyStatement::Return(_)
            | PyStatement::Continue(_)
            | PyStatement::Break(_)
    )
}

pub fn is_none(expr: &PyExpr) -> bool {
    matches!(
        expr,
        PyExpr::Constant(PyConstantExpr {
            value: PyConstant::None,
            ..
        })
    )
}

pub fn is_true(expr: &PyExpr) -> bool {
    matches!(
        expr,
        PyExpr::Constant(PyConstantExpr {
            value: PyConstant::Boolean(true),
            ..
        })
    )
}

pub fn is_false(expr: &PyExpr) -> bool {
    matches!(
        expr,
        PyExpr::Constant(PyConstantExpr {
            value: PyConstant::Boolean(false),
            ..
        })
    )
}
