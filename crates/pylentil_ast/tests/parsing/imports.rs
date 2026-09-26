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
fn parse_imports_import() {
    assert_eq!(s("import"), "(import os)");
}

#[test]
fn parse_imports_import_dotted() {
    assert_eq!(s("import_dotted"), "(import os.path)");
}

#[test]
fn parse_imports_import_deeply_dotted() {
    assert_eq!(s("import_deeply_dotted"), "(import a.b.c)");
}

#[test]
fn parse_imports_import_as() {
    assert_eq!(s("import_as"), "(import numpy:np)");
}

#[test]
fn parse_imports_import_dotted_as() {
    assert_eq!(s("import_dotted_as"), "(import os.path:p)");
}

#[test]
fn parse_imports_import_several() {
    assert_eq!(s("import_several"), "(import os sys)");
}

#[test]
fn parse_imports_import_several_with_alias() {
    assert_eq!(s("import_several_with_alias"), "(import os numpy:np)");
}

#[test]
fn parse_imports_from_import() {
    assert_eq!(s("from_import"), "(from os path)");
}

#[test]
fn parse_imports_from_import_several() {
    assert_eq!(s("from_import_several"), "(from os path sep)");
}

#[test]
fn parse_imports_from_import_as() {
    assert_eq!(s("from_import_as"), "(from os path:p)");
}

#[test]
fn parse_imports_from_import_several_with_alias() {
    assert_eq!(s("from_import_several_with_alias"), "(from os path:p sep)");
}

#[test]
fn parse_imports_from_dotted_import() {
    assert_eq!(s("from_dotted_import"), "(from os.path join)");
}

#[test]
fn parse_imports_from_import_star() {
    assert_eq!(s("from_import_star"), "(from os *)");
}

#[test]
fn parse_imports_from_import_parenthesized() {
    assert_eq!(s("from_import_parenthesized"), "(from os path sep)");
}

#[test]
fn parse_imports_from_import_parenthesized_trailing_comma() {
    assert_eq!(s("from_import_parenthesized_trailing_comma"), "(from os path sep)");
}

#[test]
fn parse_imports_from_import_parenthesized_multiline() {
    assert_eq!(s("from_import_parenthesized_multiline"), "(from os path sep)");
}

#[test]
fn parse_imports_relative_import() {
    assert_eq!(s("relative_import"), "(from . sibling)");
}

#[test]
fn parse_imports_relative_import_two_dots() {
    assert_eq!(s("relative_import_two_dots"), "(from .. parent)");
}

#[test]
fn parse_imports_relative_import_three_dots() {
    assert_eq!(s("relative_import_three_dots"), "(from ... grandparent)");
}

#[test]
fn parse_imports_relative_import_with_module() {
    assert_eq!(s("relative_import_with_module"), "(from .pkg thing)");
}

#[test]
fn parse_imports_relative_import_deep() {
    assert_eq!(s("relative_import_deep"), "(from ...pkg.sub thing)");
}

#[test]
fn parse_imports_import_in_block() {
    assert_eq!(s("import_in_block"), "(if a ((import os)) ())");
}

#[test]
fn parse_imports_import_then_statement() {
    assert_eq!(many("import_then_statement"), vec!["(import os)", "(assign (x) 1)"]);
}

#[test]
fn parse_imports_imports_separated_by_semicolon() {
    assert_eq!(many("imports_separated_by_semicolon"), vec!["(import os)", "(import sys)"]);
}

#[test]
fn parse_imports_import_missing_name() {
    assert_rejected(F, "import_missing_name");
}

#[test]
fn parse_imports_import_trailing_comma() {
    assert_rejected(F, "import_trailing_comma");
}

#[test]
fn parse_imports_import_as_missing_name() {
    assert_rejected(F, "import_as_missing_name");
}

#[test]
fn parse_imports_import_star_without_from() {
    assert_rejected(F, "import_star_without_from");
}

#[test]
fn parse_imports_from_missing_import() {
    assert_rejected(F, "from_missing_import");
}

#[test]
fn parse_imports_from_missing_names() {
    assert_rejected(F, "from_missing_names");
}

#[test]
fn parse_imports_from_import_unparenthesized_trailing_comma() {
    assert_rejected(F, "from_import_unparenthesized_trailing_comma");
}

#[test]
fn parse_imports_from_import_star_with_alias() {
    assert_rejected(F, "from_import_star_with_alias");
}

#[test]
fn parse_imports_from_import_star_mixed_with_names() {
    assert_rejected(F, "from_import_star_mixed_with_names");
}

#[test]
fn parse_imports_from_import_unclosed_parenthesis() {
    assert_rejected(F, "from_import_unclosed_parenthesis");
}
