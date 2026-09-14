# case: equal
a == b

# case: not_equal
a != b

# case: less_than
a < b

# case: greater_or_equal
a >= b

# case: chained
a < b < c

# case: chained_mixed_operators
a < b == c

# case: chained_three_operators
a < b <= c < d

# case: comparison_binds_looser_than_addition
a + 1 == b

# case: comparison_on_both_sides
a + 1 == b * 2

# case: chained_with_arithmetic
1 < x + 1 < 10

# case: is_operator
a is b

# case: is_not_operator
a is not b

# case: in_operator
a in b

# case: not_in_operator
a not in b

# case: not_operator
not a

# case: not_with_comparison
not a == b

# case: parenthesized_comparison
(a < b) < c

# case: missing_right_operand
a ==

# case: double_operator
a == == b
