# case: raise_with_or
def f():
    try:
        a = 1
    except X or Y: # ! (12, 18)
        b = 2

# case: raise_with_and
def f():
    try:
        a = 1
    except X and Y: # ! (12, 19)
        b = 2