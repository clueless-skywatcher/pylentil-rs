use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use pylentil_ast::{
    ast::{PyModule, PyStatement},
    code::PyCode,
};

use crate::{
    lint::Lint, rules::{pycodestyle::bare_except::BareExcept, pylint::{binary_op_exception::BinaryOpException, self_assigning_variable::SelfAssigningVariable, unreachable::Unreachable, useless_return::UselessReturn, wildcard_import::WildcardImport}}, violation::LintViolation,
};

pub struct Pylentil {
    pub lints: HashSet<Arc<dyn Lint>>,
    pub paths: Vec<File>,
}

impl Pylentil {
    pub fn check(&self, code: &PyCode) -> Vec<LintViolation> {
        let mut violations = vec![];
        violations.append(&mut self.check_ast(&code.metadata.path, &code.ast));
        violations
    }

    fn check_ast(&self, path: &PathBuf, ast: &PyModule) -> Vec<LintViolation> {
        let mut violations = vec![];

        for statement in ast.body.iter() {
            violations.append(&mut self.check_statement(path, statement));
        }

        violations
    }

    fn rule_enabled(&self, lint: &(dyn Lint + 'static)) -> bool {
        self.lints.contains(lint)
    }

    fn check_statement(&self, path: &PathBuf, statement: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        match statement {
            PyStatement::FunctionDef { .. } => {
                violations.append(&mut self.check_funcdef(path, statement));
            }
            PyStatement::ClassDef { .. } => {}
            PyStatement::Return { .. } => {
                violations.append(&mut self.check_return(path, statement));
            }
            PyStatement::Delete { .. } => {}
            PyStatement::Assign { .. } => {
                violations.append(&mut self.check_assign(path, statement));
            }
            PyStatement::TypeAlias { .. } => {}
            PyStatement::AugAssign { .. } => {
                violations.append(&mut self.check_aug_assign(path, statement));
            }
            PyStatement::AnnAssign { .. } => {
                violations.append(&mut self.check_ann_assign(path, statement));
            }
            PyStatement::For { .. } => {}
            PyStatement::AsyncFor { .. } => {}
            PyStatement::While { .. } => {}
            PyStatement::If { .. } => {
                violations.append(&mut self.check_if(path, statement));
            }
            PyStatement::With { .. } => {}
            PyStatement::AsyncWith { .. } => {}
            PyStatement::Match { .. } => {}
            PyStatement::Raise { .. } => {}
            PyStatement::Try { .. } => {
                violations.append(&mut self.check_try(path, statement));
            }
            PyStatement::TryStar { .. } => {}
            PyStatement::Assert { .. } => {}
            PyStatement::Import { .. } => {
                violations.append(&mut self.check_import(path, statement));
            }
            PyStatement::ImportFrom { .. } => {
                violations.append(&mut self.check_import_from(path, statement));
            }
            PyStatement::Global { .. } => {}
            PyStatement::Nonlocal { .. } => {}
            PyStatement::Expr { .. } => {
                violations.append(&mut self.check_expr(path, statement));
            }
            PyStatement::Pass { .. } => {
                violations.append(&mut self.check_pass(path, statement));
            }
            PyStatement::Break { .. } => {
                violations.append(&mut self.check_break(path, statement));
            }
            PyStatement::Continue { .. } => {
                violations.append(&mut self.check_continue(path, statement));
            }
        }

        violations
    }

    fn check_body(&self, path: &PathBuf, body: &[PyStatement]) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&Unreachable) {
            violations.append(&mut Unreachable.check_block(path, body));
        }

        for statement in body {
            violations.append(&mut self.check_statement(path, statement));
        }

        violations
    }

    fn check_funcdef(&self, path: &PathBuf, funcdef: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&UselessReturn) {
            violations.append(&mut UselessReturn.check(path, &funcdef));
        }

        if let PyStatement::FunctionDef { body, .. } = &funcdef {
            violations.append(&mut self.check_body(path, body));
        }

        violations
    }

    fn check_return(&self, _path: &PathBuf, _return_stmt: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_assign(&self, path: &PathBuf, assign: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&SelfAssigningVariable) {
            violations.append(&mut SelfAssigningVariable.check(path, assign));
        }
        
        violations
    }

    fn check_aug_assign(&self, _path: &PathBuf, _aug_assign: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_ann_assign(&self, path: &PathBuf, ann_assign: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&SelfAssigningVariable) {
            violations.append(&mut SelfAssigningVariable.check(path, ann_assign));
        }

        violations
    }

    fn check_try(&self, path: &PathBuf, try_stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&BareExcept) {
            violations.append(&mut BareExcept.check(path, &try_stmt));
        }
        if self.rule_enabled(&BinaryOpException) {
            violations.append(&mut BinaryOpException.check(path, &try_stmt));
        }

        if let PyStatement::Try {
            body,
            handlers,
            orelse,
            finalbody,
            ..
        } = &try_stmt
        {
            violations.append(&mut self.check_body(path, body));
            for handler in handlers {
                violations.append(&mut self.check_body(path, &handler.body));
            }
            violations.append(&mut self.check_body(path, orelse));
            violations.append(&mut self.check_body(path, finalbody));
        }

        violations
    }

    fn check_if(&self, path: &PathBuf, if_stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        if let PyStatement::If { body, orelse, .. } = &if_stmt {
            violations.append(&mut self.check_body(path, body));
            violations.append(&mut self.check_body(path, orelse));
        }

        violations
    }

    fn check_import(&self, _path: &PathBuf, _import: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_import_from(&self, path: &PathBuf, import_from: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&WildcardImport) {
            violations.append(&mut WildcardImport.check(path, import_from));
        }

        violations
    }

    fn check_expr(&self, _path: &PathBuf, _expr: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_pass(&self, _path: &PathBuf, _pass: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_break(&self, _path: &PathBuf, _break_stmt: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_continue(&self, _path: &PathBuf, _continue_stmt: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    pub fn get_violation_report(&self, code: &PyCode) {
        println!("--------------------------------------------------------------------");
        println!("{:^68}", code.metadata.path.display());
        println!("--------------------------------------------------------------------");
        let violations = self.check(code);
        for LintViolation {
            path,
            check_violated,
            span
        } in violations
        {
            let (line, col) = code.get_line_and_byte_offset(span.start);
            println!(
                "{}(line {}:{}) - {}: {}",
                path.to_str().unwrap(),
                line, col,
                check_violated.code(),
                check_violated.message()
            );
            println!(
                "    Possible fix: {}",
                check_violated.possible_fix().unwrap_or("None".to_string())
            );
        }
    }
}
