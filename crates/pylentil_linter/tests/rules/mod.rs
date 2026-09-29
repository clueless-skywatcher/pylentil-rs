use std::{fs, path::PathBuf};

use pylentil_ast::{code::PyCode, lexer::PyCommentSpan, parser::PyParser};
use pylentil_common::{errors::PylentilError, span::PySpan};
use pylentil_linter::{lint::Lint, lint_config::PylentilBuilder};

mod pycodestyle;
mod pylint;

const CHECK_COMMENT: &str = "# !";

pub fn fixture_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(rel)
}

pub fn test_rule(rule: &(dyn Lint + 'static)) {
    let parser_code = create_fixture_parse_tree(rule);
    let expected_locations: Vec<PySpan> = parser_code.metadata.comments
        .iter()
        .filter(|comment_span| matches!(comment_span, PyCommentSpan {
            comment: CHECK_COMMENT,
            ..
        }))
        .map(|comment_span| comment_span.span)
        .collect();

    let lint = PylentilBuilder::new()
        .with_lint(rule.code().to_string())
        .with_path(PathBuf::from(fixture_file_name(rule)))
        .build().unwrap();

    let violations = lint.check(&parser_code);
    
    let actual_locations: Vec<PySpan> = violations.iter()
        .map(|violation| violation.span)
        .collect();

    assert_eq!(expected_locations, actual_locations);
}

fn fixture_file_name(rule: &(dyn Lint + 'static)) -> String {
    format!("rules/{}/{}.py", rule.source().get_name(), rule.code())
}

fn create_fixture_parse_tree<'a>(rule: &'a (dyn Lint + 'static)) -> PyCode<'a> {
    let filename = fixture_path(fixture_file_name(rule).as_str());
    let contents = fs::read_to_string(&filename).map_err(|e| PylentilError::IOFailed {
        path: filename.to_str().unwrap().to_string(),
        reason: e.to_string(),
    });

    let contents = Box::leak(contents.unwrap().into_boxed_str());
    let parser = PyParser::new(contents, filename);
    parser.unwrap().parse().unwrap()
}