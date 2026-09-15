//! Lexer tests.
//!
//! Expectations are written against *Python*, not against what the lexer
//! currently does: a failure here is a statement about the language, not about
//! the code. Cases come from `fixtures/lexer/*.py`, one file per lexical area.

mod common;

use common::{case, cases, content, count_kind, kinds, lex_failures, lex_outcome, Outcome};
use pylentil_ast::PyTokenType::{Assign, Dedent, Ident, Indent, Int, Newline, EOF};
use pylentil_common::errors::PylentilError;

// ============================================================== basics ======
// The happy path: what already works, and what must keep working.

mod basics {
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
}

// ============================================================= numbers ======

mod numbers {
    use super::*;

    fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
        content(&case("lexer/numbers.py", name))
    }

    #[test]
    fn decimal_integers() {
        assert_eq!(tokens("decimal_integer"), Ok(vec!["Int(42)".into()]));
        assert_eq!(tokens("zero"), Ok(vec!["Int(0)".into()]));
        assert_eq!(tokens("leading_zeros"), Ok(vec!["Int(007)".into()]));
    }

    #[test]
    fn floats() {
        assert_eq!(tokens("simple_float"), Ok(vec!["Float(3.14)".into()]));
        assert_eq!(tokens("float_trailing_dot"), Ok(vec!["Float(2.)".into()]));
    }

    #[test]
    fn huge_integers_lex_even_though_they_overflow_i64() {
        // The lexer only carries the digits; range is the parser's problem.
        assert!(tokens("huge_integer").is_ok());
    }

    #[test]
    fn a_number_followed_by_a_keyword_splits() {
        // Python tokenises `1if` as 1 then `if`.
        assert_eq!(tokens("number_touching_identifier"), Ok(vec!["Int(1)".into(), "If".into()]));
    }

    #[test]
    fn leading_dot_float_is_one_number() {
        assert_eq!(tokens("float_leading_dot"), Ok(vec!["Float(.5)".into()]));
    }

    #[test]
    fn underscore_separators_are_part_of_the_number() {
        assert_eq!(tokens("underscore_separated"), Ok(vec!["Int(1_000_000)".into()]));
    }

    #[test]
    fn hex_octal_and_binary_literals_are_single_int_tokens() {
        assert_eq!(tokens("hexadecimal"), Ok(vec!["Int(0xDEADBEEF)".into()]));
        assert_eq!(tokens("hexadecimal_lowercase"), Ok(vec!["Int(0x1f)".into()]));
        assert_eq!(tokens("binary"), Ok(vec!["Int(0b1010)".into()]));
        assert_eq!(tokens("octal"), Ok(vec!["Int(0o755)".into()]));
    }

    #[test]
    fn exponent_notation_is_a_single_float_token() {
        assert_eq!(tokens("exponent"), Ok(vec!["Float(1e10)".into()]));
        assert_eq!(tokens("exponent_capital"), Ok(vec!["Float(1E10)".into()]));
        assert_eq!(tokens("negative_exponent"), Ok(vec!["Float(1.5e-3)".into()]));
    }

    #[test]
    fn imaginary_literals_are_a_single_token() {
        assert_eq!(tokens("imaginary").map(|t| t.len()), Ok(1));
        assert_eq!(tokens("imaginary_float").map(|t| t.len()), Ok(1));
    }
}

// ============================================================= strings ======

mod strings {
    use super::*;

    fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
        content(&case("lexer/strings.py", name))
    }

    #[test]
    fn both_quote_styles_carry_the_same_value() {
        assert_eq!(tokens("double_quoted"), Ok(vec!["String(hello)".into()]));
        assert_eq!(tokens("single_quoted"), Ok(vec!["String(hello)".into()]));
    }

    #[test]
    fn an_empty_string_is_still_a_string_token() {
        assert_eq!(tokens("empty"), Ok(vec!["String()".into()]));
    }

    #[test]
    fn an_escaped_quote_does_not_end_the_string() {
        assert_eq!(tokens("escaped_quote").map(|t| t.len()), Ok(1));
    }

    #[test]
    fn an_escaped_backslash_does_not_escape_the_closing_quote() {
        assert_eq!(tokens("escaped_backslash").map(|t| t.len()), Ok(1));
    }

    #[test]
    fn the_other_quote_style_is_ordinary_content() {
        assert_eq!(tokens("other_quote_inside"), Ok(vec!["String(it's fine)".into()]));
    }

    #[test]
    fn non_ascii_content_survives_byte_indexing() {
        assert_eq!(tokens("non_ascii_contents"), Ok(vec!["String(héllo wörld)".into()]));
        assert_eq!(tokens("emoji_contents"), Ok(vec!["String(shipped 🚀)".into()]));
    }

    #[test]
    fn adjacent_literals_stay_two_tokens() {
        assert_eq!(tokens("adjacent_concatenation").map(|t| t.len()), Ok(2));
    }

    #[test]
    fn a_hash_inside_a_string_is_not_a_comment() {
        assert_eq!(tokens("hash_inside_string"), Ok(vec!["String(# not a comment)".into()]));
    }

    #[test]
    fn an_unterminated_string_is_rejected() {
        assert!(matches!(lex_outcome(&case("lexer/strings.py", "unterminated_at_eof")), Outcome::Err(_)));
    }

    #[test]
    fn a_trailing_escape_at_eof_is_rejected() {
        assert!(matches!(lex_outcome(&case("lexer/strings.py", "trailing_escape_at_eof")), Outcome::Err(_)));
    }

    #[test]
    fn a_single_quoted_string_cannot_span_a_line_break() {
        // Python: unterminated string literal on line 1. The danger is silently
        // swallowing the next line and closing on a *later* quote.
        let code = "a = \"unterminated\nb = \"also\"\n";
        assert!(
            matches!(lex_outcome(code), Outcome::Err(_)),
            "a newline must terminate a single-quoted string, got {:?}",
            kinds(code)
        );
    }

    #[test]
    fn triple_quoted_strings_are_one_token() {
        assert_eq!(tokens("triple_double"), Ok(vec!["String(triple quoted)".into()]));
        assert_eq!(tokens("triple_single"), Ok(vec!["String(triple quoted)".into()]));
    }

    #[test]
    fn triple_quoted_strings_may_span_lines() {
        assert_eq!(tokens("triple_spanning_lines").map(|t| t.len()), Ok(1));
    }

    #[test]
    fn string_prefixes_belong_to_the_literal() {
        // f/r/b are part of the token, not a separate identifier.
        assert_eq!(tokens("f_string").map(|t| t.len()), Ok(1));
        assert_eq!(tokens("raw_string").map(|t| t.len()), Ok(1));
        assert_eq!(tokens("bytes_literal").map(|t| t.len()), Ok(1));
    }
}

// =========================================================== operators ======

mod operators {
    use super::*;

    fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
        content(&case("lexer/operators.py", name))
    }

    #[test]
    fn arithmetic_and_power() {
        assert_eq!(tokens("arithmetic").map(|t| t.len()), Ok(11));
        assert_eq!(
            tokens("power"),
            Ok(vec!["Ident(a)".into(), "DoubleStar".into(), "Ident(b)".into()])
        );
    }

    #[test]
    fn left_shift() {
        assert_eq!(
            tokens("left_shift"),
            Ok(vec!["Ident(a)".into(), "LShift".into(), "Ident(b)".into()])
        );
    }

    #[test]
    fn right_shift() {
        assert_eq!(
            tokens("right_shift"),
            Ok(vec!["Ident(a)".into(), "RShift".into(), "Ident(b)".into()])
        );
    }

    #[test]
    fn a_greater_followed_by_a_less_is_not_a_shift() {
        let toks = tokens("greater_then_less_adjacent");
        assert_eq!(
            toks,
            Ok(vec!["Ident(a)".into(), "Greater".into(), "Less".into(), "Ident(b)".into()]),
            "`><` is two comparison tokens, never a shift"
        );
    }

    #[test]
    fn comparisons_and_bitwise() {
        assert_eq!(tokens("comparisons").map(|t| t.len()), Ok(5));
        assert_eq!(tokens("bitwise").map(|t| t.len()), Ok(7));
        assert_eq!(tokens("invert"), Ok(vec!["Tilde".into(), "Ident(a)".into()]));
    }

    #[test]
    fn a_lone_bang_is_rejected() {
        assert!(matches!(lex_outcome(&case("lexer/operators.py", "lone_bang")), Outcome::Err(_)));
    }

    #[test]
    fn brackets_and_semicolons() {
        assert_eq!(tokens("brackets").map(|t| t.len()), Ok(6));
        assert_eq!(tokens("semicolon_separated").map(|t| t.len()), Ok(7));
    }

    #[test]
    fn floor_division_is_one_token() {
        assert_eq!(tokens("floor_division").map(|t| t.len()), Ok(3));
    }

    #[test]
    fn matrix_multiplication_operator() {
        assert_eq!(tokens("matrix_multiply").map(|t| t.len()), Ok(3));
    }

    #[test]
    fn augmented_assignment_operators_are_single_tokens() {
        let mut broken = Vec::new();
        for c in cases("lexer/operators.py") {
            if !c.name.starts_with("augmented_") {
                continue;
            }
            // `a OP= 1` is three tokens in Python.
            match content(&c.code) {
                Ok(t) if t.len() == 3 => {}
                other => broken.push(format!("  {}: {other:?}", c.name)),
            }
        }
        assert!(broken.is_empty(), "augmented assignment must be one token:\n{}", broken.join("\n"));
    }

    #[test]
    fn walrus_is_one_token() {
        assert_eq!(tokens("walrus").map(|t| t.len()), Ok(3));
    }

    #[test]
    fn return_arrow_is_one_token() {
        // def f() -> int:  ->  Def Ident LParen RParen Arrow Ident Colon
        assert_eq!(tokens("arrow").map(|t| t.len()), Ok(7));
    }

    #[test]
    fn decorator_at_sign_is_lexable() {
        assert!(lex_outcome(&case("lexer/operators.py", "decorator")).is_accepted());
    }

    #[test]
    fn ellipsis_is_one_token() {
        assert_eq!(tokens("ellipsis").map(|t| t.len()), Ok(1));
    }
}

// ======================================== identifiers and keywords ==========

mod identifiers {
    use super::*;

    fn tokens(name: &str) -> Result<Vec<String>, PylentilError> {
        content(&case("lexer/identifiers.py", name))
    }

    #[test]
    fn ordinary_identifiers() {
        assert_eq!(tokens("simple"), Ok(vec!["Ident(value)".into()]));
        assert_eq!(tokens("underscore_prefixed"), Ok(vec!["Ident(_private)".into()]));
        assert_eq!(tokens("dunder"), Ok(vec!["Ident(__init__)".into()]));
        assert_eq!(tokens("digits_inside"), Ok(vec!["Ident(item2)".into()]));
        assert_eq!(tokens("single_underscore"), Ok(vec!["Ident(_)".into()]));
    }

    #[test]
    fn a_keyword_is_only_a_keyword_when_it_is_the_whole_word() {
        assert_eq!(tokens("starts_with_keyword"), Ok(vec!["Ident(ifconfig)".into()]));
        assert_eq!(tokens("keyword_with_trailing_underscore"), Ok(vec!["Ident(class_)".into()]));
        assert_eq!(tokens("keyword_embedded"), Ok(vec!["Ident(my_if_helper)".into()]));
        assert_eq!(tokens("keyword_prefix_of_identifier"), Ok(vec!["Ident(nonlocality)".into()]));
    }

    #[test]
    fn every_keyword_lexes_to_a_keyword_token() {
        let toks = tokens("all_keywords").expect("keywords must lex");
        assert_eq!(toks.len(), 35, "expected all 35 keywords");
        let leaked: Vec<&String> = toks.iter().filter(|t| t.starts_with("Ident(")).collect();
        assert!(leaked.is_empty(), "these keywords lexed as identifiers: {leaked:?}");
    }

    #[test]
    fn soft_keywords_are_ordinary_identifiers() {
        assert_eq!(tokens("soft_keyword_match"), Ok(vec!["Ident(match)".into()]));
        assert_eq!(tokens("soft_keyword_case"), Ok(vec!["Ident(case)".into()]));
        assert_eq!(tokens("soft_keyword_type"), Ok(vec!["Ident(type)".into()]));
    }

    #[test]
    fn characters_that_are_not_python_are_rejected() {
        for name in ["dollar_sign", "question_mark", "backtick"] {
            assert!(
                matches!(lex_outcome(&case("lexer/identifiers.py", name)), Outcome::Err(_)),
                "{name} must be rejected"
            );
        }
    }

    #[test]
    fn identifiers_may_contain_non_ascii_letters() {
        assert_eq!(tokens("unicode_identifier"), Ok(vec!["Ident(café)".into()]));
        assert_eq!(tokens("cyrillic_identifier"), Ok(vec!["Ident(переменная)".into()]));
    }
}

// ============================================================ comments ======

mod comments {
    use super::*;

    #[test]
    fn a_hash_inside_a_string_is_not_a_comment() {
        assert_eq!(
            content(&case("lexer/comments.py", "hash_in_string_not_comment")),
            Ok(vec!["Ident(x)".into(), "Assign".into(), "String(# not a comment)".into()])
        );
    }

    #[test]
    fn comments_are_ignored_everywhere_they_may_appear() {
        let failures = lex_failures("lexer/comments.py");
        assert!(
            failures.is_empty(),
            "comments must be skipped by the lexer:\n{}",
            failures.join("\n")
        );
    }

    #[test]
    fn a_comment_contributes_no_tokens() {
        assert_eq!(content("x = 1  # assign one\n"), content("x = 1\n"));
    }

    #[test]
    fn a_comment_does_not_open_or_close_a_block() {
        let code = "if a:\n    # explain\n    b\n";
        assert_eq!(count_kind(code, Indent), Ok(1));
        assert_eq!(count_kind(code, Dedent), Ok(1));
    }

    #[test]
    fn a_file_of_only_comments_lexes_to_no_content() {
        assert_eq!(content("# nothing but this\n"), Ok(vec![]));
    }
}

// ========================================================= indentation ======

mod indentation {
    use super::*;

    fn layout(name: &str) -> (Result<usize, PylentilError>, Result<usize, PylentilError>) {
        let code = case("lexer/indentation.py", name);
        (count_kind(&code, Indent), count_kind(&code, Dedent))
    }

    #[test]
    fn one_block_opens_and_closes_once() {
        assert_eq!(layout("single_level"), (Ok(1), Ok(1)));
    }

    #[test]
    fn nested_blocks_nest_their_layout_tokens() {
        assert_eq!(layout("two_levels"), (Ok(2), Ok(2)));
    }

    #[test]
    fn dedenting_one_level_closes_one_block() {
        assert_eq!(layout("dedent_one_level"), (Ok(2), Ok(2)));
    }

    #[test]
    fn dedenting_to_module_level_closes_every_block() {
        assert_eq!(layout("dedent_to_module_level"), (Ok(2), Ok(2)));
    }

    #[test]
    fn a_tab_opens_a_block_just_like_spaces() {
        assert_eq!(layout("tab_indent"), (Ok(1), Ok(1)));
    }

    #[test]
    fn indent_width_does_not_matter() {
        assert_eq!(layout("eight_space_indent"), (Ok(1), Ok(1)));
    }

    #[test]
    fn a_block_left_open_at_eof_is_closed() {
        assert_eq!(layout("block_at_eof"), (Ok(1), Ok(1)));
    }

    #[test]
    fn deep_nesting_stays_balanced() {
        assert_eq!(layout("deep_nesting"), (Ok(4), Ok(4)));
    }

    #[test]
    fn a_blank_line_does_not_close_a_block() {
        assert_eq!(layout("blank_line_inside_block"), (Ok(1), Ok(1)));
    }

    #[test]
    fn consecutive_blocks_each_open_and_close() {
        assert_eq!(layout("reindent_after_dedent"), (Ok(2), Ok(2)));
        assert_eq!(layout("blank_line_between_blocks"), (Ok(2), Ok(2)));
    }

    #[test]
    fn every_fixture_balances_indents_against_dedents() {
        let mut unbalanced = Vec::new();
        for c in cases("lexer/indentation.py") {
            match (count_kind(&c.code, Indent), count_kind(&c.code, Dedent)) {
                (Ok(i), Ok(d)) if i == d => {}
                (Ok(i), Ok(d)) => unbalanced.push(format!("  {}: {i} indents, {d} dedents", c.name)),
                _ => {} // rejection is covered by the dedicated tests below
            }
        }
        assert!(unbalanced.is_empty(), "layout tokens must balance:\n{}", unbalanced.join("\n"));
    }

    #[test]
    fn an_indented_first_line_is_an_error() {
        // Python: IndentationError: unexpected indent
        assert!(
            matches!(lex_outcome(&case("lexer/indentation.py", "first_line_indented")), Outcome::Err(_)),
            "module-level code cannot start indented"
        );
    }

    #[test]
    fn an_inconsistent_dedent_is_an_error_not_a_panic() {
        // Python: IndentationError: unindent does not match any outer level
        match lex_outcome(&case("lexer/indentation.py", "inconsistent_dedent")) {
            Outcome::Err(_) => {}
            Outcome::Ok(t) => panic!("accepted an inconsistent dedent: {t:?}"),
            Outcome::Panic(m) => panic!("panicked instead of erroring: {m}"),
        }
    }

    #[test]
    fn a_dedent_to_an_unknown_level_is_an_error_not_a_panic() {
        match lex_outcome(&case("lexer/indentation.py", "dedent_to_unknown_level")) {
            Outcome::Err(_) => {}
            Outcome::Ok(t) => panic!("accepted a dedent to an unvisited level: {t:?}"),
            Outcome::Panic(m) => panic!("panicked instead of erroring: {m}"),
        }
    }
}

// ============================================ whitespace and continuations ==

mod whitespace {
    use super::*;

    #[test]
    fn mixing_tabs_and_spaces_in_one_indent_is_rejected() {
        for name in ["tab_then_spaces", "spaces_then_tab"] {
            assert_eq!(
                lex_outcome(&case("lexer/whitespace.py", name)).error(),
                Some(&PylentilError::MixedSpacesAndTabs),
                "{name} must be rejected"
            );
        }
    }

    #[test]
    fn repeated_spaces_are_insignificant() {
        assert_eq!(
            content(&case("lexer/whitespace.py", "multiple_spaces_between_tokens")),
            content(&case("lexer/whitespace.py", "no_spaces_between_tokens"))
        );
    }

    #[test]
    fn a_backslash_joins_the_next_line() {
        let code = case("lexer/whitespace.py", "explicit_line_continuation");
        assert_eq!(count_kind(&code, Newline), Ok(1), "a continued line is one logical line");
    }

    #[test]
    fn a_newline_inside_brackets_does_not_break_the_statement() {
        let mut broken = Vec::new();
        for name in [
            "implicit_continuation_parens",
            "implicit_continuation_brackets",
            "implicit_continuation_braces",
            "implicit_continuation_call",
        ] {
            let code = case("lexer/whitespace.py", name);
            match count_kind(&code, Newline) {
                Ok(1) => {}
                other => broken.push(format!("  {name}: {other:?} newlines")),
            }
        }
        assert!(
            broken.is_empty(),
            "newlines inside brackets are not statement breaks:\n{}",
            broken.join("\n")
        );
    }

    #[test]
    fn indentation_inside_brackets_does_not_open_a_block() {
        let code = case("lexer/whitespace.py", "indented_continuation_line_looks_like_block");
        assert_eq!(count_kind(&code, Indent), Ok(0));
    }
}

// ========================================================== robustness ======

mod robustness {
    use super::*;

    #[test]
    fn a_file_without_a_trailing_newline_is_fine() {
        assert_eq!(content("x = 1"), Ok(vec!["Ident(x)".into(), "Assign".into(), "Int(1)".into()]));
    }

    #[test]
    fn a_file_of_only_newlines_has_no_content() {
        assert_eq!(content("\n\n\n"), Ok(vec![]));
    }

    #[test]
    fn a_whitespace_only_file_opens_no_block() {
        // Leading spaces on an otherwise blank line are not an indent.
        assert_eq!(count_kind("   \n\n", Indent), Ok(0));
    }

    #[test]
    fn crlf_line_endings_are_accepted() {
        assert_eq!(kinds("x = 1\r\ny = 2\r\n").map(|k| k.len()), Ok(9));
    }

    #[test]
    fn a_utf8_bom_is_skipped() {
        assert_eq!(content("\u{feff}x = 1\n").map(|t| t.len()), Ok(3));
    }

    #[test]
    fn a_null_byte_is_rejected() {
        assert!(matches!(lex_outcome("x = \0\n"), Outcome::Err(_)));
    }

    #[test]
    fn deeply_nested_brackets_do_not_blow_the_stack() {
        let code = format!("{}{}\n", "(".repeat(500), ")".repeat(500));
        assert!(!matches!(lex_outcome(&code), Outcome::Panic(_)));
    }

    #[test]
    fn a_long_file_lexes_correctly() {
        // NB: `indent_pass` re-clones the remaining token vector per token, so
        // this is quadratic — 100 lines ~10ms, 800 lines ~600ms. Correctness
        // only here; the cost is recorded in the parser's own notes.
        let code: String = (0..200).map(|i| format!("x{i} = {i} + 1\n")).collect();
        assert_eq!(kinds(&code).map(|k| k.len()), Ok(200 * 6 + 1));
    }
}

// Small conveniences used above.
trait OutcomeExt {
    fn is_accepted(&self) -> bool;
    fn error(&self) -> Option<&PylentilError>;
}

impl<T> OutcomeExt for Outcome<T> {
    fn is_accepted(&self) -> bool {
        matches!(self, Outcome::Ok(_))
    }

    fn error(&self) -> Option<&PylentilError> {
        match self {
            Outcome::Err(e) => Some(e),
            _ => None,
        }
    }
}
