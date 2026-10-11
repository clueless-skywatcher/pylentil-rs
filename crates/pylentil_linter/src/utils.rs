use pylentil_ast::ast::{PyArgType, PyClassDefStmt, PyExpr, PyNameExpr};

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
