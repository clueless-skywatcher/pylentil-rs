use super::*;
const F: &str = "parser/imports.py";

#[test]
fn parse_imports_import() {
    assert_eq!(stmt(F, "import"), import(vec![alias("os", None)]));
}

#[test]
fn parse_imports_import_dotted() {
    assert_eq!(
        stmt(F, "import_dotted"),
        import(vec![alias("os.path", None)])
    );
}

#[test]
fn parse_imports_import_deeply_dotted() {
    assert_eq!(
        stmt(F, "import_deeply_dotted"),
        import(vec![alias("a.b.c", None)])
    );
}

#[test]
fn parse_imports_import_as() {
    assert_eq!(
        stmt(F, "import_as"),
        import(vec![alias("numpy", Some("np"))])
    );
}

#[test]
fn parse_imports_import_dotted_as() {
    assert_eq!(
        stmt(F, "import_dotted_as"),
        import(vec![alias("os.path", Some("p"))])
    );
}

#[test]
fn parse_imports_import_several() {
    assert_eq!(
        stmt(F, "import_several"),
        import(vec![alias("os", None), alias("sys", None)])
    );
}

#[test]
fn parse_imports_import_several_with_alias() {
    assert_eq!(
        stmt(F, "import_several_with_alias"),
        import(vec![alias("os", None), alias("numpy", Some("np"))])
    );
}

#[test]
fn parse_imports_from_import() {
    assert_eq!(
        stmt(F, "from_import"),
        import_from(Some("os"), vec![alias("path", None)], None)
    );
}

#[test]
fn parse_imports_from_import_several() {
    assert_eq!(
        stmt(F, "from_import_several"),
        import_from(
            Some("os"),
            vec![alias("path", None), alias("sep", None)],
            None
        )
    );
}

#[test]
fn parse_imports_from_import_as() {
    assert_eq!(
        stmt(F, "from_import_as"),
        import_from(Some("os"), vec![alias("path", Some("p"))], None)
    );
}

#[test]
fn parse_imports_from_import_several_with_alias() {
    assert_eq!(
        stmt(F, "from_import_several_with_alias"),
        import_from(
            Some("os"),
            vec![alias("path", Some("p")), alias("sep", None)],
            None
        )
    );
}

#[test]
fn parse_imports_from_dotted_import() {
    assert_eq!(
        stmt(F, "from_dotted_import"),
        import_from(Some("os.path"), vec![alias("join", None)], None)
    );
}

#[test]
fn parse_imports_from_import_star() {
    assert_eq!(
        stmt(F, "from_import_star"),
        import_from(Some("os"), vec![alias("*", None)], None)
    );
}

#[test]
fn parse_imports_from_import_parenthesized() {
    assert_eq!(
        stmt(F, "from_import_parenthesized"),
        import_from(
            Some("os"),
            vec![alias("path", None), alias("sep", None)],
            None
        )
    );
}

#[test]
fn parse_imports_from_import_parenthesized_trailing_comma() {
    assert_eq!(
        stmt(F, "from_import_parenthesized_trailing_comma"),
        import_from(
            Some("os"),
            vec![alias("path", None), alias("sep", None)],
            None
        )
    );
}

#[test]
fn parse_imports_from_import_parenthesized_multiline() {
    assert_eq!(
        stmt(F, "from_import_parenthesized_multiline"),
        import_from(
            Some("os"),
            vec![alias("path", None), alias("sep", None)],
            None
        )
    );
}

#[test]
fn parse_imports_relative_import() {
    assert_eq!(
        stmt(F, "relative_import"),
        import_from(None, vec![alias("sibling", None)], Some(1))
    );
}

#[test]
fn parse_imports_relative_import_two_dots() {
    assert_eq!(
        stmt(F, "relative_import_two_dots"),
        import_from(None, vec![alias("parent", None)], Some(2))
    );
}

#[test]
fn parse_imports_relative_import_three_dots() {
    assert_eq!(
        stmt(F, "relative_import_three_dots"),
        import_from(None, vec![alias("grandparent", None)], Some(3))
    );
}

#[test]
fn parse_imports_relative_import_with_module() {
    assert_eq!(
        stmt(F, "relative_import_with_module"),
        import_from(Some("pkg"), vec![alias("thing", None)], Some(1))
    );
}

#[test]
fn parse_imports_relative_import_deep() {
    assert_eq!(
        stmt(F, "relative_import_deep"),
        import_from(Some("pkg.sub"), vec![alias("thing", None)], Some(3))
    );
}

#[test]
fn parse_imports_import_in_block() {
    assert_eq!(
        stmt(F, "import_in_block"),
        if_stmt(name("a"), vec![import(vec![alias("os", None)])], vec![])
    );
}

#[test]
fn parse_imports_import_then_statement() {
    assert_eq!(
        body(F, "import_then_statement"),
        vec![
            import(vec![alias("os", None)]),
            assign(vec![store(name("x"))], int(1))
        ]
    );
}

#[test]
fn parse_imports_imports_separated_by_semicolon() {
    assert_eq!(
        body(F, "imports_separated_by_semicolon"),
        vec![
            import(vec![alias("os", None)]),
            import(vec![alias("sys", None)])
        ]
    );
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
