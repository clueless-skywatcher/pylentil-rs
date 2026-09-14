# case: arithmetic
a + b - c * d / e % f

# case: power
a ** b

# case: floor_division
a // b

# case: matrix_multiply
a @ b

# case: left_shift
a << b

# case: right_shift
a >> b

# case: comparisons
a == b != c

# case: relational
a < b > c

# case: relational_or_equal
a <= b >= c

# case: greater_then_less_adjacent
a >< b

# case: less_then_greater_adjacent
a <> b

# case: bitwise
a & b | c ^ d

# case: invert
~a

# case: augmented_add
a += 1

# case: augmented_sub
a -= 1

# case: augmented_mul
a *= 2

# case: augmented_div
a /= 2

# case: augmented_floordiv
a //= 2

# case: augmented_pow
a **= 2

# case: augmented_mod
a %= 2

# case: augmented_bitand
a &= 1

# case: augmented_bitor
a |= 1

# case: augmented_bitxor
a ^= 1

# case: augmented_shift
a <<= 1

# case: walrus
n := 10

# case: arrow
def f() -> int:

# case: ellipsis
...

# case: decorator
@decorator

# case: semicolon_separated
a = 1; b = 2

# case: lone_bang
a ! b

# case: brackets
([{}])
