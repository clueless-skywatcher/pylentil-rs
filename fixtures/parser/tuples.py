# case: bare_pair
a, b

# case: bare_triple
a, b, c

# case: bare_four
a, b, c, d

# case: trailing_comma
a,

# case: parenthesized_pair
(a, b)

# case: parenthesized_single_is_not_a_tuple
(a)

# case: single_element_tuple
(a,)

# case: empty_tuple
()

# case: nested_on_the_left
(a, b), c

# case: nested_on_the_right
a, (b, c)

# case: nested_both_sides
(a, b), (c, d)

# case: deeply_nested
((a, b), (c, (d, e)))

# case: tuple_of_expressions
1 + 2, 3 * 4

# case: tuple_of_literals
1, "two", True, None

# case: tuple_with_comparison
a < b, c

# case: redundant_parentheses
((a, b))

# case: leading_comma
, a

# case: double_comma
a,, b
