# case: equal_true
value = True
if value == True:  # ! (13, 17)
    pass

# case: equal_false
value = False
if value == False:  # ! (13, 18)
    pass

# case: not_equal_true
value = True
if value != True:  # ! (13, 17)
    pass

# case: not_equal_false
value = False
if value != False:  # ! (13, 18)
    pass

# case: assignment_comparison_true
value = True
result = value == True  # ! (19, 23)
result

# case: assignment_comparison_false
value = False
result = value == False  # ! (19, 24)
result

# case: return_comparison_true
value = True
value == True  # ! (10, 14)

# case: return_comparison_false
value = False
value == False  # ! (10, 15)

# case: assert_equal_true
value = True
assert value == True  # ! (17, 21)

# case: assert_equal_false
value = False
assert value == False  # ! (17, 22)

# case: assert_not_equal_true
value = True
assert value != True  # ! (17, 21)

# case: assert_not_equal_false
value = False
assert value != False  # ! (17, 22)

# case: parenthesized_comparison
value = True
if (value == True):  # ! (14, 18)
    pass

# case: chained_comparison
value = True
if value == True == True:  # ! (13, 17), (21, 25)
    pass

# case: boolean_expression
value = True
if value and value == True:  # ! (23, 27)
    pass

# case: boolean_expression_false
value = False
if value or value == False:  # ! (22, 27)
    pass

# case: comparison_in_list
values = [True, False]
result = [value == True for value in values]  # ! (20, 24)
result

# case: comparison_in_dict
value = True
result = {"valid": value == True}  # ! (29, 33)
result

# case: comparison_in_conditional_expression
value = True
result = "yes" if value == True else "no"  # ! (28, 32)
result

# case: comparison_in_generator
values = [True, False]
result = any(value == True for value in values)  # ! (23, 27)
result

# case: comparison_in_any
values = [True, False]
result = any(value != True for value in values)  # ! (23, 27)
result

# case: comparison_in_all
values = [True, False]
result = all(value == False for value in values)  # ! (23, 28)
result

# case: comparison_in_while
value = True
while value == True:  # ! (16, 20)
    break

# case: comparison_in_assertion_message
value = True
assert value == True, "expected true"  # ! (17, 21)

# case: comparison_with_none
value = None
result = value == True  # ! (19, 23)
result

# case: comparison_with_integer
value = 1
result = value == True  # ! (19, 23)
result

# case: comparison_with_call
result = bool(1) == True  # ! (21, 25)
result

# case: comparison_with_attribute
class Example:
    enabled = True

result = Example.enabled == True  # ! (29, 33)
result

# case: comparison_with_subscript
values = {"enabled": True}
result = values["enabled"] == True  # ! (31, 35)
result

# case: comparison_with_method_call
values = [True]
result = values.pop() == True  # ! (26, 30)
result

# case: comparison_in_return_if
value = False
if value != False:  # ! (13, 18)
    pass

# case: multiple_violations
first = True
second = False
if first == True or second == False:  # ! (13, 17), (31, 36)
    pass

# case: valid_is_true
value = True
if value is True:
    pass

# case: valid_is_false
value = False
if value is False:
    pass

# case: valid_not_is_true
value = True
if value is not True:
    pass

# case: valid_not_is_false
value = False
if value is not False:
    pass

# case: valid_equality_with_variable
value = True
expected = True
if value == expected:
    pass

# case: valid_inequality_with_variable
value = False
expected = True
if value != expected:
    pass

# case: valid_boolean_context
value = True
if value:
    pass

# case: valid_negated_boolean_context
value = False
if not value:
    pass

# case: valid_numeric_comparison
value = 1
if value == 1:
    pass

# case: valid_string_comparison
value = "True"
if value == "True":
    pass

# case: valid_comparison_with_none
value = None
if value is None:
    pass

# case: valid_comparison_with_falsey_value
value = 0
if value == 0:
    pass
