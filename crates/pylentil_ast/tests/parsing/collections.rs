use super::*;
const F: &str = "parser/collections.py";

#[test]
fn lists() {
    assert_eq!(e(F, "empty_list"), "(list )");
    assert_eq!(e(F, "single_element_list"), "(list 1)");
    assert_eq!(e(F, "list_of_literals"), "(list 1 2 3)");
    assert_eq!(e(F, "list_of_expressions"), "(list (+ 1 2) (* 3 4))");
    assert_eq!(e(F, "nested_list"), "(list (list 1) (list 2))");
    assert_eq!(e(F, "list_trailing_comma"), "(list 1 2)");
}

#[test]
fn dicts() {
    assert_eq!(e(F, "empty_dict"), "(dict )");
    assert_eq!(e(F, "dict_one_entry"), "(dict str(a): 1)");
    assert_eq!(e(F, "dict_several_entries"), "(dict str(a): 1 str(b): 2)");
    assert_eq!(e(F, "dict_trailing_comma"), "(dict str(a): 1)");
}

#[test]
fn sets_and_comprehensions_are_distinguishable() {
    assert_eq!(e(F, "set_literal"), "(set 1 2)");
    assert!(e(F, "list_comprehension").starts_with("<ListComp"));
}

#[test]
fn every_collection_form_parses() {
    let failures = parse_failures(F);
    let real: Vec<&String> = failures
        .iter()
        .filter(|f| !f.contains("unclosed_list") && !f.contains("mismatched_brackets"))
        .collect();
    assert!(real.is_empty(), "collection literals must parse:\n{real:#?}");
}

#[test]
fn malformed_collections_are_rejected() {
    assert_rejected(F, "unclosed_list");
    assert_rejected(F, "mismatched_brackets");
}
