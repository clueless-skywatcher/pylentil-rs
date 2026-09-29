# case: bare_except
try:
    x
except: # !
    y

# case: bare_except_in_function
def a():
    try:
        x
    except: # !
        y

# case: bare_except_in_if_statement
if a == 2:
    try:
        x
    except: # !
        y

