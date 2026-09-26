use super::*;
const F: &str = "parser/calls.py";

#[test]
fn parse_calls_no_arguments() {
    assert_eq!(expr(F, "no_arguments"), call(name("f"), vec![], vec![]));
}

#[test]
fn parse_calls_one_argument() {
    assert_eq!(
        expr(F, "one_argument"),
        call(name("f"), vec![int(1)], vec![])
    );
}

#[test]
fn parse_calls_several_arguments() {
    assert_eq!(
        expr(F, "several_arguments"),
        call(name("f"), vec![int(1), int(2), int(3)], vec![])
    );
}

#[test]
fn parse_calls_expression_argument() {
    assert_eq!(
        expr(F, "expression_argument"),
        call(
            name("f"),
            vec![bin_op(int(1), PyBinaryOp::Add, int(2))],
            vec![]
        )
    );
}

#[test]
fn parse_calls_tuple_argument() {
    assert_eq!(
        expr(F, "tuple_argument"),
        call(
            name("f"),
            vec![parenthesized_tuple(vec![int(1), int(2)])],
            vec![]
        )
    );
}

#[test]
fn parse_calls_keyword_argument() {
    assert_eq!(
        expr(F, "keyword_argument"),
        call(name("f"), vec![], vec![keyword(Some("a"), int(1))])
    );
}

#[test]
fn parse_calls_mixed_arguments() {
    assert_eq!(
        expr(F, "mixed_arguments"),
        call(name("f"), vec![int(1)], vec![keyword(Some("b"), int(2))])
    );
}

#[test]
fn parse_calls_nested_call() {
    assert_eq!(
        expr(F, "nested_call"),
        call(
            name("f"),
            vec![call(name("g"), vec![int(1)], vec![])],
            vec![]
        )
    );
}

#[test]
fn parse_calls_call_result_called() {
    assert_eq!(
        expr(F, "call_result_called"),
        call(call(name("f"), vec![], vec![]), vec![], vec![])
    );
}

#[test]
fn parse_calls_attribute() {
    assert_eq!(expr(F, "attribute"), attribute(name("a"), "b"));
}

#[test]
fn parse_calls_chained_attribute() {
    assert_eq!(
        expr(F, "chained_attribute"),
        attribute(attribute(name("a"), "b"), "c")
    );
}

#[test]
fn parse_calls_method_call() {
    assert_eq!(
        expr(F, "method_call"),
        call(attribute(name("a"), "b"), vec![], vec![])
    );
}

#[test]
fn parse_calls_call_then_attribute() {
    assert_eq!(
        expr(F, "call_then_attribute"),
        attribute(call(name("f"), vec![], vec![]), "g")
    );
}

#[test]
fn parse_calls_attribute_on_call_result_called() {
    assert_eq!(
        expr(F, "attribute_on_call_result_called"),
        call(
            attribute(call(name("f"), vec![], vec![]), "g"),
            vec![],
            vec![]
        )
    );
}

#[test]
fn parse_calls_star_args() {
    assert_eq!(
        expr(F, "star_args"),
        call(name("f"), vec![starred(name("args"))], vec![])
    );
}

#[test]
fn parse_calls_double_star_kwargs() {
    assert_eq!(
        expr(F, "double_star_kwargs"),
        call(name("f"), vec![], vec![keyword(None, name("kwargs"))])
    );
}

#[test]
fn parse_calls_trailing_comma_in_call() {
    assert_eq!(
        expr(F, "trailing_comma_in_call"),
        call(name("f"), vec![int(1)], vec![])
    );
}

#[test]
fn parse_calls_keywords_before_positional_arguments() {
    assert_rejected(F, "keywords_before_positional_arguments");
}

#[test]
fn parse_calls_unclosed_call() {
    assert_rejected(F, "unclosed_call");
}

#[test]
fn parse_calls_attribute_without_name() {
    assert_rejected(F, "attribute_without_name");
}
