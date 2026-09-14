# case: addition
1 + 2

# case: subtraction_is_left_associative
1 - 2 - 3

# case: division_is_left_associative
8 / 4 / 2

# case: multiplication_binds_tighter_than_addition
1 + 2 * 3

# case: multiplication_on_the_left
2 * 3 + 1

# case: parentheses_override_precedence
(1 + 2) * 3

# case: power_is_right_associative
2 ** 3 ** 2

# case: power_binds_tighter_than_multiplication
2 * 3 ** 2

# case: power_on_the_left
2 ** 3 * 4

# case: modulo_and_addition
7 % 3 + 1

# case: modulo_and_multiplication
2 * 7 % 3

# case: unary_minus
-1

# case: unary_minus_on_name
-x

# case: unary_plus
+1

# case: double_unary_minus
- -1

# case: subtract_a_negative
1 - -2

# case: unary_minus_binds_looser_than_power
-2 ** 2

# case: unary_minus_binds_tighter_than_multiplication
-2 * 3

# case: floor_division
7 // 2

# case: shift_binds_looser_than_addition
1 << 2 + 3

# case: shift_is_left_associative
1 << 2 << 3

# case: long_chain
1 + 2 * 3 - 4 / 5

# case: nested_parentheses
((1 + 2) * (3 - 4))

# case: missing_right_operand
1 +

# case: missing_left_operand
* 2

# case: unbalanced_open_paren
(1 + 2

# case: unbalanced_close_paren
1 + 2)
