use super::*;
const F: &str = "parser/collections.py";

#[test]
fn parse_collections_empty_list() {
    assert_eq!(e(F, "empty_list"), "(list )");
}

#[test]
fn parse_collections_list_of_literals() {
    assert_eq!(e(F, "list_of_literals"), "(list 1 2 3)");
}

#[test]
fn parse_collections_list_of_expressions() {
    assert_eq!(e(F, "list_of_expressions"), "(list (+ 1 2) (* 3 4))");
}

#[test]
fn parse_collections_nested_list() {
    assert_eq!(e(F, "nested_list"), "(list (list 1) (list 2))");
}

#[test]
fn parse_collections_list_trailing_comma() {
    assert_eq!(e(F, "list_trailing_comma"), "(list 1 2)");
}

#[test]
fn parse_collections_single_element_list() {
    assert_eq!(e(F, "single_element_list"), "(list 1)");
}

#[test]
fn parse_collections_empty_dict() {
    assert_eq!(e(F, "empty_dict"), "(dict )");
}

#[test]
fn parse_collections_dict_one_entry() {
    assert_eq!(e(F, "dict_one_entry"), "(dict str(a): 1)");
}

#[test]
fn parse_collections_dict_several_entries() {
    assert_eq!(e(F, "dict_several_entries"), "(dict str(a): 1 str(b): 2)");
}

#[test]
fn parse_collections_nested_dict() {
    assert_parses(F, "nested_dict");
}

#[test]
fn parse_collections_dict_trailing_comma() {
    assert_eq!(e(F, "dict_trailing_comma"), "(dict str(a): 1)");
}

#[test]
fn parse_collections_set_literal() {
    assert_eq!(e(F, "set_literal"), "(set 1 2)");
}

#[test]
fn parse_collections_list_comprehension() {
    assert!(e(F, "list_comprehension").starts_with("<ListComp"));
}

#[test]
fn parse_collections_list_comprehension_with_condition() {
    assert_parses(F, "list_comprehension_with_condition");
}

#[test]
fn parse_collections_dict_comprehension() {
    assert_parses(F, "dict_comprehension");
}

#[test]
fn parse_collections_set_comprehension() {
    assert_parses(F, "set_comprehension");
}

#[test]
fn parse_collections_generator_expression() {
    assert_parses(F, "generator_expression");
}

#[test]
fn parse_collections_list_containing_tuple() {
    assert_parses(F, "list_containing_tuple");
}

#[test]
fn parse_collections_tuple_containing_list() {
    assert_parses(F, "tuple_containing_list");
}

#[test]
fn parse_collections_unclosed_list() {
    assert_rejected(F, "unclosed_list");
}

#[test]
fn parse_collections_mismatched_brackets() {
    assert_rejected(F, "mismatched_brackets");
}
