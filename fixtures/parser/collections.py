# case: empty_list
[]

# case: list_of_literals
[1, 2, 3]

# case: list_of_expressions
[1 + 2, 3 * 4]

# case: nested_list
[[1], [2]]

# case: list_trailing_comma
[1, 2,]

# case: single_element_list
[1]

# case: empty_dict
{}

# case: dict_one_entry
{"a": 1}

# case: dict_several_entries
{"a": 1, "b": 2}

# case: nested_dict
{"a": {"b": 1}}

# case: dict_trailing_comma
{"a": 1,}

# case: set_literal
{1, 2}

# case: list_comprehension
[x for x in items]

# case: list_comprehension_with_condition
[x for x in items if x]

# case: dict_comprehension
{k: v for k, v in pairs}

# case: set_comprehension
{x for x in items}

# case: generator_expression
(x for x in items)

# case: list_containing_tuple
[(1, 2)]

# case: tuple_containing_list
([1], [2])

# case: unclosed_list
[1, 2

# case: mismatched_brackets
[1, 2)
