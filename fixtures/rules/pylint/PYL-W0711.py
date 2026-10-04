# case: raise_with_or
def f():
    try:
        a = 1
    except X or Y: # !
        b = 2

# case: raise_with_and
def f():
    try:
        a = 1
    except X and Y: # !
        b = 2