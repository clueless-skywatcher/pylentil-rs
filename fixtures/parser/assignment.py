# case: simple
x = 1

# case: expression_value
x = 1 + 2

# case: name_value
x = y

# case: tuple_unpack
a, b = 1, 2

# case: parenthesized_target
(a, b) = 1, 2

# case: nested_unpack
(a, b), c = (1, 2), 3

# case: swap
a, b = b, a

# case: chained
a = b = 1

# case: chained_three
a = b = c = 1

# case: augmented_add
x += 1

# case: augmented_mul
x *= 2

# case: annotated
x: int = 1

# case: annotation_only
x: int

# case: attribute_target
obj.field = 1

# case: subscript_target
items[0] = 1

# case: starred_target
a, *rest = items

# case: walrus
(n := 10)

# case: value_is_a_tuple
x = 1, 2

# case: assignment_split_across_lines
x
= 1

# case: missing_right_hand_side
x =

# case: missing_left_hand_side
= 1

# case: assign_to_literal
1 = x

# case: equality_is_not_assignment
x == 1
