use super::*;
const F: &str = "parser/functions.py";

#[test]
fn parse_functions_no_args() {
    assert_eq!(
        stmt(F, "no_args"),
        function_def("f", PyArguments::default(), vec![PyStatement::Pass])
    );
}

#[test]
fn parse_functions_one_arg() {
    assert_eq!(
        stmt(F, "one_arg"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a")],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_two_args() {
    assert_eq!(
        stmt(F, "two_args"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a"), arg("b")],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_trailing_comma() {
    assert_eq!(
        stmt(F, "trailing_comma"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a"), arg("b")],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_default_arg() {
    assert_eq!(
        stmt(F, "default_arg"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a")],
                defaults: vec![Some(int(1))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_defaults_mixed() {
    assert_eq!(
        stmt(F, "defaults_mixed"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a"), arg("b"), arg("c")],
                defaults: vec![Some(int(1)), Some(int(2))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_default_expression() {
    assert_eq!(
        stmt(F, "default_expression"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a")],
                defaults: vec![Some(bin_op(int(1), PyBinaryOp::Add, int(2)))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_annotated() {
    assert_eq!(
        stmt(F, "annotated"),
        function_def(
            "f",
            PyArguments {
                args: vec![annotated_arg("a", name("int"))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_annotated_default() {
    assert_eq!(
        stmt(F, "annotated_default"),
        function_def(
            "f",
            PyArguments {
                args: vec![annotated_arg("a", name("int"))],
                defaults: vec![Some(int(1))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_annotation_expression() {
    assert_eq!(
        stmt(F, "annotation_expression"),
        function_def(
            "f",
            PyArguments {
                args: vec![annotated_arg("a", subscript(name("list"), name("int")))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_return_annotation() {
    assert_eq!(
        stmt(F, "return_annotation"),
        PyStatement::FunctionDef {
            name: "f".into(),
            args: Box::new(PyArguments::default()),
            body: vec![PyStatement::Pass],
            decorator_list: vec![],
            returns: Some(Box::new(name("int"))),
            type_comment: None,
            type_params: vec![],
            is_async: false
        }
    );
}

#[test]
fn parse_functions_return_annotation_expression() {
    assert_eq!(
        stmt(F, "return_annotation_expression"),
        PyStatement::FunctionDef {
            name: "f".into(),
            args: Box::new(PyArguments::default()),
            body: vec![PyStatement::Pass],
            decorator_list: vec![],
            returns: Some(Box::new(subscript(name("list"), name("int")))),
            type_comment: None,
            type_params: vec![],
            is_async: false
        }
    );
}

#[test]
fn parse_functions_vararg() {
    assert_eq!(
        stmt(F, "vararg"),
        function_def(
            "f",
            PyArguments {
                vararg: Some(vararg("args")),
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_vararg_annotated() {
    assert_eq!(
        stmt(F, "vararg_annotated"),
        function_def(
            "f",
            PyArguments {
                vararg: Some(PyArg {
                    annotation: Some(Box::new(name("int"))),
                    ..vararg("args")
                }),
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_kwarg() {
    assert_eq!(
        stmt(F, "kwarg"),
        function_def(
            "f",
            PyArguments {
                kwarg: Some(arg("kw")),
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_vararg_and_kwarg() {
    assert_eq!(
        stmt(F, "vararg_and_kwarg"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a")],
                vararg: Some(vararg("args")),
                kwarg: Some(arg("kw")),
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_kwonly() {
    assert_eq!(
        stmt(F, "kwonly"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a")],
                kwonlyargs: vec![arg("b")],
                kw_defaults: vec![None],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_kwonly_default() {
    assert_eq!(
        stmt(F, "kwonly_default"),
        function_def(
            "f",
            PyArguments {
                kwonlyargs: vec![arg("b")],
                kw_defaults: vec![Some(int(1))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_kwonly_after_vararg() {
    assert_eq!(
        stmt(F, "kwonly_after_vararg"),
        function_def(
            "f",
            PyArguments {
                vararg: Some(vararg("args")),
                kwonlyargs: vec![arg("b"), arg("c")],
                kw_defaults: vec![None, Some(int(2))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_kwonly_then_kwarg() {
    assert_eq!(
        stmt(F, "kwonly_then_kwarg"),
        function_def(
            "f",
            PyArguments {
                kwonlyargs: vec![arg("b")],
                kw_defaults: vec![None],
                kwarg: Some(arg("kw")),
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_posonly() {
    assert_eq!(
        stmt(F, "posonly"),
        function_def(
            "f",
            PyArguments {
                posonlyargs: vec![arg("a"), arg("b")],
                args: vec![arg("c")],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_posonly_only() {
    assert_eq!(
        stmt(F, "posonly_only"),
        function_def(
            "f",
            PyArguments {
                posonlyargs: vec![arg("a")],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_posonly_with_defaults() {
    assert_eq!(
        stmt(F, "posonly_with_defaults"),
        function_def(
            "f",
            PyArguments {
                posonlyargs: vec![arg("a"), arg("b")],
                args: vec![arg("c")],
                defaults: vec![Some(int(1)), Some(int(2))],
                ..Default::default()
            },
            vec![PyStatement::Pass]
        )
    );
}

#[test]
fn parse_functions_everything() {
    assert_eq!(
        stmt(F, "everything"),
        PyStatement::FunctionDef {
            name: "f".into(),
            args: Box::new(PyArguments {
                posonlyargs: vec![arg("a")],
                args: vec![arg("b"), arg("c")],
                vararg: Some(vararg("rest")),
                kwonlyargs: vec![arg("d"), arg("e")],
                kw_defaults: vec![None, Some(int(2))],
                kwarg: Some(arg("kw")),
                defaults: vec![Some(int(1))],
                ..Default::default()
            }),
            body: vec![return_stmt(Some(name("a")))],
            decorator_list: vec![],
            returns: Some(Box::new(name("int"))),
            type_comment: None,
            type_params: vec![],
            is_async: false
        }
    );
}

#[test]
fn parse_functions_inline_body() {
    assert_parses(F, "inline_body");
}

#[test]
fn parse_functions_inline_body_two_statements() {
    assert_parses(F, "inline_body_two_statements");
}

#[test]
fn parse_functions_multi_statement_body() {
    assert_parses(F, "multi_statement_body");
}

#[test]
fn parse_functions_body_with_return() {
    // assert_eq!(stmt(F, "inline_body"), function_def("f", PyArguments::default(), vec![PyStatement::Pass]));
    // assert_eq!(stmt(F, "inline_body_two_statements"), function_def("f", PyArguments::default(), vec![expr_stmt(name("a")), expr_stmt(name("b"))]));
    // assert_eq!(stmt(F, "multi_statement_body"), function_def("f", PyArguments::default(), vec![expr_stmt(name("a")), expr_stmt(name("b"))]));
    assert_eq!(
        stmt(F, "body_with_return"),
        function_def(
            "f",
            PyArguments {
                args: vec![arg("a")],
                ..Default::default()
            },
            vec![return_stmt(Some(name("a")))]
        )
    );
}

#[test]
fn parse_functions_docstring_body() {
    assert_parses(F, "docstring_body");
}

#[test]
fn parse_functions_nested_def() {
    assert_eq!(
        stmt(F, "nested_def"),
        function_def(
            "f",
            PyArguments::default(),
            vec![function_def(
                "g",
                PyArguments::default(),
                vec![PyStatement::Pass]
            )]
        )
    );
}

#[test]
fn parse_functions_def_then_statement() {
    assert_eq!(
        body(F, "def_then_statement"),
        vec![
            function_def("f", PyArguments::default(), vec![PyStatement::Pass]),
            expr_stmt(name("x"))
        ]
    );
}

#[test]
fn parse_functions_def_in_if() {
    assert_eq!(
        stmt(F, "def_in_if"),
        if_stmt(
            name("a"),
            vec![function_def(
                "f",
                PyArguments::default(),
                vec![PyStatement::Pass]
            )],
            vec![]
        )
    );
}

#[test]
fn parse_functions_decorated() {
    assert_eq!(
        stmt(F, "decorated"),
        PyStatement::FunctionDef {
            name: "f".into(),
            args: Box::new(PyArguments::default()),
            body: vec![PyStatement::Pass],
            decorator_list: vec![name("dec")],
            returns: None,
            type_comment: None,
            type_params: vec![],
            is_async: false
        }
    );
}

#[test]
fn parse_functions_two_decorators() {
    assert_eq!(
        stmt(F, "two_decorators"),
        PyStatement::FunctionDef {
            name: "f".into(),
            args: Box::new(PyArguments::default()),
            body: vec![PyStatement::Pass],
            decorator_list: vec![name("a"), name("b")],
            returns: None,
            type_comment: None,
            type_params: vec![],
            is_async: false
        }
    );
}

#[test]
fn parse_functions_decorator_call() {
    assert_eq!(
        stmt(F, "decorator_call"),
        PyStatement::FunctionDef {
            name: "f".into(),
            args: Box::new(PyArguments::default()),
            body: vec![PyStatement::Pass],
            decorator_list: vec![call(
                attribute(name("app"), "route"),
                vec![string("/")],
                vec![]
            )],
            returns: None,
            type_comment: None,
            type_params: vec![],
            is_async: false
        }
    );
}

#[test]
fn parse_functions_async_def() {
    assert_eq!(
        stmt(F, "async_def"),
        PyStatement::FunctionDef {
            name: "f".into(),
            args: Box::new(PyArguments::default()),
            body: vec![PyStatement::Pass],
            decorator_list: vec![],
            returns: None,
            type_comment: None,
            type_params: vec![],
            is_async: true
        }
    );
}

#[test]
fn parse_functions_missing_name() {
    assert_rejected(F, "missing_name");
}

#[test]
fn parse_functions_missing_parens() {
    assert_rejected(F, "missing_parens");
}

#[test]
fn parse_functions_missing_colon() {
    assert_rejected(F, "missing_colon");
}

#[test]
fn parse_functions_missing_body() {
    assert_rejected(F, "missing_body");
}

#[test]
fn parse_functions_positional_after_default() {
    assert_rejected(F, "positional_after_default");
}

#[test]
fn parse_functions_duplicate_vararg() {
    assert_rejected(F, "duplicate_vararg");
}

#[test]
fn parse_functions_kwarg_not_last() {
    assert_rejected(F, "kwarg_not_last");
}

#[test]
fn parse_functions_two_posonly_markers() {
    assert_rejected(F, "two_posonly_markers");
}

#[test]
fn parse_functions_posonly_marker_first() {
    assert_rejected(F, "posonly_marker_first");
}

#[test]
fn parse_functions_bare_star_last() {
    assert_rejected(F, "bare_star_last");
}

#[test]
fn parse_functions_literal_parameter() {
    assert_rejected(F, "literal_parameter");
}

#[test]
fn parse_functions_duplicate_parameter() {
    assert_rejected(F, "duplicate_parameter");
}

#[test]
fn parse_functions_unclosed_parens() {
    assert_rejected(F, "unclosed_parens");
}
