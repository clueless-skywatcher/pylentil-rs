use super::*;
const F: &str = "parser/bitwise.py";

#[test]
fn parse_bitwise_bitand_binds_tighter_than_bitor() {
    assert_eq!(e(F, "bitand_binds_tighter_than_bitor"), "(| a (& b c))");
}

#[test]
fn parse_bitwise_bitxor_binds_tighter_than_bitor() {
    assert_eq!(e(F, "bitxor_binds_tighter_than_bitor"), "(| a (^ b c))");
}

#[test]
fn parse_bitwise_bitand_binds_tighter_than_bitxor() {
    assert_eq!(e(F, "bitand_binds_tighter_than_bitxor"), "(^ a (& b c))");
}

#[test]
fn parse_bitwise_bitor_is_left_associative() {
    assert_eq!(e(F, "bitor_is_left_associative"), "(| (| a b) c)");
}

#[test]
fn parse_bitwise_shift_binds_tighter_than_bitand() {
    assert_eq!(e(F, "shift_binds_tighter_than_bitand"), "(& a (<< b c))");
}

#[test]
fn parse_bitwise_arithmetic_binds_tighter_than_bitand() {
    assert_eq!(e(F, "arithmetic_binds_tighter_than_bitand"), "(& a (+ b c))");
}

#[test]
fn parse_bitwise_comparison_binds_looser_than_bitand() {
    // (a & b) == c, not a & (b == c)
    assert_eq!(e(F, "comparison_binds_looser_than_bitand"), "(compare (& a b) == c)");
}

#[test]
fn parse_bitwise_comparison_binds_looser_than_bitor() {
    assert_eq!(e(F, "comparison_binds_looser_than_bitor"), "(compare (| a b) == c)");
}

#[test]
fn parse_bitwise_invert() {
    assert_eq!(e(F, "invert"), "(~ a)");
}

#[test]
fn parse_bitwise_invert_binds_tighter_than_bitand() {
    assert_eq!(e(F, "invert_binds_tighter_than_bitand"), "(& (~ a) b)");
}

#[test]
fn parse_bitwise_all_three_bitwise() {
    assert_eq!(e(F, "all_three_bitwise"), "(| a (^ b (& c d)))");
}
