
mod common;

use common::{case, cases, expr, one_stmt, parse_failures, parse_module, parse_outcome, stmts, Outcome};
use pylentil_ast::ast::{PyExpr, PyRefContext, PyStatement};

/// Parse one named case and render it.
fn e(fixture: &str, name: &str) -> String {
    let code = case(fixture, name);
    match expr(&code) {
        Ok(rendered) => rendered,
        Err(err) => format!("<rejected: {err:?}>"),
    }
}

fn assert_rejected(fixture: &str, name: &str) {
    let code = case(fixture, name);
    match parse_outcome(&code) {
        Outcome::Err(_) => {}
        Outcome::Ok(ast) => panic!("accepted invalid Python `{}` as {ast:?}", code.trim()),
        Outcome::Panic(m) => panic!("panicked instead of erroring on `{}`: {m}", code.trim()),
    }
}


mod basics {
    use super::*;

    #[test]
    fn integer_literal() {
        assert_eq!(expr("42\n"), Ok("42".into()));
    }

    #[test]
    fn float_literal() {
        assert_eq!(expr("3.5\n"), Ok("3.5".into()));
    }

    #[test]
    fn string_literal() {
        assert_eq!(expr("\"text\"\n"), Ok("str(text)".into()));
    }

    #[test]
    fn booleans_and_none() {
        assert_eq!(expr("True\n"), Ok("True".into()));
        assert_eq!(expr("False\n"), Ok("False".into()));
        assert_eq!(expr("None\n"), Ok("None".into()));
    }

    #[test]
    fn a_name() {
        assert_eq!(expr("value\n"), Ok("value".into()));
    }

    #[test]
    fn addition() {
        assert_eq!(expr("1 + 2\n"), Ok("(+ 1 2)".into()));
    }

    #[test]
    fn multiplication_binds_tighter_than_addition() {
        assert_eq!(expr("1 + 2 * 3\n"), Ok("(+ 1 (* 2 3))".into()));
    }

    #[test]
    fn parentheses_are_transparent() {
        assert_eq!(expr("(42)\n"), Ok("42".into()));
        assert_eq!(expr("((42))\n"), Ok("42".into()));
    }

    #[test]
    fn a_comparison() {
        assert_eq!(expr("a == b\n"), Ok("(compare a == b)".into()));
    }

    #[test]
    fn a_parenthesized_tuple() {
        assert_eq!(expr("(a, b)\n"), Ok("(ptuple a b)".into()));
    }

    #[test]
    fn simple_assignment() {
        assert_eq!(one_stmt("x = 1\n"), Ok("(assign (x) 1)".into()));
    }

    #[test]
    fn an_if_statement() {
        assert_eq!(one_stmt("if a:\n    b\n"), Ok("(if a (b) ())".into()));
    }

    #[test]
    fn an_if_else_statement() {
        assert_eq!(one_stmt("if a:\n    b\nelse:\n    c\n"), Ok("(if a (b) (c))".into()));
    }

    #[test]
    fn a_module_holds_several_statements() {
        assert_eq!(stmts("a\nb\nc\n"), Ok(vec!["a".into(), "b".into(), "c".into()]));
    }

    #[test]
    fn an_empty_module_has_no_statements() {
        assert_eq!(stmts(""), Ok(vec![]));
        assert_eq!(stmts("\n\n\n"), Ok(vec![]));
    }
}


mod literals {
    use super::*;
    const F: &str = "parser/literals.py";

    #[test]
    fn numbers_and_text() {
        assert_eq!(e(F, "integer"), "42");
        assert_eq!(e(F, "zero"), "0");
        assert_eq!(e(F, "float"), "3.5");
        assert_eq!(e(F, "float_trailing_dot"), "2.0");
        assert_eq!(e(F, "string"), "str(text)");
        assert_eq!(e(F, "single_quoted_string"), "str(text)");
        assert_eq!(e(F, "empty_string"), "str()");
    }

    #[test]
    fn constants_and_names() {
        assert_eq!(e(F, "true"), "True");
        assert_eq!(e(F, "false"), "False");
        assert_eq!(e(F, "none"), "None");
        assert_eq!(e(F, "name"), "value");
    }

    #[test]
    fn an_escape_sequence_stays_inside_the_string() {
        assert_eq!(e(F, "escape_in_string"), "str(tab\\there)");
    }

    #[test]
    fn a_negative_literal() {
        assert_eq!(e(F, "negative_literal"), "(neg 1)");
    }

    #[test]
    fn an_integer_too_large_for_i64_is_still_an_integer() {
        // Python integers are arbitrary precision; overflowing i64 is not a
        // syntax error.
        assert!(
            !e(F, "huge_integer").starts_with("<rejected"),
            "99999999999999999999 must parse, not be rejected as invalid syntax"
        );
    }

    #[test]
    fn ellipsis_is_a_constant() {
        assert_eq!(e(F, "ellipsis"), "...");
    }
}

mod arithmetic {
    use super::*;
    const F: &str = "parser/arithmetic.py";

    #[test]
    fn addition_and_left_associativity() {
        assert_eq!(e(F, "addition"), "(+ 1 2)");
        assert_eq!(e(F, "subtraction_is_left_associative"), "(- (- 1 2) 3)");
        assert_eq!(e(F, "division_is_left_associative"), "(/ (/ 8 4) 2)");
    }

    #[test]
    fn multiplication_binds_tighter_than_addition() {
        assert_eq!(e(F, "multiplication_binds_tighter_than_addition"), "(+ 1 (* 2 3))");
        assert_eq!(e(F, "multiplication_on_the_left"), "(+ (* 2 3) 1)");
        assert_eq!(e(F, "parentheses_override_precedence"), "(* (+ 1 2) 3)");
    }

    #[test]
    fn power_is_right_associative() {
        assert_eq!(e(F, "power_is_right_associative"), "(** 2 (** 3 2))");
    }

    #[test]
    fn power_binds_tighter_than_multiplication() {
        assert_eq!(e(F, "power_binds_tighter_than_multiplication"), "(* 2 (** 3 2))");
        assert_eq!(e(F, "power_on_the_left"), "(* (** 2 3) 4)");
    }

    #[test]
    fn modulo_shares_precedence_with_multiplication() {
        assert_eq!(e(F, "modulo_and_addition"), "(+ (% 7 3) 1)");
        assert_eq!(e(F, "modulo_and_multiplication"), "(% (* 2 7) 3)");
    }

    #[test]
    fn a_long_chain_groups_left_to_right() {
        assert_eq!(e(F, "long_chain"), "(- (+ 1 (* 2 3)) (/ 4 5))");
    }

    #[test]
    fn nested_parentheses() {
        assert_eq!(e(F, "nested_parentheses"), "(* (+ 1 2) (- 3 4))");
    }

    #[test]
    fn unary_minus() {
        assert_eq!(e(F, "unary_minus"), "(neg 1)");
        assert_eq!(e(F, "unary_minus_on_name"), "(neg x)");
        assert_eq!(e(F, "unary_plus"), "(pos 1)");
        assert_eq!(e(F, "double_unary_minus"), "(neg (neg 1))");
        assert_eq!(e(F, "subtract_a_negative"), "(- 1 (neg 2))");
    }

    #[test]
    fn unary_minus_binds_looser_than_power() {
        // -2 ** 2 is -4, not 4.
        assert_eq!(e(F, "unary_minus_binds_looser_than_power"), "(neg (** 2 2))");
    }

    #[test]
    fn unary_minus_binds_tighter_than_multiplication() {
        assert_eq!(e(F, "unary_minus_binds_tighter_than_multiplication"), "(* (neg 2) 3)");
    }

    #[test]
    fn floor_division() {
        assert_eq!(e(F, "floor_division"), "(// 7 2)");
    }

    #[test]
    fn shifts_bind_looser_than_addition() {
        assert_eq!(e(F, "shift_binds_looser_than_addition"), "(<< 1 (+ 2 3))");
        assert_eq!(e(F, "shift_is_left_associative"), "(<< (<< 1 2) 3)");
    }

    #[test]
    fn malformed_arithmetic_is_rejected() {
        for name in [
            "missing_right_operand",
            "missing_left_operand",
            "unbalanced_open_paren",
            "unbalanced_close_paren",
        ] {
            assert_rejected(F, name);
        }
    }
}

mod bitwise {
    use super::*;
    const F: &str = "parser/bitwise.py";

    #[test]
    fn and_binds_tighter_than_xor_binds_tighter_than_or() {
        assert_eq!(e(F, "bitand_binds_tighter_than_bitor"), "(| a (& b c))");
        assert_eq!(e(F, "bitxor_binds_tighter_than_bitor"), "(| a (^ b c))");
        assert_eq!(e(F, "bitand_binds_tighter_than_bitxor"), "(^ a (& b c))");
        assert_eq!(e(F, "all_three_bitwise"), "(| a (^ b (& c d)))");
    }

    #[test]
    fn bitor_is_left_associative() {
        assert_eq!(e(F, "bitor_is_left_associative"), "(| (| a b) c)");
    }

    #[test]
    fn arithmetic_and_shifts_bind_tighter_than_bitwise() {
        assert_eq!(e(F, "shift_binds_tighter_than_bitand"), "(& a (<< b c))");
        assert_eq!(e(F, "arithmetic_binds_tighter_than_bitand"), "(& a (+ b c))");
    }

    #[test]
    fn comparison_binds_looser_than_bitwise() {
        // (a & b) == c, not a & (b == c)
        assert_eq!(e(F, "comparison_binds_looser_than_bitand"), "(compare (& a b) == c)");
        assert_eq!(e(F, "comparison_binds_looser_than_bitor"), "(compare (| a b) == c)");
    }

    #[test]
    fn invert() {
        assert_eq!(e(F, "invert"), "(~ a)");
        assert_eq!(e(F, "invert_binds_tighter_than_bitand"), "(& (~ a) b)");
    }
}

mod comparisons {
    use super::*;
    const F: &str = "parser/comparisons.py";

    #[test]
    fn simple_comparisons() {
        assert_eq!(e(F, "equal"), "(compare a == b)");
        assert_eq!(e(F, "not_equal"), "(compare a != b)");
        assert_eq!(e(F, "less_than"), "(compare a < b)");
        assert_eq!(e(F, "greater_or_equal"), "(compare a >= b)");
    }

    #[test]
    fn comparisons_chain_into_one_node() {
        // Python folds a < b < c into a single Compare, not nested ones.
        assert_eq!(e(F, "chained"), "(compare a < b < c)");
        assert_eq!(e(F, "chained_mixed_operators"), "(compare a < b == c)");
        assert_eq!(e(F, "chained_three_operators"), "(compare a < b <= c < d)");
    }

    #[test]
    fn arithmetic_binds_tighter_than_comparison() {
        assert_eq!(e(F, "comparison_binds_looser_than_addition"), "(compare (+ a 1) == b)");
        assert_eq!(e(F, "comparison_on_both_sides"), "(compare (+ a 1) == (* b 2))");
        assert_eq!(e(F, "chained_with_arithmetic"), "(compare 1 < (+ x 1) < 10)");
    }

    #[test]
    fn parentheses_stop_a_chain() {
        assert_eq!(e(F, "parenthesized_comparison"), "(compare (compare a < b) < c)");
    }

    #[test]
    fn identity_and_membership_operators() {
        assert_eq!(e(F, "is_operator"), "(compare a is b)");
        assert_eq!(e(F, "is_not_operator"), "(compare a is-not b)");
        assert_eq!(e(F, "in_operator"), "(compare a in b)");
        assert_eq!(e(F, "not_in_operator"), "(compare a not-in b)");
    }

    #[test]
    fn not_binds_looser_than_comparison() {
        assert_eq!(e(F, "not_operator"), "(not a)");
        assert_eq!(e(F, "not_with_comparison"), "(not (compare a == b))");
    }

    #[test]
    fn malformed_comparisons_are_rejected() {
        assert_rejected(F, "missing_right_operand");
        assert_rejected(F, "double_operator");
    }
}

mod boolean {
    use super::*;
    const F: &str = "parser/boolean.py";

    #[test]
    fn and_and_or() {
        assert_eq!(e(F, "and_expression"), "(and a b)");
        assert_eq!(e(F, "or_expression"), "(or a b)");
    }

    #[test]
    fn and_binds_tighter_than_or() {
        assert_eq!(e(F, "and_binds_tighter_than_or"), "(or a (and b c))");
        assert_eq!(e(F, "and_on_the_left"), "(or (and a b) c)");
        assert_eq!(e(F, "parentheses_override_precedence"), "(and (or a b) c)");
    }

    #[test]
    fn not_binds_tighter_than_and() {
        assert_eq!(e(F, "not_binds_tighter_than_and"), "(and (not a) b)");
        assert_eq!(e(F, "double_not"), "(not (not a))");
        assert_eq!(e(F, "not_with_parentheses"), "(not (and a b))");
    }

    #[test]
    fn a_chain_collapses_into_one_node() {
        // Python flattens a and b and c into a single BoolOp with three values.
        assert_eq!(e(F, "chained_and"), "(and a b c)");
        assert_eq!(e(F, "chained_or"), "(or a b c)");
    }

    #[test]
    fn comparison_binds_tighter_than_and() {
        assert_eq!(e(F, "comparison_in_boolean"), "(and (compare a < b) (compare c < d))");
    }

    #[test]
    fn conditional_expressions() {
        assert_eq!(e(F, "ternary"), "(ifexp b a c)");
        assert_eq!(e(F, "ternary_nested"), "(ifexp b a (ifexp d c e))");
        assert_eq!(e(F, "ternary_with_arithmetic"), "(ifexp c (+ 1 2) 3)");
    }

    #[test]
    fn a_missing_operand_is_rejected() {
        assert_rejected(F, "and_missing_operand");
    }
}


mod tuples {
    use super::*;
    const F: &str = "parser/tuples.py";

    #[test]
    fn a_bare_pair() {
        assert_eq!(e(F, "bare_pair"), "(tuple a b)");
    }

    #[test]
    fn a_parenthesized_pair_is_marked_parenthesized() {
        assert_eq!(e(F, "parenthesized_pair"), "(ptuple a b)");
    }

    #[test]
    fn parentheses_alone_do_not_make_a_tuple() {
        assert_eq!(e(F, "parenthesized_single_is_not_a_tuple"), "a");
    }

    #[test]
    fn nesting_is_preserved() {
        assert_eq!(e(F, "nested_on_the_left"), "(tuple (ptuple a b) c)");
        assert_eq!(e(F, "nested_on_the_right"), "(tuple a (ptuple b c))");
        assert_eq!(e(F, "nested_both_sides"), "(tuple (ptuple a b) (ptuple c d))");
        assert_eq!(e(F, "deeply_nested"), "(ptuple (ptuple a b) (ptuple c (ptuple d e)))");
    }

    #[test]
    fn elements_may_be_expressions() {
        assert_eq!(e(F, "tuple_of_expressions"), "(tuple (+ 1 2) (* 3 4))");
        assert_eq!(e(F, "tuple_of_literals"), "(tuple 1 str(two) True None)");
        assert_eq!(e(F, "tuple_with_comparison"), "(tuple (compare a < b) c)");
    }

    #[test]
    fn a_longer_tuple_stays_flat() {
        // Commas are one flat sequence, not nested pairs.
        assert_eq!(e(F, "bare_triple"), "(tuple a b c)");
        assert_eq!(e(F, "bare_four"), "(tuple a b c d)");
    }

    #[test]
    fn a_trailing_comma_makes_a_one_element_tuple() {
        assert_eq!(e(F, "trailing_comma"), "(tuple a)");
        assert_eq!(e(F, "single_element_tuple"), "(ptuple a)");
    }

    #[test]
    fn the_empty_tuple() {
        assert_eq!(e(F, "empty_tuple"), "(ptuple )");
    }

    #[test]
    fn redundant_parentheses_do_not_nest() {
        assert_eq!(e(F, "redundant_parentheses"), "(ptuple a b)");
    }

    #[test]
    fn malformed_tuples_are_rejected() {
        assert_rejected(F, "leading_comma");
        assert_rejected(F, "double_comma");
    }
}


mod assignment {
    use super::*;
    const F: &str = "parser/assignment.py";

    fn s(name: &str) -> String {
        let code = case(F, name);
        match one_stmt(&code) {
            Ok(rendered) => rendered,
            Err(err) => format!("<rejected: {err:?}>"),
        }
    }

    #[test]
    fn simple_assignment() {
        assert_eq!(s("simple"), "(assign (x) 1)");
        assert_eq!(s("name_value"), "(assign (x) y)");
        assert_eq!(s("expression_value"), "(assign (x) (+ 1 2))");
    }

    #[test]
    fn tuple_unpacking_spreads_the_targets() {
        assert_eq!(s("tuple_unpack"), "(assign (a b) (tuple 1 2))");
        assert_eq!(s("parenthesized_target"), "(assign (a b) (tuple 1 2))");
        assert_eq!(s("swap"), "(assign (a b) (tuple b a))");
    }

    #[test]
    fn a_tuple_value_is_kept_whole() {
        assert_eq!(s("value_is_a_tuple"), "(assign (x) (tuple 1 2))");
    }

    #[test]
    fn nested_unpacking() {
        assert_eq!(s("nested_unpack"), "(assign ((ptuple a b) c) (tuple (ptuple 1 2) 3))");
    }

    #[test]
    fn assignment_targets_are_stores_not_loads() {
        let module = parse_module(&case(F, "simple")).expect("x = 1 must parse");
        let PyStatement::Assign { targets, .. } = &module.body[0] else {
            panic!("expected an assignment, got {:?}", module.body[0]);
        };
        let PyExpr::Name { ctx, .. } = &targets[0] else {
            panic!("expected a name target, got {:?}", targets[0]);
        };
        assert!(
            matches!(ctx, PyRefContext::Store),
            "an assignment target is a Store, got {ctx:?}"
        );
    }

    #[test]
    fn chained_assignment() {
        assert_eq!(s("chained"), "(assign (a b) 1)");
        assert_eq!(s("chained_three"), "(assign (a b c) 1)");
    }

    #[test]
    fn augmented_assignment() {
        assert_eq!(s("augmented_add"), "(augassign x += 1)");
        assert_eq!(s("augmented_mul"), "(augassign x *= 2)");
    }

    #[test]
    fn annotated_assignment() {
        assert_eq!(s("annotated"), "(annassign x int 1)");
        assert_eq!(s("annotation_only"), "(annassign x int -)");
    }

    #[test]
    fn attribute_and_subscript_targets() {
        assert_eq!(s("attribute_target"), "(assign ((attr obj field)) 1)");
        assert_eq!(s("subscript_target"), "(assign ((subscript items 0)) 1)");
    }

    #[test]
    fn a_starred_target() {
        assert_eq!(s("starred_target"), "(assign (a (star rest)) items)");
    }

    #[test]
    fn the_walrus_operator() {
        assert_eq!(s("walrus"), "(:= n 10)");
    }

    #[test]
    fn an_equality_test_is_not_an_assignment() {
        assert_eq!(s("equality_is_not_assignment"), "(compare x == 1)");
    }

    #[test]
    fn an_assignment_cannot_span_a_newline() {
        // `x` and `= 1` are two lines: Python reports a syntax error rather
        // than joining them into one assignment.
        assert_rejected(F, "assignment_split_across_lines");
    }

    #[test]
    fn malformed_assignments_are_rejected() {
        assert_rejected(F, "missing_right_hand_side");
        assert_rejected(F, "missing_left_hand_side");
        assert_rejected(F, "assign_to_literal");
    }
}


mod calls {
    use super::*;
    const F: &str = "parser/calls.py";

    #[test]
    fn calls() {
        assert_eq!(e(F, "no_arguments"), "(call f)");
        assert_eq!(e(F, "one_argument"), "(call f 1)");
        assert_eq!(e(F, "several_arguments"), "(call f 1 2 3)");
        assert_eq!(e(F, "expression_argument"), "(call f (+ 1 2))");
        assert_eq!(e(F, "nested_call"), "(call f (call g 1))");
    }

    #[test]
    fn a_tuple_argument_is_one_argument() {
        assert_eq!(e(F, "tuple_argument"), "(call f (ptuple 1 2))");
    }

    #[test]
    fn keyword_arguments() {
        assert_eq!(e(F, "keyword_argument"), "(call f a=1)");
        assert_eq!(e(F, "mixed_arguments"), "(call f 1 b=2)");
    }

    #[test]
    fn attribute_access_chains_left_to_right() {
        assert_eq!(e(F, "attribute"), "(attr a b)");
        assert_eq!(e(F, "chained_attribute"), "(attr (attr a b) c)");
    }

    #[test]
    fn calls_and_attributes_compose() {
        assert_eq!(e(F, "method_call"), "(call (attr a b))");
        assert_eq!(e(F, "call_then_attribute"), "(attr (call f) g)");
        assert_eq!(e(F, "call_result_called"), "(call (call f))");
        assert_eq!(e(F, "attribute_on_call_result_called"), "(call (attr (call f) g))");
    }

    #[test]
    fn subscripts() {
        assert_eq!(e(F, "subscript"), "(subscript a 0)");
        assert_eq!(e(F, "chained_subscript"), "(subscript (subscript a 0) 1)");
        assert_eq!(e(F, "subscript_with_expression"), "(subscript a (+ i 1))");
        assert_eq!(e(F, "subscript_of_call"), "(subscript (call f) 0)");
        assert_eq!(e(F, "call_on_subscript"), "(call (subscript a 0))");
    }

    #[test]
    fn slices() {
        assert_eq!(e(F, "slice"), "(subscript a (slice 1 2 -))");
        assert_eq!(e(F, "slice_with_step"), "(subscript a (slice 1 2 3))");
        assert_eq!(e(F, "open_slice"), "(subscript a (slice - - -))");
    }

    #[test]
    fn argument_unpacking() {
        assert_eq!(e(F, "star_args"), "(call f (star args))");
        assert_eq!(e(F, "double_star_kwargs"), "(call f **kwargs)");
    }

    #[test]
    fn a_trailing_comma_in_a_call_is_allowed() {
        assert_eq!(e(F, "trailing_comma_in_call"), "(call f 1)");
    }

    #[test]
    fn malformed_calls_are_rejected() {
        assert_rejected(F, "unclosed_call");
        assert_rejected(F, "attribute_without_name");
    }
}


mod collections {
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
        assert!(e(F, "list_comprehension").starts_with("(<ListComp"));
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
}


mod if_statements {
    use super::*;
    const F: &str = "parser/if_statements.py";

    fn s(name: &str) -> String {
        let code = case(F, name);
        match parse_outcome(&code) {
            Outcome::Ok(body) => body.join(" "),
            Outcome::Err(err) => format!("<rejected: {err:?}>"),
            Outcome::Panic(m) => format!("<panic: {m}>"),
        }
    }

    #[test]
    fn a_simple_block() {
        assert_eq!(s("simple"), "(if a (b) ())");
    }

    #[test]
    fn an_else_branch() {
        assert_eq!(s("with_else"), "(if a (b) (c))");
    }

    #[test]
    fn nesting() {
        assert_eq!(s("nested"), "(if a ((if b (c) ())) ())");
        assert_eq!(s("nested_with_else"), "(if a ((if b (c) (d))) ())");
    }

    #[test]
    fn a_statement_after_the_block_is_a_sibling() {
        assert_eq!(s("statement_after_block"), "(if a (b) ()) c");
        assert_eq!(s("two_if_statements"), "(if a (b) ()) (if c (d) ())");
    }

    #[test]
    fn conditions_may_be_expressions() {
        assert_eq!(s("comparison_condition"), "(if (compare x < 10) ((assign (y) 1)) ())");
        assert_eq!(s("parenthesized_condition"), "(if a (b) ())");
    }

    #[test]
    fn a_body_may_hold_several_statements() {
        assert_eq!(s("multi_statement_body"), "(if a (b c) ())");
    }

    #[test]
    fn a_body_may_hold_assignments() {
        assert_eq!(s("assignment_in_body"), "(if a ((assign (x) 1)) ())");
        assert_eq!(s("two_assignments_in_body"), "(if a ((assign (x) 1) (assign (y) 2)) ())");
        assert_eq!(s("assignment_then_expression"), "(if a ((assign (x) 1) y) ())");
        assert_eq!(s("expression_then_assignment"), "(if a (y (assign (x) 1)) ())");
        assert_eq!(s("else_with_assignment"), "(if a ((assign (x) 1)) ((assign (x) 2)))");
    }

    #[test]
    fn a_nested_block_may_be_followed_by_a_sibling() {
        assert_eq!(s("nested_then_sibling_statement"), "(if a ((if b (c) ()) d) ())");
    }

    #[test]
    fn a_boolean_condition() {
        assert_eq!(s("boolean_condition"), "(if (and a b) (c) ())");
    }

    #[test]
    fn elif_is_an_else_holding_an_if() {
        assert_eq!(s("elif"), "(if a (b) ((if c (d) ())))");
        assert_eq!(s("elif_else"), "(if a (b) ((if c (d) (e))))");
        assert_eq!(s("two_elifs"), "(if a (b) ((if c (d) ((if e (f) ())))))");
    }

    #[test]
    fn a_blank_line_before_else_is_allowed() {
        assert_eq!(s("blank_line_before_else"), "(if a (b) (c))");
    }

    #[test]
    fn an_inline_body_is_allowed() {
        assert_eq!(s("inline_body"), "(if a (b) ())");
        assert_eq!(s("inline_body_with_else"), "(if a (b) (c))");
    }

    #[test]
    fn a_tuple_condition_is_still_a_condition() {
        assert_eq!(s("tuple_condition"), "(if (tuple a b) (c) ())");
    }

    #[test]
    fn malformed_if_statements_are_rejected() {
        for name in ["missing_colon", "missing_body", "missing_condition", "else_without_if"] {
            assert_rejected(F, name);
        }
    }
}


mod statements {
    use super::*;
    const F: &str = "parser/statements.py";

    #[test]
    fn simple_statements() {
        assert_eq!(one_stmt(&case(F, "pass")), Ok("pass".into()));
        assert_eq!(one_stmt(&case(F, "break")), Ok("break".into()));
        assert_eq!(one_stmt(&case(F, "continue")), Ok("continue".into()));
    }

    #[test]
    fn loops() {
        assert_eq!(one_stmt(&case(F, "while_loop")), Ok("(while a (b) ())".into()));
        assert_eq!(one_stmt(&case(F, "for_loop")), Ok("(for i items ((call print i)))".into()));
    }

    #[test]
    fn function_definitions() {
        assert_eq!(one_stmt(&case(F, "function_no_arguments")), Ok("(def f (pass))".into()));
        assert_eq!(one_stmt(&case(F, "function_definition")), Ok("(def f ((return a)))".into()));
    }

    #[test]
    fn class_definitions() {
        assert_eq!(one_stmt(&case(F, "class_definition")), Ok("(class C (pass))".into()));
    }

    #[test]
    fn statements_separated_by_semicolons() {
        assert_eq!(
            stmts(&case(F, "semicolon_separated")),
            Ok(vec!["(assign (a) 1)".into(), "(assign (b) 2)".into()])
        );
    }

    #[test]
    fn every_statement_form_parses() {
        let failures = parse_failures(F);
        assert!(
            failures.is_empty(),
            "{} of {} statement forms fail to parse:\n{}",
            failures.len(),
            cases(F).len(),
            failures.join("\n")
        );
    }
}


mod modules {
    use super::*;
    const F: &str = "parser/modules.py";

    fn s(name: &str) -> Vec<String> {
        stmts(&case(F, name)).unwrap_or_else(|e| vec![format!("<rejected: {e:?}>")])
    }

    #[test]
    fn consecutive_statements() {
        assert_eq!(s("single_statement"), vec!["a"]);
        assert_eq!(s("two_statements"), vec!["a", "b"]);
        assert_eq!(s("three_statements"), vec!["a", "b", "c"]);
    }

    #[test]
    fn blank_lines_are_not_statements() {
        assert_eq!(s("statements_with_blank_lines"), vec!["a", "b"]);
        assert_eq!(s("consecutive_blank_lines"), vec!["a", "b"]);
    }

    #[test]
    fn assignments_and_expressions_mix() {
        assert_eq!(s("two_assignments"), vec!["(assign (x) 1)", "(assign (y) 2)"]);
        assert_eq!(s("assignment_then_expression"), vec!["(assign (x) 1)", "x"]);
        assert_eq!(s("expression_then_assignment"), vec!["x", "(assign (x) 1)"]);
    }

    #[test]
    fn blocks_and_statements_mix() {
        assert_eq!(s("statement_block_statement"), vec!["a", "(if b (c) ())", "d"]);
        assert_eq!(s("block_then_block"), vec!["(if a (b) ())", "(if c (d) ())"]);
        assert_eq!(s("block_last"), vec!["a", "(if b (c) ())"]);
    }

    #[test]
    fn a_longer_module() {
        assert_eq!(s("many_statements").len(), 5);
    }

    #[test]
    fn a_module_without_a_trailing_newline() {
        assert_eq!(stmts("a = 1"), Ok(vec!["(assign (a) 1)".into()]));
    }
}


mod robustness {
    use super::*;

    #[test]
    fn a_malformed_float_is_rejected() {
        assert!(matches!(parse_outcome("1.2.3\n"), Outcome::Err(_)));
    }

    #[test]
    fn an_identifier_cannot_start_with_a_digit() {
        assert!(matches!(parse_outcome("2fast\n"), Outcome::Err(_)));
    }

    #[test]
    fn deep_nesting_does_not_blow_the_stack() {
        let code = format!("{}1{}\n", "(".repeat(200), ")".repeat(200));
        assert!(!matches!(parse_outcome(&code), Outcome::Panic(_)));
    }

    #[test]
    fn a_stray_layout_token_is_never_a_statement() {
        // Whatever the lexer emits, the parser must not accept a bare Dedent or
        // Newline as an expression.
        assert!(matches!(parse_outcome("if a:\n    b\n  c\n"), Outcome::Err(_)));
    }

    #[test]
    fn nothing_in_the_fixtures_makes_the_parser_panic() {
        let mut panics = Vec::new();
        for fixture in [
            "parser/literals.py",
            "parser/arithmetic.py",
            "parser/bitwise.py",
            "parser/comparisons.py",
            "parser/boolean.py",
            "parser/tuples.py",
            "parser/assignment.py",
            "parser/calls.py",
            "parser/collections.py",
            "parser/if_statements.py",
            "parser/statements.py",
            "parser/modules.py",
        ] {
            for c in cases(fixture) {
                if let Outcome::Panic(m) = parse_outcome(&c.code) {
                    panics.push(format!("  {fixture} :: {}: {m}", c.name));
                }
            }
        }
        assert!(
            panics.is_empty(),
            "invalid input must produce errors, never panics:\n{}",
            panics.join("\n")
        );
    }
}
