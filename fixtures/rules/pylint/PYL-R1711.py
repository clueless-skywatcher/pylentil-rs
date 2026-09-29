# case: two_statements
def a():
    a = 2
    return  # !

# case: three_statements
def x():
    a = 2
    b = a + 1
    return  # !

# case: returning_none
def a():
    a = 1
    return None  # !

# case: with_arguments
def f(a, b):
    c = a + b
    return  # !

# case: with_default_argument
def f(a=1):
    b = a
    return None  # !

# case: with_annotations
def f(a: int):
    b = a
    return None  # !

# case: after_if_statement
def f(a):
    if a:
        b = 1
    return  # !

# case: after_if_else_statement
def f(a):
    if a:
        b = 1
    else:
        b = 2
    return None  # !

# case: after_try_statement
def f():
    try:
        x
    except ValueError:
        y
    return  # !

# case: after_call
def f():
    print(1)
    return  # !

# case: nested_function
def outer():
    def inner():
        a = 1
        return  # !
    return inner

# case: outer_and_nested_function
def outer():
    def inner():
        a = 1
        return None  # !
    b = inner
    return  # !

# case: function_in_if_statement
if a:
    def f():
        b = 1
        return  # !

# case: function_in_else
if a:
    pass
else:
    def f():
        b = 1
        return None  # !

# case: function_in_try
try:
    def f():
        b = 1
        return  # !
except ValueError:
    pass

# case: function_in_except_handler
try:
    x
except ValueError:
    def f():
        b = 1
        return  # !

# case: only_return_not_flagged
def f():
    return

# case: only_return_none_not_flagged
def f():
    return None

# case: returning_value_not_flagged
def f():
    a = 1
    return a

# case: returning_constant_not_flagged
def f():
    a = 1
    return 0

# case: returning_false_not_flagged
def f():
    a = 1
    return False

# case: returning_call_not_flagged
def f():
    a = 1
    return g(a)

# case: return_not_last_statement_not_flagged
def f():
    return
    a = 1

# case: return_inside_if_not_flagged
def f(a):
    b = 1
    if a:
        return

# case: return_none_inside_else_not_flagged
def f(a):
    b = 1
    if a:
        pass
    else:
        return None

# case: no_return_not_flagged
def f():
    a = 1
    b = 2

# case: ending_with_pass_not_flagged
def f():
    a = 1
    pass
