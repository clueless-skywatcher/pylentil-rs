use super::*;
const F: &str = "parser/calls.py";

#[test]
fn calls() {
    assert_eq!(e(F, "no_arguments"), "(call f)");
    assert_eq!(e(F, "one_argument"), "(call f 1)");
    assert_eq!(e(F, "several_arguments"), "(call f 1 2 3)");
    assert_eq!(e(F, "expression_argument"), "(call f (+ 1 2))");
    assert_eq!(e(F, "nested_call"), "(call f (call g 1))");
}

#[test]
fn a_tuple_argument_is_one_argument() {
    assert_eq!(e(F, "tuple_argument"), "(call f (ptuple 1 2))");
}

#[test]
fn keyword_arguments() {
    assert_eq!(e(F, "keyword_argument"), "(call f a=1)");
    assert_eq!(e(F, "mixed_arguments"), "(call f 1 b=2)");
}

#[test]
fn attribute_access_chains_left_to_right() {
    assert_eq!(e(F, "attribute"), "(attr a b)");
    assert_eq!(e(F, "chained_attribute"), "(attr (attr a b) c)");
}

#[test]
fn calls_and_attributes_compose() {
    assert_eq!(e(F, "method_call"), "(call (attr a b))");
    assert_eq!(e(F, "call_then_attribute"), "(attr (call f) g)");
    assert_eq!(e(F, "call_result_called"), "(call (call f))");
    assert_eq!(e(F, "attribute_on_call_result_called"), "(call (attr (call f) g))");
}

#[test]
fn argument_unpacking() {
    assert_eq!(e(F, "star_args"), "(call f (star args))");
    assert_eq!(e(F, "double_star_kwargs"), "(call f **kwargs)");
}

#[test]
fn a_trailing_comma_in_a_call_is_allowed() {
    assert_eq!(e(F, "trailing_comma_in_call"), "(call f 1)");
}

#[test]
fn malformed_calls_are_rejected() {
    assert_rejected(F, "unclosed_call");
    assert_rejected(F, "attribute_without_name");
    assert_rejected(F, "keywords_before_positional_arguments");
}
