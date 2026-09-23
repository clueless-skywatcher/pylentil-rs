use super::*;

#[test]
fn empty_input_lexes_to_a_single_eof() {
    assert_eq!(kinds(""), Ok(vec![EOF]));
}

#[test]
fn a_lone_identifier() {
    assert_eq!(content("x"), Ok(vec!["Ident(x)".into()]));
}

#[test]
fn assignment_token_sequence() {
    assert_eq!(
        content("x = 1\n"),
        Ok(vec!["Ident(x)".into(), "Assign".into(), "Int(1)".into()])
    );
}

#[test]
fn newline_and_eof_are_kept_as_layout() {
    assert_eq!(kinds("x = 1\n"), Ok(vec![Ident, Assign, Int, Newline, EOF]));
}

#[test]
fn keywords_get_their_own_kinds() {
    assert_eq!(
        content("if x:\n"),
        Ok(vec!["If".into(), "Ident(x)".into(), "Colon".into()])
    );
}

#[test]
fn delimiters() {
    assert_eq!(
        kinds("()[]{},:;.").map(|k| k.len()),
        Ok(11), // ten delimiters plus EOF
        "every delimiter must be exactly one token"
    );
}

#[test]
fn arithmetic_operators() {
    assert_eq!(
        content("+ - * / % **"),
        Ok(vec![
            "Plus".into(),
            "Minus".into(),
            "Star".into(),
            "Slash".into(),
            "Percent".into(),
            "DoubleStar".into(),
        ])
    );
}

#[test]
fn comparison_operators() {
    assert_eq!(
        content("== != < > <= >="),
        Ok(vec![
            "DoubleEqual".into(),
            "NotEqual".into(),
            "Less".into(),
            "Greater".into(),
            "LessEqual".into(),
            "GreaterEqual".into(),
        ])
    );
}

#[test]
fn bitwise_operators() {
    assert_eq!(
        content("& | ^ ~"),
        Ok(vec![
            "Ampersand".into(),
            "VerticalBar".into(),
            "Caret".into(),
            "Tilde".into(),
        ])
    );
}

#[test]
fn string_token_value_excludes_the_quotes() {
    assert_eq!(content("\"hi\""), Ok(vec!["String(hi)".into()]));
}

#[test]
fn number_token_values_are_the_source_text() {
    assert_eq!(content("1 2.5"), Ok(vec!["Int(1)".into(), "Float(2.5)".into()]));
}

#[test]
fn spacing_between_tokens_is_insignificant() {
    assert_eq!(content("a+b"), content("a   +   b"));
}

#[test]
fn a_block_emits_one_indent_and_one_dedent() {
    let code = "if a:\n    b\n";
    assert_eq!(count_kind(code, Indent), Ok(1));
    assert_eq!(count_kind(code, Dedent), Ok(1));
}
