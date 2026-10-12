# case: first_statement_is_return

def func(x):
    return 5
    x += 2  # ! (5, 11)
    some_value = 5 # ! (5, 19)
    print(some_value) # ! (5, 22)

# case: first_statement_is_raise

def func(x):
    raise Exception
    print(20) # ! (5, 14)
    x = a + 2 # ! (5, 14)
