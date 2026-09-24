use super::*;
const F: &str = "parser/imports.py";

fn s(name: &str) -> String {
    let code = case(F, name);
    match parse_outcome(&code) {
        Outcome::Ok(body) => body.join(" "),
        Outcome::Err(err) => format!("<rejected: {err:?}>"),
        Outcome::Panic(m) => format!("<panic: {m}>"),
    }
}

fn many(name: &str) -> Vec<String> {
    stmts(&case(F, name)).unwrap_or_else(|e| vec![format!("<rejected: {e:?}>")])
}

#[test]
fn a_plain_import() {
    assert_eq!(s("import"), "(import os)");
}

#[test]
fn a_dotted_module_stays_one_name() {
    assert_eq!(s("import_dotted"), "(import os.path)");
    assert_eq!(s("import_deeply_dotted"), "(import a.b.c)");
}

#[test]
fn an_import_with_an_alias() {
    assert_eq!(s("import_as"), "(import numpy:np)");
    assert_eq!(s("import_dotted_as"), "(import os.path:p)");
}

#[test]
fn several_modules_in_one_import() {
    assert_eq!(s("import_several"), "(import os sys)");
    assert_eq!(s("import_several_with_alias"), "(import os numpy:np)");
}

#[test]
fn a_from_import() {
    assert_eq!(s("from_import"), "(from os path)");
    assert_eq!(s("from_dotted_import"), "(from os.path join)");
}

#[test]
fn several_names_in_one_from_import() {
    assert_eq!(s("from_import_several"), "(from os path sep)");
    assert_eq!(s("from_import_several_with_alias"), "(from os path:p sep)");
}

#[test]
fn a_from_import_with_an_alias() {
    assert_eq!(s("from_import_as"), "(from os path:p)");
}

#[test]
fn a_star_import() {
    assert_eq!(s("from_import_star"), "(from os *)");
}

#[test]
fn parenthesized_names_are_the_same_as_bare_ones() {
    assert_eq!(s("from_import_parenthesized"), "(from os path sep)");
    assert_eq!(s("from_import_parenthesized_trailing_comma"), "(from os path sep)");
    assert_eq!(s("from_import_parenthesized_multiline"), "(from os path sep)");
}

#[test]
fn a_relative_import_without_a_module() {
    assert_eq!(s("relative_import"), "(from . sibling)");
    assert_eq!(s("relative_import_two_dots"), "(from .. parent)");
}

#[test]
fn three_leading_dots_are_three_levels_not_an_ellipsis() {
    assert_eq!(s("relative_import_three_dots"), "(from ... grandparent)");
    assert_eq!(s("relative_import_deep"), "(from ...pkg.sub thing)");
}

#[test]
fn a_relative_import_with_a_module() {
    assert_eq!(s("relative_import_with_module"), "(from .pkg thing)");
}

#[test]
fn an_import_is_an_ordinary_simple_statement() {
    assert_eq!(s("import_in_block"), "(if a ((import os)) ())");
    assert_eq!(many("import_then_statement"), vec!["(import os)", "(assign (x) 1)"]);
    assert_eq!(many("imports_separated_by_semicolon"), vec!["(import os)", "(import sys)"]);
}

const MALFORMED: &[&str] = &[
    "import_missing_name",
    "import_trailing_comma",
    "import_as_missing_name",
    "import_star_without_from",
    "from_missing_import",
    "from_missing_names",
    "from_import_unparenthesized_trailing_comma",
    "from_import_star_with_alias",
    "from_import_star_mixed_with_names",
    "from_import_unclosed_parenthesis",
];

#[test]
fn malformed_imports_are_rejected() {
    for name in MALFORMED {
        assert_rejected(F, name);
    }
}

#[test]
fn every_well_formed_import_parses() {
    let real: Vec<String> = parse_failures(F)
        .into_iter()
        .filter(|f| {
            let name = f.trim_start().split(':').next().unwrap_or("");
            !MALFORMED.contains(&name)
        })
        .collect();
    assert!(real.is_empty(), "import forms must parse:\n{}", real.join("\n"));
}
