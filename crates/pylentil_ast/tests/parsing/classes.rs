use super::*;
const F: &str = "parser/classes.py";

#[test]
fn parse_classes_no_bases() {
    p_assert_eq!(stmt(F, "no_bases"), class_def("C", vec![pass()]));
}

#[test]
fn parse_classes_empty_parens() {
    p_assert_eq!(stmt(F, "empty_parens"), class_def("C", vec![pass()]));
}

#[test]
fn parse_classes_one_base() {
    p_assert_eq!(
        stmt(F, "one_base"),
        class_def_with("C", vec![class_base(name("Base"))], vec![pass()], vec![])
    );
}

#[test]
fn parse_classes_two_bases() {
    p_assert_eq!(
        stmt(F, "two_bases"),
        class_def_with(
            "C",
            vec![class_base(name("A")), class_base(name("B"))],
            vec![pass()],
            vec![]
        )
    );
}

#[test]
fn parse_classes_trailing_comma() {
    p_assert_eq!(
        stmt(F, "trailing_comma"),
        class_def_with(
            "C",
            vec![class_base(name("A")), class_base(name("B"))],
            vec![pass()],
            vec![]
        )
    );
}

#[test]
fn parse_classes_dotted_base() {
    p_assert_eq!(
        stmt(F, "dotted_base"),
        class_def_with(
            "C",
            vec![class_base(attribute(name("mod"), "Base"))],
            vec![pass()],
            vec![]
        )
    );
}

#[test]
fn parse_classes_subscript_base() {
    p_assert_eq!(
        stmt(F, "subscript_base"),
        class_def_with(
            "C",
            vec![class_base(subscript(name("list"), name("int")))],
            vec![pass()],
            vec![]
        )
    );
}

#[test]
fn parse_classes_expression_base() {
    p_assert_eq!(
        stmt(F, "expression_base"),
        class_def_with(
            "C",
            vec![class_base(bin_op(name("A"), PyBinaryOp::Add, name("B")))],
            vec![pass()],
            vec![]
        )
    );
}

#[test]
fn parse_classes_keyword_base() {
    p_assert_eq!(
        stmt(F, "keyword_base"),
        class_def_with(
            "C",
            vec![
                class_base(name("A")),
                class_keyword(Some("metaclass"), name("M"))
            ],
            vec![pass()],
            vec![]
        )
    );
}

#[test]
fn parse_classes_star_bases() {
    p_assert_eq!(
        stmt(F, "star_bases"),
        class_def_with(
            "C",
            vec![
                class_base(name("A")),
                class_base(starred(name("rest"))),
                class_keyword(None, name("kw"))
            ],
            vec![pass()],
            vec![]
        )
    );
}

#[test]
fn parse_classes_inline_body() {
    p_assert_eq!(stmt(F, "inline_body"), class_def("C", vec![pass()]));
}

#[test]
fn parse_classes_inline_body_two_statements() {
    p_assert_eq!(
        stmt(F, "inline_body_two_statements"),
        class_def("C", vec![expr_stmt(name("a")), expr_stmt(name("b"))])
    );
}

#[test]
fn parse_classes_multi_statement_body() {
    p_assert_eq!(
        stmt(F, "multi_statement_body"),
        class_def("C", vec![expr_stmt(name("a")), expr_stmt(name("b"))])
    );
}

#[test]
fn parse_classes_body_with_assign() {
    p_assert_eq!(
        stmt(F, "body_with_assign"),
        class_def("C", vec![assign(vec![store(name("x"))], int(1))])
    );
}

#[test]
fn parse_classes_method() {
    p_assert_eq!(
        stmt(F, "method"),
        class_def(
            "C",
            vec![function_def(
                "f",
                PyArguments {
                    args: vec![arg("self")],
                    ..Default::default()
                },
                vec![pass()]
            )]
        )
    );
}

#[test]
fn parse_classes_nested_class() {
    p_assert_eq!(
        stmt(F, "nested_class"),
        class_def("C", vec![class_def("D", vec![pass()])])
    );
}

#[test]
fn parse_classes_class_then_statement() {
    p_assert_eq!(
        body(F, "class_then_statement"),
        vec![class_def("C", vec![pass()]), expr_stmt(name("x"))]
    );
}

#[test]
fn parse_classes_class_in_if() {
    p_assert_eq!(
        stmt(F, "class_in_if"),
        if_stmt(name("a"), vec![class_def("C", vec![pass()])], vec![])
    );
}

#[test]
fn parse_classes_decorated() {
    p_assert_eq!(
        stmt(F, "decorated"),
        class_def_with("C", vec![], vec![pass()], vec![name("dec")])
    );
}

#[test]
fn parse_classes_decorated_with_base() {
    p_assert_eq!(
        stmt(F, "decorated_with_base"),
        class_def_with(
            "C",
            vec![class_base(name("Base"))],
            vec![pass()],
            vec![attribute(name("a"), "b"), call(name("c"), vec![], vec![])]
        )
    );
}

#[test]
fn parse_classes_decorated_method() {
    p_assert_eq!(
        stmt(F, "decorated_method"),
        class_def(
            "C",
            vec![PyStatement::FunctionDef {
                name: "f".into(),
                args: Box::new(PyArguments {
                    args: vec![arg("self")],
                    ..Default::default()
                }),
                body: vec![pass()],
                decorator_list: vec![name("dec")],
                returns: None,
                type_comment: None,
                type_params: vec![],
                is_async: false,
                span: PySpan::default(),
            }]
        )
    );
}

#[test]
fn parse_classes_missing_name() {
    assert_rejected(F, "missing_name");
}

#[test]
fn parse_classes_literal_name() {
    assert_rejected(F, "literal_name");
}

#[test]
fn parse_classes_dotted_name() {
    assert_rejected(F, "dotted_name");
}

#[test]
fn parse_classes_missing_colon() {
    assert_rejected(F, "missing_colon");
}

#[test]
fn parse_classes_missing_body() {
    assert_rejected(F, "missing_body");
}

#[test]
fn parse_classes_unclosed_parens() {
    assert_rejected(F, "unclosed_parens");
}

#[test]
fn parse_classes_positional_after_keyword() {
    assert_rejected(F, "positional_after_keyword");
}

#[test]
fn parse_classes_posonly_marker() {
    assert_rejected(F, "posonly_marker");
}

#[test]
fn parse_classes_bare_star() {
    assert_rejected(F, "bare_star");
}
