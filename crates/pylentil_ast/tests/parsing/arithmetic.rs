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
