# case: and_expression
a and b

# case: or_expression
a or b

# case: and_binds_tighter_than_or
a or b and c

# case: and_on_the_left
a and b or c

# case: not_binds_tighter_than_and
not a and b

# case: chained_and
a and b and c

# case: chained_or
a or b or c

# case: comparison_binds_tighter_than_and
a < b and c < d

# case: parentheses_override_precedence
(a or b) and c

# case: double_not
not not a

# case: not_with_parentheses
not (a and b)

# case: ternary
a if b else c

# case: ternary_nested
a if b else c if d else e

# case: ternary_with_arithmetic
1 + 2 if c else 3

# case: and_missing_operand
a and
