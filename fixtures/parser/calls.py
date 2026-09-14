# case: no_arguments
f()

# case: one_argument
f(1)

# case: several_arguments
f(1, 2, 3)

# case: expression_argument
f(1 + 2)

# case: tuple_argument
f((1, 2))

# case: keyword_argument
f(a=1)

# case: mixed_arguments
f(1, b=2)

# case: nested_call
f(g(1))

# case: call_result_called
f()()

# case: attribute
a.b

# case: chained_attribute
a.b.c

# case: method_call
a.b()

# case: call_then_attribute
f().g

# case: attribute_on_call_result_called
f().g()

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

# case: star_args
f(*args)

# case: double_star_kwargs
f(**kwargs)

# case: trailing_comma_in_call
f(1,)

# case: unclosed_call
f(1

# case: attribute_without_name
a.
