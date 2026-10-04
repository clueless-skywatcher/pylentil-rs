use pylentil_ast::ast::PyExceptHandler;

use super::*;
const F: &str = "parser/exceptions.py";

fn handler(type_: PyExpr, as_name: Option<&str>) -> PyExceptHandler {
    except_handler(Some(type_), as_name, vec![expr_stmt(name("b"))])
}

#[test]
fn parse_exceptions_name() {
    p_assert_eq!(
        stmt(F, "name"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(name("E"), None)],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_name_as() {
    p_assert_eq!(
        stmt(F, "name_as"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(name("E"), Some("err"))],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_parenthesized_name() {
    // Parentheses around one name do not make a tuple.
    p_assert_eq!(
        stmt(F, "parenthesized_name"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(name("E"), None)],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_tuple() {
    p_assert_eq!(
        stmt(F, "tuple"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                parenthesized_tuple(vec![name("E"), name("F")]),
                None
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_tuple_as() {
    p_assert_eq!(
        stmt(F, "tuple_as"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                parenthesized_tuple(vec![name("E"), name("F")]),
                Some("err")
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_single_element_tuple() {
    p_assert_eq!(
        stmt(F, "single_element_tuple"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(parenthesized_tuple(vec![name("E")]), None)],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_attribute() {
    p_assert_eq!(
        stmt(F, "attribute"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(attribute(name("os"), "error"), None)],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_nested_attribute() {
    p_assert_eq!(
        stmt(F, "nested_attribute"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                attribute(attribute(name("pkg"), "mod"), "Error"),
                None
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_attribute_as() {
    p_assert_eq!(
        stmt(F, "attribute_as"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(attribute(name("os"), "error"), Some("err"))],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_call() {
    p_assert_eq!(
        stmt(F, "call"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(call(name("errors"), vec![], vec![]), None)],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_call_with_argument() {
    p_assert_eq!(
        stmt(F, "call_with_argument"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(call(name("errors"), vec![name("E")], vec![]), None)],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_subscript() {
    p_assert_eq!(
        stmt(F, "subscript"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(subscript(name("errors"), name("E")), None)],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_subscript_as() {
    p_assert_eq!(
        stmt(F, "subscript_as"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(subscript(name("errors"), name("E")), Some("err"))],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_or_expression() {
    p_assert_eq!(
        stmt(F, "or_expression"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                bool_op(PyBoolOp::Or, vec![name("E"), name("F")]),
                None
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_and_expression() {
    p_assert_eq!(
        stmt(F, "and_expression"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                bool_op(PyBoolOp::And, vec![name("E"), name("F")]),
                None
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_bitwise_or() {
    p_assert_eq!(
        stmt(F, "bitwise_or"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                bin_op(name("E"), PyBinaryOp::BitOr, name("F")),
                None
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_or_expression_as() {
    // `as` belongs to the handler, not to the expression.
    p_assert_eq!(
        stmt(F, "or_expression_as"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                bool_op(PyBoolOp::Or, vec![name("E"), name("F")]),
                Some("err")
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_parenthesized_or() {
    p_assert_eq!(
        stmt(F, "parenthesized_or"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![handler(
                bool_op(PyBoolOp::Or, vec![name("E"), name("F")]),
                None
            )],
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_exceptions_mixed_handlers() {
    p_assert_eq!(
        stmt(F, "mixed_handlers"),
        try_stmt(
            vec![expr_stmt(name("a"))],
            vec![
                except_handler(
                    Some(attribute(name("os"), "error")),
                    Some("err"),
                    vec![expr_stmt(name("b"))]
                ),
                except_handler(
                    Some(parenthesized_tuple(vec![name("E"), name("F")])),
                    None,
                    vec![expr_stmt(name("c"))]
                ),
                except_handler(
                    Some(bool_op(PyBoolOp::Or, vec![name("G"), name("H")])),
                    None,
                    vec![expr_stmt(name("d"))]
                ),
            ],
            vec![],
            vec![]
        )
    );
}
