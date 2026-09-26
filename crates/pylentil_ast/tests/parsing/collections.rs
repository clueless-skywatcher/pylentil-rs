use super::*;
const F: &str = "parser/collections.py";

#[test]
fn parse_collections_empty_list() {
    assert_eq!(expr(F, "empty_list"), list(vec![]));
}

#[test]
fn parse_collections_list_of_literals() {
    assert_eq!(
        expr(F, "list_of_literals"),
        list(vec![int(1), int(2), int(3)])
    );
}

#[test]
fn parse_collections_list_of_expressions() {
    assert_eq!(
        expr(F, "list_of_expressions"),
        list(vec![
            bin_op(int(1), PyBinaryOp::Add, int(2)),
            bin_op(int(3), PyBinaryOp::Mul, int(4))
        ])
    );
}

#[test]
fn parse_collections_nested_list() {
    assert_eq!(
        expr(F, "nested_list"),
        list(vec![list(vec![int(1)]), list(vec![int(2)])])
    );
}

#[test]
fn parse_collections_list_trailing_comma() {
    assert_eq!(expr(F, "list_trailing_comma"), list(vec![int(1), int(2)]));
}

#[test]
fn parse_collections_single_element_list() {
    assert_eq!(expr(F, "single_element_list"), list(vec![int(1)]));
}

#[test]
fn parse_collections_empty_dict() {
    assert_eq!(expr(F, "empty_dict"), dict(vec![]));
}

#[test]
fn parse_collections_dict_one_entry() {
    assert_eq!(expr(F, "dict_one_entry"), dict(vec![(string("a"), int(1))]));
}

#[test]
fn parse_collections_dict_several_entries() {
    assert_eq!(
        expr(F, "dict_several_entries"),
        dict(vec![(string("a"), int(1)), (string("b"), int(2))])
    );
}

#[test]
fn parse_collections_nested_dict() {
    assert_parses(F, "nested_dict");
}

#[test]
fn parse_collections_dict_trailing_comma() {
    assert_eq!(
        expr(F, "dict_trailing_comma"),
        dict(vec![(string("a"), int(1))])
    );
}

#[test]
fn parse_collections_set_literal() {
    assert_eq!(expr(F, "set_literal"), set(vec![int(1), int(2)]));
}

#[test]
fn parse_collections_list_comprehension() {
    assert!(matches!(
        expr(F, "list_comprehension"),
        PyExpr::ListComp { .. }
    ));
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
