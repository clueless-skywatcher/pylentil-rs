# case: subscript
a[0]

# case: chained_subscript
a[0][1]

# case: subscript_with_expression
a[i + 1]

# case: slice
a[1:2]

# case: slice_with_step
a[1:2:3]

# case: open_slice
a[:]

# case: subscript_of_call
f()[0]

# case: call_on_subscript
a[0]()

# case: slice_lower_only
a[1:]

# case: slice_upper_only
a[:2]

# case: slice_trailing_colon
a[1:2:]

# case: open_slice_both_colons
a[::]

# case: slice_step_only
a[::2]

# case: slice_omitted_upper_with_step
a[1::2]

# case: slice_omitted_lower_with_step
a[:2:3]

# case: slice_bounds_are_expressions
a[x + 1:y * 2]

# case: slice_bounds_are_calls
a[f(1):g(2)]

# case: slice_bound_is_a_subscript
a[b[0]:c]

# case: tuple_index
a[1, 2]

# case: empty_subscript
a[]

# case: too_many_slice_parts
a[1:2:3:4]
