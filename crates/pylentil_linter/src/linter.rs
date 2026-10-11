use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use pylentil_ast::{
    ast::{PyExceptHandler, PyModule, PyStatement},
    code::PyCode,
};

use crate::{
    lint::Lint,
    rules::{
        pycodestyle::bare_except::BareExcept,
        pylint::{
            binary_op_exception::BinaryOpException, self_assigning_variable::SelfAssigningVariable,
            unreachable::Unreachable, useless_return::UselessReturn,
            wildcard_import::WildcardImport,
        },
    },
    violation::LintViolation,
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
        self.check_statement(path, &ast.body)
    }

    fn rule_enabled(&self, lint: &(dyn Lint + 'static)) -> bool {
        self.lints.contains(lint)
    }

    fn check_statement(&self, path: &PathBuf, statement: &PyStatement) -> Vec<LintViolation> {
        match statement {
            PyStatement::FunctionDef(_) => self.check_funcdef(path, statement),
            PyStatement::ClassDef(_) => self.check_classdef(path, statement),
            PyStatement::Return(_) => self.check_return(path, statement),
            PyStatement::Delete(_) => self.check_delete(path, statement),
            PyStatement::Assign(_) => self.check_assign(path, statement),
            PyStatement::TypeAlias(_) => self.check_type_alias(path, statement),
            PyStatement::AugAssign(_) => self.check_aug_assign(path, statement),
            PyStatement::AnnAssign(_) => self.check_ann_assign(path, statement),
            PyStatement::For(_) => self.check_for(path, statement),
            PyStatement::AsyncFor(_) => self.check_async_for(path, statement),
            PyStatement::While(_) => self.check_while(path, statement),
            PyStatement::If(_) => self.check_if(path, statement),
            PyStatement::With(_) => self.check_with(path, statement),
            PyStatement::AsyncWith(_) => self.check_async_with(path, statement),
            PyStatement::Match(_) => self.check_match(path, statement),
            PyStatement::Raise(_) => self.check_raise(path, statement),
            PyStatement::Try(_) => self.check_try(path, statement),
            PyStatement::TryStar(_) => self.check_try_star(path, statement),
            PyStatement::Assert(_) => self.check_assert(path, statement),
            PyStatement::Import(_) => self.check_import(path, statement),
            PyStatement::ImportFrom(_) => self.check_import_from(path, statement),
            PyStatement::Global(_) => self.check_global(path, statement),
            PyStatement::Nonlocal(_) => self.check_nonlocal(path, statement),
            PyStatement::Expr(_) => self.check_expr(path, statement),
            PyStatement::Pass(_) => self.check_pass(path, statement),
            PyStatement::Break(_) => self.check_break(path, statement),
            PyStatement::Continue(_) => self.check_continue(path, statement),
            PyStatement::Block(_) => self.check_block(path, statement),
        }
    }

    fn check_block(&self, path: &PathBuf, statement: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];
        let PyStatement::Block(block) = statement else {
            return violations;
        };

        if self.rule_enabled(&Unreachable) {
            violations.append(&mut Unreachable.check(path, statement));
        }

        for statement in &block.body {
            violations.append(&mut self.check_statement(path, statement));
        }

        violations
    }

    fn check_funcdef(&self, path: &PathBuf, funcdef: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&UselessReturn) {
            violations.append(&mut UselessReturn.check(path, funcdef));
        }

        if let PyStatement::FunctionDef(funcdef) = funcdef {
            violations.append(&mut self.check_statement(path, &funcdef.body));
        }

        violations
    }

    fn check_classdef(&self, path: &PathBuf, classdef: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::ClassDef(classdef) = classdef else {
            return vec![];
        };
        self.check_statement(path, &classdef.body)
    }

    fn check_return(&self, _path: &PathBuf, _return_stmt: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_delete(&self, _path: &PathBuf, _delete: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_assign(&self, path: &PathBuf, assign: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&SelfAssigningVariable) {
            violations.append(&mut SelfAssigningVariable.check(path, assign));
        }

        violations
    }

    fn check_type_alias(&self, _path: &PathBuf, _type_alias: &PyStatement) -> Vec<LintViolation> {
        vec![]
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

    fn check_for(&self, path: &PathBuf, for_stmt: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::For(for_stmt) = for_stmt else {
            return vec![];
        };
        self.check_body_and_orelse(path, &for_stmt.body, &for_stmt.orelse)
    }

    fn check_async_for(&self, path: &PathBuf, for_stmt: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::AsyncFor(for_stmt) = for_stmt else {
            return vec![];
        };
        self.check_body_and_orelse(path, &for_stmt.body, &for_stmt.orelse)
    }

    fn check_while(&self, path: &PathBuf, while_stmt: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::While(while_stmt) = while_stmt else {
            return vec![];
        };
        self.check_body_and_orelse(path, &while_stmt.body, &while_stmt.orelse)
    }

    fn check_if(&self, path: &PathBuf, if_stmt: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::If(if_stmt) = if_stmt else {
            return vec![];
        };
        self.check_body_and_orelse(path, &if_stmt.body, &if_stmt.orelse)
    }

    fn check_with(&self, path: &PathBuf, with_stmt: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::With(with_stmt) = with_stmt else {
            return vec![];
        };
        self.check_statement(path, &with_stmt.body)
    }

    fn check_async_with(&self, path: &PathBuf, with_stmt: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::AsyncWith(with_stmt) = with_stmt else {
            return vec![];
        };
        self.check_statement(path, &with_stmt.body)
    }

    fn check_match(&self, path: &PathBuf, match_stmt: &PyStatement) -> Vec<LintViolation> {
        let PyStatement::Match(match_stmt) = match_stmt else {
            return vec![];
        };
        let mut violations = vec![];
        for case in &match_stmt.cases {
            violations.append(&mut self.check_statement(path, &case.body));
        }
        violations
    }

    fn check_raise(&self, _path: &PathBuf, _raise: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_try(&self, path: &PathBuf, try_stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&BareExcept) {
            violations.append(&mut BareExcept.check(path, try_stmt));
        }
        if self.rule_enabled(&BinaryOpException) {
            violations.append(&mut BinaryOpException.check(path, try_stmt));
        }

        let PyStatement::Try(try_stmt) = try_stmt else {
            return violations;
        };
        violations.append(&mut self.check_try_suites(
            path,
            &try_stmt.body,
            &try_stmt.handlers,
            &try_stmt.orelse,
            &try_stmt.finalbody,
        ));
        violations
    }

    fn check_try_star(&self, path: &PathBuf, try_stmt: &PyStatement) -> Vec<LintViolation> {
        let mut violations = vec![];

        if self.rule_enabled(&BareExcept) {
            violations.append(&mut BareExcept.check(path, try_stmt));
        }
        if self.rule_enabled(&BinaryOpException) {
            violations.append(&mut BinaryOpException.check(path, try_stmt));
        }

        let PyStatement::TryStar(try_stmt) = try_stmt else {
            return violations;
        };
        violations.append(&mut self.check_try_suites(
            path,
            &try_stmt.body,
            &try_stmt.handlers,
            &try_stmt.orelse,
            &try_stmt.finalbody,
        ));
        violations
    }

    fn check_assert(&self, _path: &PathBuf, _assert: &PyStatement) -> Vec<LintViolation> {
        vec![]
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

    fn check_global(&self, _path: &PathBuf, _global: &PyStatement) -> Vec<LintViolation> {
        vec![]
    }

    fn check_nonlocal(&self, _path: &PathBuf, _nonlocal: &PyStatement) -> Vec<LintViolation> {
        vec![]
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

    fn check_body_and_orelse(
        &self,
        path: &PathBuf,
        body: &PyStatement,
        orelse: &Option<PyStatement>,
    ) -> Vec<LintViolation> {
        let mut violations = self.check_statement(path, body);
        if let Some(orelse) = orelse {
            violations.append(&mut self.check_statement(path, orelse));
        }
        violations
    }

    fn check_try_suites(
        &self,
        path: &PathBuf,
        body: &PyStatement,
        handlers: &[PyExceptHandler],
        orelse: &Option<PyStatement>,
        finalbody: &Option<PyStatement>,
    ) -> Vec<LintViolation> {
        let mut violations = self.check_statement(path, body);
        for handler in handlers {
            violations.append(&mut self.check_statement(path, &handler.body));
        }
        if let Some(orelse) = orelse {
            violations.append(&mut self.check_statement(path, orelse));
        }
        if let Some(finalbody) = finalbody {
            violations.append(&mut self.check_statement(path, finalbody));
        }
        violations
    }

    pub fn get_violation_report(&self, code: &PyCode) {
        println!("--------------------------------------------------------------------");
        println!("{:^68}", code.metadata.path.display());
        println!("--------------------------------------------------------------------");
        let violations = self.check(code);
        for LintViolation {
            path,
            check_violated,
            span,
        } in violations
        {
            let (line, col) = code.get_line_and_byte_offset(span.start);
            println!(
                "{}(line {}:{}) - {}: {}",
                path.to_str().unwrap(),
                line,
                col,
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
