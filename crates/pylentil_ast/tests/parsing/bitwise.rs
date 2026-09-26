use super::*;
const F: &str = "parser/bitwise.py";

#[test]
fn parse_bitwise_bitand_binds_tighter_than_bitor() {
    assert_eq!(
        expr(F, "bitand_binds_tighter_than_bitor"),
        bin_op(
            name("a"),
            PyBinaryOp::BitOr,
            bin_op(name("b"), PyBinaryOp::BitAnd, name("c"))
        )
    );
}

#[test]
fn parse_bitwise_bitxor_binds_tighter_than_bitor() {
    assert_eq!(
        expr(F, "bitxor_binds_tighter_than_bitor"),
        bin_op(
            name("a"),
            PyBinaryOp::BitOr,
            bin_op(name("b"), PyBinaryOp::BitXor, name("c"))
        )
    );
}

#[test]
fn parse_bitwise_bitand_binds_tighter_than_bitxor() {
    assert_eq!(
        expr(F, "bitand_binds_tighter_than_bitxor"),
        bin_op(
            name("a"),
            PyBinaryOp::BitXor,
            bin_op(name("b"), PyBinaryOp::BitAnd, name("c"))
        )
    );
}

#[test]
fn parse_bitwise_bitor_is_left_associative() {
    assert_eq!(
        expr(F, "bitor_is_left_associative"),
        bin_op(
            bin_op(name("a"), PyBinaryOp::BitOr, name("b")),
            PyBinaryOp::BitOr,
            name("c")
        )
    );
}

#[test]
fn parse_bitwise_shift_binds_tighter_than_bitand() {
    assert_eq!(
        expr(F, "shift_binds_tighter_than_bitand"),
        bin_op(
            name("a"),
            PyBinaryOp::BitAnd,
            bin_op(name("b"), PyBinaryOp::LShift, name("c"))
        )
    );
}

#[test]
fn parse_bitwise_arithmetic_binds_tighter_than_bitand() {
    assert_eq!(
        expr(F, "arithmetic_binds_tighter_than_bitand"),
        bin_op(
            name("a"),
            PyBinaryOp::BitAnd,
            bin_op(name("b"), PyBinaryOp::Add, name("c"))
        )
    );
}

#[test]
fn parse_bitwise_comparison_binds_looser_than_bitand() {
    // (a & b) == c, not a & (b == c)
    assert_eq!(
        expr(F, "comparison_binds_looser_than_bitand"),
        compare(
            bin_op(name("a"), PyBinaryOp::BitAnd, name("b")),
            vec![PyComparisonOp::Eq],
            vec![name("c")]
        )
    );
}

#[test]
fn parse_bitwise_comparison_binds_looser_than_bitor() {
    assert_eq!(
        expr(F, "comparison_binds_looser_than_bitor"),
        compare(
            bin_op(name("a"), PyBinaryOp::BitOr, name("b")),
            vec![PyComparisonOp::Eq],
            vec![name("c")]
        )
    );
}

#[test]
fn parse_bitwise_invert() {
    assert_eq!(expr(F, "invert"), unary_op(PyUnaryOp::Invert, name("a")));
}

#[test]
fn parse_bitwise_invert_binds_tighter_than_bitand() {
    assert_eq!(
        expr(F, "invert_binds_tighter_than_bitand"),
        bin_op(
            unary_op(PyUnaryOp::Invert, name("a")),
            PyBinaryOp::BitAnd,
            name("b")
        )
    );
}

#[test]
fn parse_bitwise_all_three_bitwise() {
    assert_eq!(
        expr(F, "all_three_bitwise"),
        bin_op(
            name("a"),
            PyBinaryOp::BitOr,
            bin_op(
                name("b"),
                PyBinaryOp::BitXor,
                bin_op(name("c"), PyBinaryOp::BitAnd, name("d"))
            )
        )
    );
}
