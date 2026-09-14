# case: bitand_binds_tighter_than_bitor
a | b & c

# case: bitxor_binds_tighter_than_bitor
a | b ^ c

# case: bitand_binds_tighter_than_bitxor
a ^ b & c

# case: bitor_is_left_associative
a | b | c

# case: shift_binds_tighter_than_bitand
a & b << c

# case: arithmetic_binds_tighter_than_bitand
a & b + c

# case: comparison_binds_looser_than_bitand
a & b == c

# case: comparison_binds_looser_than_bitor
a | b == c

# case: invert
~a

# case: invert_binds_tighter_than_bitand
~a & b

# case: all_three_bitwise
a | b ^ c & d
