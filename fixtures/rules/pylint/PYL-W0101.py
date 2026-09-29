# case: first_statement_is_return

def func(x):
    return 5
    x += 2  # !
    some_value = 5 # !
    print(some_value) # !

# case: first_statement_is_raise

def func(x):
    raise Exception
    print(20) # !
    x = a + 2 # !
