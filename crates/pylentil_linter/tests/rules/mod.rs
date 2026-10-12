mod test_pycodestyle;
mod test_pylint;

use std::{fs, path::PathBuf};

use pylentil_ast::{code::PyCode, parser::PyParser};
use pylentil_common::errors::PylentilError;
use pylentil_linter::{lint::Lint, lint_config::PylentilBuilder};

const CHECK_COMMENT: &str = "# !";

pub fn fixture_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(rel)
}

pub fn test_rule(rule: &(dyn Lint + 'static)) {
    let parser_code = create_fixture_parse_tree(rule);

    let lint = PylentilBuilder::new()
        .with_lint(rule.code().to_string())
        .with_path(fixture_path(&fixture_file_name(rule)))
        .build()
        .unwrap();

    let violations = lint.check(&parser_code);
    let mut expected_locations = expected_spans(&parser_code);
    let mut actual_locations: Vec<(usize, usize)> = violations
        .iter()
        .map(|violation| (violation.span.start, violation.span.end_or_start()))
        .collect();

    expected_locations.sort();
    actual_locations.sort();
    assert_eq!(expected_locations, actual_locations);
}

fn expected_spans(code: &PyCode) -> Vec<(usize, usize)> {
    let mut expected = Vec::new();
    let line_starts = &code.metadata.line_starts;

    for comment in &code.metadata.comments {
        if !comment.comment.starts_with(CHECK_COMMENT) {
            continue;
        }

        let columns = column_spans(comment.comment);
        assert!(
            !columns.is_empty(),
            "check comment `{}` has no column spans",
            comment.comment
        );

        let comment_line = line_index(line_starts, comment.span.start) + 1;
        for (start, end) in columns {
            let start_line = start.0.unwrap_or(comment_line);
            let end_line = end.0.unwrap_or(comment_line);
            let start_byte = line_starts[start_line - 1] + start.1 - 1;
            let end_byte = line_starts[end_line - 1] + end.1 - 1;
            expected.push((start_byte, end_byte));
        }
    }

    expected
}

fn column_spans(comment: &str) -> Vec<((Option<usize>, usize), (Option<usize>, usize))> {
    let mut spans = Vec::new();
    let bytes = comment.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] != b'(' {
            i += 1;
            continue;
        }
        i += 1;
        let start = parse_endpoint(bytes, &mut i);
        i = skip_separator(bytes, i);
        let end = parse_endpoint(bytes, &mut i);
        while i < bytes.len() && bytes[i] != b')' {
            i += 1;
        }
        if i < bytes.len() {
            i += 1;
        }
        spans.push((start, end));
    }

    spans
}

fn parse_endpoint(bytes: &[u8], i: &mut usize) -> (Option<usize>, usize) {
    let first = parse_column(bytes, i);
    if *i < bytes.len() && bytes[*i] == b':' {
        *i += 1;
        (Some(first), parse_column(bytes, i))
    } else {
        (None, first)
    }
}

fn parse_column(bytes: &[u8], i: &mut usize) -> usize {
    while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
        *i += 1;
    }
    let start = *i;
    while *i < bytes.len() && bytes[*i].is_ascii_digit() {
        *i += 1;
    }
    std::str::from_utf8(&bytes[start..*i])
        .ok()
        .and_then(|text| text.parse().ok())
        .unwrap_or_else(|| panic!("expected a column number in a check comment"))
}

fn skip_separator(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
        i += 1;
    }
    i
}

fn line_index(line_starts: &[usize], pos: usize) -> usize {
    let mut i = line_starts.len() - 1;
    while i > 0 && line_starts[i] > pos {
        i -= 1;
    }
    i
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
