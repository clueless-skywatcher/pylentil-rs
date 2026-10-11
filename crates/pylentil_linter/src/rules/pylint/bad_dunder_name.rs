use std::path::Path;

use pylentil_ast::ast::{PyBlockStmt, PyExpr, PyStatement};

use crate::{
    lint::{Lint, LintCategory, LintSeverity, LintSource},
    violation::LintViolation,
};

const VALID_DUNDER_NAMES: &[&str] = &[
    "__abs__",
    "__add__",
    "__aenter__",
    "__aexit__",
    "__aiter__",
    "__and__",
    "__anext__",
    "__await__",
    "__bool__",
    "__bytes__",
    "__call__",
    "__ceil__",
    "__class__",
    "__class_getitem__",
    "__complex__",
    "__contains__",
    "__copy__",
    "__deepcopy__",
    "__del__",
    "__delattr__",
    "__delete__",
    "__delitem__",
    "__dict__",
    "__dir__",
    "__divmod__",
    "__doc__",
    "__enter__",
    "__eq__",
    "__exit__",
    "__float__",
    "__floor__",
    "__floordiv__",
    "__format__",
    "__fspath__",
    "__ge__",
    "__get__",
    "__getattr__",
    "__getattribute__",
    "__getitem__",
    "__getnewargs__",
    "__getnewargs_ex__",
    "__getstate__",
    "__gt__",
    "__hash__",
    "__iadd__",
    "__iand__",
    "__ifloordiv__",
    "__ilshift__",
    "__imatmul__",
    "__imod__",
    "__imul__",
    "__index__",
    "__init__",
    "__init_subclass__",
    "__instancecheck__",
    "__int__",
    "__invert__",
    "__ior__",
    "__ipow__",
    "__irshift__",
    "__isub__",
    "__iter__",
    "__itruediv__",
    "__ixor__",
    "__le__",
    "__len__",
    "__length_hint__",
    "__lshift__",
    "__lt__",
    "__matmul__",
    "__missing__",
    "__mod__",
    "__module__",
    "__mul__",
    "__ne__",
    "__neg__",
    "__new__",
    "__next__",
    "__or__",
    "__pos__",
    "__post_init__",
    "__pow__",
    "__radd__",
    "__rand__",
    "__rdivmod__",
    "__reduce__",
    "__reduce_ex__",
    "__repr__",
    "__reversed__",
    "__rfloordiv__",
    "__rlshift__",
    "__rmatmul__",
    "__rmod__",
    "__rmul__",
    "__ror__",
    "__round__",
    "__rpow__",
    "__rrshift__",
    "__rshift__",
    "__rsub__",
    "__rtruediv__",
    "__rxor__",
    "__set__",
    "__set_name__",
    "__setattr__",
    "__setitem__",
    "__setstate__",
    "__sizeof__",
    "__str__",
    "__sub__",
    "__subclasscheck__",
    "__subclasses__",
    "__subclasshook__",
    "__suppress_context__",
    "__truediv__",
    "__trunc__",
    "__weakref__",
    "__xor__",
    "_add_alias_",
    "_add_value_alias_",
    "_generate_next_value_",
    "_missing_",
    "_numeric_repr_",
];

#[derive(Debug, Clone)]
pub struct BadDunderName;

impl Lint for BadDunderName {
    fn code(&self) -> &'static str {
        "PYL-W3201"
    }

    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }

    fn message(&self) -> String {
        "Dunder name has been misspelled or is unknown".to_string()
    }

    fn severity(&self) -> LintSeverity {
        LintSeverity::Warning
    }

    fn source(&self) -> LintSource {
        LintSource::Pylint
    }

    fn check(&self, path: &Path, stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        match stmt {
            PyStatement::ClassDef(class_def) => match &class_def.body {
                PyStatement::Block(body_block) => {
                    violations.append(&mut self.check_body(&body_block, path))
                }
                _ => {}
            },

            _ => {}
        }

        violations
    }
}

impl BadDunderName {
    fn check_body(&self, body_block: &PyBlockStmt, path: &Path) -> Vec<LintViolation> {
        let mut violations = vec![];
        for stmt in body_block.body.iter() {
            match stmt {
                PyStatement::FunctionDef(func_def) => {
                    let PyExpr::Name(name) = &func_def.name else {
                        continue;
                    };
                    if !self.is_valid_method(name.id.clone()) {
                        self.report(&mut violations, path, &func_def.name.span());
                    }
                }
                _ => {}
            }
        }

        violations
    }

    fn is_valid_method(&self, func_name: String) -> bool {
        if &func_name.as_str().starts_with("_") & &func_name.as_str().ends_with("_") {
            VALID_DUNDER_NAMES.contains(&func_name.as_str())
        } else {
            true
        }
    }
}
