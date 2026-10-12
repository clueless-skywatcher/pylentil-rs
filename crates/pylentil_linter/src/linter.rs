use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use pylentil_ast::{
    ast::{
        PyArgType, PyArguments, PyCompareExpr, PyComprehension, PyExceptHandler, PyExpr, PyModule,
        PyStatement, PyTypeParam, PyWithItem,
    },
    code::PyCode,
};

use crate::{
    lint::Lint,
    rules::{
        pycodestyle::{
            bare_except::BareExcept, none_comparison::NoneComparison,
            true_false_comparison::TrueFalseComparison,
        },
        pylint::{
            bad_dunder_name::BadDunderName, binary_op_exception::BinaryOpException,
            self_assigning_variable::SelfAssigningVariable, unreachable::Unreachable,
            useless_return::UselessReturn, wildcard_import::WildcardImport,
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
        let mut violations = self.walk_statement_exprs(path, statement);
        violations.append(&mut match statement {
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
        });
        violations
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
        let mut violations = vec![];

        if self.rule_enabled(&BadDunderName) {
            violations.append(&mut BadDunderName.check(path, classdef));
        }

        if let PyStatement::ClassDef(classdef_stmt) = classdef {
            violations.append(&mut self.check_statement(path, &classdef_stmt.body));
        }

        violations
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

    fn check_compare(&self, path: &PathBuf, expr: &PyExpr) -> Vec<LintViolation> {
        let mut violations = vec![];

        if !matches!(expr, PyExpr::Compare(_)) {
            return violations;
        }

        if self.rule_enabled(&NoneComparison) {
            violations.extend(NoneComparison.check_expr(path, expr));
        }

        if self.rule_enabled(&TrueFalseComparison) {
            violations.extend(TrueFalseComparison.check_expr(path, expr));
        }

        violations
    }

    fn walk_statement_exprs(&self, path: &PathBuf, statement: &PyStatement) -> Vec<LintViolation> {
        match statement {
            PyStatement::FunctionDef(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.name);
                violations.extend(self.walk_exprs(path, &stmt.decorator_list));
                violations.extend(self.walk_optional(path, &stmt.returns));
                violations.extend(self.walk_arguments(path, &stmt.args));
                violations.extend(self.walk_type_params(path, &stmt.type_params));
                violations
            }
            PyStatement::ClassDef(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.name);
                violations.extend(self.walk_exprs(path, &stmt.decorator_list));
                violations.extend(self.walk_arg_types(path, &stmt.bases));
                for keyword in &stmt.keywords {
                    violations.extend(self.walk_expr(path, &keyword.value));
                }
                violations.extend(self.walk_type_params(path, &stmt.type_params));
                violations
            }
            PyStatement::Return(stmt) => self.walk_optional(path, &stmt.value),
            PyStatement::Delete(stmt) => self.walk_exprs(path, &stmt.targets),
            PyStatement::Assign(stmt) => {
                let mut violations = self.walk_exprs(path, &stmt.targets);
                violations.extend(self.walk_expr(path, &stmt.value));
                violations
            }
            PyStatement::TypeAlias(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.name);
                violations.extend(self.walk_expr(path, &stmt.value));
                violations.extend(self.walk_type_params(path, &stmt.type_params));
                violations
            }
            PyStatement::AugAssign(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.target);
                violations.extend(self.walk_expr(path, &stmt.value));
                violations
            }
            PyStatement::AnnAssign(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.target);
                violations.extend(self.walk_expr(path, &stmt.annotation));
                violations.extend(self.walk_optional(path, &stmt.value));
                violations
            }
            PyStatement::For(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.target);
                violations.extend(self.walk_expr(path, &stmt.iter));
                violations
            }
            PyStatement::AsyncFor(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.target);
                violations.extend(self.walk_expr(path, &stmt.iter));
                violations
            }
            PyStatement::While(stmt) => self.walk_expr(path, &stmt.test),
            PyStatement::If(stmt) => self.walk_expr(path, &stmt.test),
            PyStatement::With(stmt) => self.walk_with_items(path, &stmt.items),
            PyStatement::AsyncWith(stmt) => self.walk_with_items(path, &stmt.items),
            PyStatement::Match(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.subject);
                for case in &stmt.cases {
                    violations.extend(self.walk_optional(path, &case.guard));
                }
                violations
            }
            PyStatement::Raise(stmt) => {
                let mut violations = self.walk_optional(path, &stmt.exc);
                violations.extend(self.walk_optional(path, &stmt.cause));
                violations
            }
            PyStatement::Try(stmt) => self.walk_handlers(path, &stmt.handlers),
            PyStatement::TryStar(stmt) => self.walk_handlers(path, &stmt.handlers),
            PyStatement::Assert(stmt) => {
                let mut violations = self.walk_expr(path, &stmt.test);
                violations.extend(self.walk_optional(path, &stmt.msg));
                violations
            }
            PyStatement::Expr(stmt) => self.walk_expr(path, &stmt.value),
            PyStatement::Import(_)
            | PyStatement::ImportFrom(_)
            | PyStatement::Global(_)
            | PyStatement::Nonlocal(_)
            | PyStatement::Pass(_)
            | PyStatement::Break(_)
            | PyStatement::Continue(_)
            | PyStatement::Block(_) => vec![],
        }
    }

    fn walk_expr(&self, path: &PathBuf, expr: &PyExpr) -> Vec<LintViolation> {
        let mut violations = match expr {
            PyExpr::Compare(_) => self.check_compare(path, expr),
            PyExpr::Lambda(expr) => self.walk_arguments(path, &expr.args),
            _ => vec![],
        };

        for child in child_exprs(expr) {
            violations.extend(self.walk_expr(path, child));
        }

        violations
    }

    fn walk_exprs(&self, path: &PathBuf, exprs: &[PyExpr]) -> Vec<LintViolation> {
        exprs
            .iter()
            .flat_map(|expr| self.walk_expr(path, expr))
            .collect()
    }

    fn walk_optional(&self, path: &PathBuf, expr: &Option<Box<PyExpr>>) -> Vec<LintViolation> {
        expr.as_ref()
            .map(|expr| self.walk_expr(path, expr))
            .unwrap_or_default()
    }

    fn walk_arguments(&self, path: &PathBuf, args: &PyArguments) -> Vec<LintViolation> {
        let mut violations = vec![];
        let params = args
            .posonlyargs
            .iter()
            .chain(&args.args)
            .chain(&args.kwonlyargs)
            .chain(args.vararg.as_ref())
            .chain(args.kwarg.as_ref());

        for param in params {
            violations.extend(self.walk_expr(path, &param.arg));
            violations.extend(self.walk_optional(path, &param.annotation));
        }

        for default in args.defaults.iter().chain(&args.kw_defaults) {
            if let Some(default) = default {
                violations.extend(self.walk_expr(path, default));
            }
        }

        violations
    }

    fn walk_arg_types(&self, path: &PathBuf, args: &[PyArgType]) -> Vec<LintViolation> {
        let mut violations = vec![];
        for arg in args {
            match arg {
                PyArgType::Arg(arg) => {
                    violations.extend(self.walk_expr(path, &arg.arg));
                    violations.extend(self.walk_optional(path, &arg.annotation));
                }
                PyArgType::Keyword {
                    keyword,
                    annotation,
                    ..
                } => {
                    violations.extend(self.walk_expr(path, &keyword.value));
                    violations.extend(self.walk_optional(path, annotation));
                }
                PyArgType::PosOnlyMarker | PyArgType::KeywordOnlyMarker => {}
            }
        }
        violations
    }

    fn walk_type_params(&self, path: &PathBuf, params: &[PyTypeParam]) -> Vec<LintViolation> {
        let mut violations = vec![];
        for param in params {
            if let PyTypeParam::TypeVar {
                bound: Some(bound), ..
            } = param
            {
                violations.extend(self.walk_expr(path, bound));
            }
        }
        violations
    }

    fn walk_with_items(&self, path: &PathBuf, items: &[PyWithItem]) -> Vec<LintViolation> {
        let mut violations = vec![];
        for item in items {
            violations.extend(self.walk_expr(path, &item.context_expr));
            violations.extend(self.walk_optional(path, &item.optional_vars));
        }
        violations
    }

    fn walk_handlers(&self, path: &PathBuf, handlers: &[PyExceptHandler]) -> Vec<LintViolation> {
        let mut violations = vec![];
        for handler in handlers {
            violations.extend(self.walk_optional(path, &handler.type_));
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

            if let Some(possible_fix) = check_violated.possible_fix() {
                println!("    Possible fix: {}", possible_fix);
            }
        }
    }
}

fn child_exprs(expr: &PyExpr) -> Vec<&PyExpr> {
    match expr {
        PyExpr::BoolOp(expr) => expr.values.iter().collect(),
        PyExpr::NamedExpr(expr) => vec![&expr.target, &expr.value],
        PyExpr::BinOp(expr) => vec![&expr.left, &expr.right],
        PyExpr::UnaryOp(expr) => vec![&expr.operand],
        PyExpr::Lambda(expr) => vec![&expr.body],
        PyExpr::IfExp(expr) => vec![&expr.test, &expr.body, &expr.orelse],
        PyExpr::Dict(expr) => expr.keys.iter().flatten().chain(&expr.values).collect(),
        PyExpr::Set(expr) => expr.elts.iter().collect(),
        PyExpr::ListComp(expr) => comprehension_exprs(&expr.elt, &expr.generators),
        PyExpr::SetComp(expr) => comprehension_exprs(&expr.elt, &expr.generators),
        PyExpr::DictComp(expr) => {
            let mut exprs = comprehension_exprs(&expr.key, &expr.generators);
            exprs.push(&expr.value);
            exprs
        }
        PyExpr::GeneratorExp(expr) => comprehension_exprs(&expr.elt, &expr.generators),
        PyExpr::Await(expr) => vec![&expr.value],
        PyExpr::Yield(expr) => expr.value.as_deref().into_iter().collect(),
        PyExpr::YieldFrom(expr) => vec![&expr.value],
        PyExpr::Compare(expr) => std::iter::once(expr.left.as_ref())
            .chain(&expr.comparators)
            .collect(),
        PyExpr::Call(expr) => std::iter::once(expr.func.as_ref())
            .chain(expr.args.iter().map(|arg| arg.as_ref()))
            .chain(expr.keywords.iter().map(|keyword| keyword.value.as_ref()))
            .collect(),
        PyExpr::FormattedValue(expr) => std::iter::once(expr.value.as_ref())
            .chain(expr.format_spec.as_deref())
            .collect(),
        PyExpr::JoinedStr(expr) => expr.values.iter().collect(),
        PyExpr::Attribute(expr) => vec![&expr.value],
        PyExpr::Subscript(expr) => vec![&expr.value, &expr.slice],
        PyExpr::Starred(expr) => vec![&expr.value],
        PyExpr::List(expr) => expr.elts.iter().collect(),
        PyExpr::Tuple(expr) => expr.elts.iter().collect(),
        PyExpr::Slice(expr) => expr
            .lower
            .as_deref()
            .into_iter()
            .chain(expr.upper.as_deref())
            .chain(expr.step.as_deref())
            .collect(),
        PyExpr::Constant(_) | PyExpr::Name(_) => vec![],
    }
}

fn comprehension_exprs<'a>(elt: &'a PyExpr, generators: &'a [PyComprehension]) -> Vec<&'a PyExpr> {
    let mut exprs = vec![elt];
    for generator in generators {
        exprs.push(&generator.target);
        exprs.push(&generator.iter);
        exprs.extend(&generator.ifs);
    }
    exprs
}
