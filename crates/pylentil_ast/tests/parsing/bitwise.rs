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
