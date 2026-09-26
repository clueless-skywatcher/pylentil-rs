use super::*;
const F: &str = "parser/arithmetic.py";

#[test]
fn parse_arithmetic_addition() {
    assert_eq!(expr(F, "addition"), bin_op(int(1), PyBinaryOp::Add, int(2)));
}

#[test]
fn parse_arithmetic_subtraction_is_left_associative() {
    assert_eq!(
        expr(F, "subtraction_is_left_associative"),
        bin_op(
            bin_op(int(1), PyBinaryOp::Sub, int(2)),
            PyBinaryOp::Sub,
            int(3)
        )
    );
}

#[test]
fn parse_arithmetic_division_is_left_associative() {
    assert_eq!(
        expr(F, "division_is_left_associative"),
        bin_op(
            bin_op(int(8), PyBinaryOp::Div, int(4)),
            PyBinaryOp::Div,
            int(2)
        )
    );
}

#[test]
fn parse_arithmetic_multiplication_binds_tighter_than_addition() {
    assert_eq!(
        expr(F, "multiplication_binds_tighter_than_addition"),
        bin_op(
            int(1),
            PyBinaryOp::Add,
            bin_op(int(2), PyBinaryOp::Mul, int(3))
        )
    );
}

#[test]
fn parse_arithmetic_multiplication_on_the_left() {
    assert_eq!(
        expr(F, "multiplication_on_the_left"),
        bin_op(
            bin_op(int(2), PyBinaryOp::Mul, int(3)),
            PyBinaryOp::Add,
            int(1)
        )
    );
}

#[test]
fn parse_arithmetic_parentheses_override_precedence() {
    assert_eq!(
        expr(F, "parentheses_override_precedence"),
        bin_op(
            bin_op(int(1), PyBinaryOp::Add, int(2)),
            PyBinaryOp::Mul,
            int(3)
        )
    );
}

#[test]
fn parse_arithmetic_power_is_right_associative() {
    assert_eq!(
        expr(F, "power_is_right_associative"),
        bin_op(
            int(2),
            PyBinaryOp::Pow,
            bin_op(int(3), PyBinaryOp::Pow, int(2))
        )
    );
}

#[test]
fn parse_arithmetic_power_binds_tighter_than_multiplication() {
    assert_eq!(
        expr(F, "power_binds_tighter_than_multiplication"),
        bin_op(
            int(2),
            PyBinaryOp::Mul,
            bin_op(int(3), PyBinaryOp::Pow, int(2))
        )
    );
}

#[test]
fn parse_arithmetic_power_on_the_left() {
    assert_eq!(
        expr(F, "power_on_the_left"),
        bin_op(
            bin_op(int(2), PyBinaryOp::Pow, int(3)),
            PyBinaryOp::Mul,
            int(4)
        )
    );
}

#[test]
fn parse_arithmetic_modulo_and_addition() {
    assert_eq!(
        expr(F, "modulo_and_addition"),
        bin_op(
            bin_op(int(7), PyBinaryOp::Mod, int(3)),
            PyBinaryOp::Add,
            int(1)
        )
    );
}

#[test]
fn parse_arithmetic_modulo_and_multiplication() {
    assert_eq!(
        expr(F, "modulo_and_multiplication"),
        bin_op(
            bin_op(int(2), PyBinaryOp::Mul, int(7)),
            PyBinaryOp::Mod,
            int(3)
        )
    );
}

#[test]
fn parse_arithmetic_unary_minus() {
    assert_eq!(
        expr(F, "unary_minus"),
        unary_op(PyUnaryOp::UnarySub, int(1))
    );
}

#[test]
fn parse_arithmetic_unary_minus_on_name() {
    assert_eq!(
        expr(F, "unary_minus_on_name"),
        unary_op(PyUnaryOp::UnarySub, name("x"))
    );
}

#[test]
fn parse_arithmetic_unary_plus() {
    assert_eq!(expr(F, "unary_plus"), unary_op(PyUnaryOp::UnaryAdd, int(1)));
}

#[test]
fn parse_arithmetic_double_unary_minus() {
    assert_eq!(
        expr(F, "double_unary_minus"),
        unary_op(PyUnaryOp::UnarySub, unary_op(PyUnaryOp::UnarySub, int(1)))
    );
}

#[test]
fn parse_arithmetic_subtract_a_negative() {
    assert_eq!(
        expr(F, "subtract_a_negative"),
        bin_op(
            int(1),
            PyBinaryOp::Sub,
            unary_op(PyUnaryOp::UnarySub, int(2))
        )
    );
}

#[test]
fn parse_arithmetic_unary_minus_binds_looser_than_power() {
    // -2 ** 2 is -4, not 4.
    assert_eq!(
        expr(F, "unary_minus_binds_looser_than_power"),
        unary_op(PyUnaryOp::UnarySub, bin_op(int(2), PyBinaryOp::Pow, int(2)))
    );
}

#[test]
fn parse_arithmetic_unary_minus_binds_tighter_than_multiplication() {
    assert_eq!(
        expr(F, "unary_minus_binds_tighter_than_multiplication"),
        bin_op(
            unary_op(PyUnaryOp::UnarySub, int(2)),
            PyBinaryOp::Mul,
            int(3)
        )
    );
}

#[test]
fn parse_arithmetic_floor_division() {
    assert_eq!(
        expr(F, "floor_division"),
        bin_op(int(7), PyBinaryOp::FloorDiv, int(2))
    );
}

#[test]
fn parse_arithmetic_shift_binds_looser_than_addition() {
    assert_eq!(
        expr(F, "shift_binds_looser_than_addition"),
        bin_op(
            int(1),
            PyBinaryOp::LShift,
            bin_op(int(2), PyBinaryOp::Add, int(3))
        )
    );
}

#[test]
fn parse_arithmetic_shift_is_left_associative() {
    assert_eq!(
        expr(F, "shift_is_left_associative"),
        bin_op(
            bin_op(int(1), PyBinaryOp::LShift, int(2)),
            PyBinaryOp::LShift,
            int(3)
        )
    );
}

#[test]
fn parse_arithmetic_long_chain() {
    assert_eq!(
        expr(F, "long_chain"),
        bin_op(
            bin_op(
                int(1),
                PyBinaryOp::Add,
                bin_op(int(2), PyBinaryOp::Mul, int(3))
            ),
            PyBinaryOp::Sub,
            bin_op(int(4), PyBinaryOp::Div, int(5))
        )
    );
}

#[test]
fn parse_arithmetic_nested_parentheses() {
    assert_eq!(
        expr(F, "nested_parentheses"),
        bin_op(
            bin_op(int(1), PyBinaryOp::Add, int(2)),
            PyBinaryOp::Mul,
            bin_op(int(3), PyBinaryOp::Sub, int(4))
        )
    );
}

#[test]
fn parse_arithmetic_missing_right_operand() {
    assert_rejected(F, "missing_right_operand");
}

#[test]
fn parse_arithmetic_missing_left_operand() {
    assert_rejected(F, "missing_left_operand");
}

#[test]
fn parse_arithmetic_unbalanced_open_paren() {
    assert_rejected(F, "unbalanced_open_paren");
}

#[test]
fn parse_arithmetic_unbalanced_close_paren() {
    assert_rejected(F, "unbalanced_close_paren");
}
