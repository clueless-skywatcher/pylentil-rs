use super::*;
const F: &str = "parser/calls.py";

#[test]
fn parse_calls_no_arguments() {
    assert_eq!(e(F, "no_arguments"), "(call f)");
}

#[test]
fn parse_calls_one_argument() {
    assert_eq!(e(F, "one_argument"), "(call f 1)");
}

#[test]
fn parse_calls_several_arguments() {
    assert_eq!(e(F, "several_arguments"), "(call f 1 2 3)");
}

#[test]
fn parse_calls_expression_argument() {
    assert_eq!(e(F, "expression_argument"), "(call f (+ 1 2))");
}

#[test]
fn parse_calls_tuple_argument() {
    assert_eq!(e(F, "tuple_argument"), "(call f (ptuple 1 2))");
}

#[test]
fn parse_calls_keyword_argument() {
    assert_eq!(e(F, "keyword_argument"), "(call f a=1)");
}

#[test]
fn parse_calls_mixed_arguments() {
    assert_eq!(e(F, "mixed_arguments"), "(call f 1 b=2)");
}

#[test]
fn parse_calls_nested_call() {
    assert_eq!(e(F, "nested_call"), "(call f (call g 1))");
}

#[test]
fn parse_calls_call_result_called() {
    assert_eq!(e(F, "call_result_called"), "(call (call f))");
}

#[test]
fn parse_calls_attribute() {
    assert_eq!(e(F, "attribute"), "(attr a b)");
}

#[test]
fn parse_calls_chained_attribute() {
    assert_eq!(e(F, "chained_attribute"), "(attr (attr a b) c)");
}

#[test]
fn parse_calls_method_call() {
    assert_eq!(e(F, "method_call"), "(call (attr a b))");
}

#[test]
fn parse_calls_call_then_attribute() {
    assert_eq!(e(F, "call_then_attribute"), "(attr (call f) g)");
}

#[test]
fn parse_calls_attribute_on_call_result_called() {
    assert_eq!(e(F, "attribute_on_call_result_called"), "(call (attr (call f) g))");
}

#[test]
fn parse_calls_star_args() {
    assert_eq!(e(F, "star_args"), "(call f (star args))");
}

#[test]
fn parse_calls_double_star_kwargs() {
    assert_eq!(e(F, "double_star_kwargs"), "(call f **kwargs)");
}

#[test]
fn parse_calls_trailing_comma_in_call() {
    assert_eq!(e(F, "trailing_comma_in_call"), "(call f 1)");
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
