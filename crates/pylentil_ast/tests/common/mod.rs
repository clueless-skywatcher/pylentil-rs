use std::fs;

struct TestCase {
    name: String,
    code: String
}

fn read_cases_from_file(filename: &str) -> Vec<TestCase> {
    let mut vec = Vec::new();

    let file_code = fs::read_to_string(filename);

    vec
}