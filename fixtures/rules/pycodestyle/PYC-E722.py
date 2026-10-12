# case: bare_except
try:
    x
except: # ! (1, 5:6)
    y

# case: bare_except_in_function
def a():
    try:
        x
    except: # ! (5, 12:10)
        y

# case: bare_except_in_if_statement
if a == 2:
    try:
        x
    except: # ! (5, 19:10)
        y

# case: bare_except_in_elif
if a == 1:
    pass
elif a == 2:
    try:
        x
    except: # ! (5, 28:10)
        y

# case: bare_except_in_else
if a == 1:
    pass
else:
    try:
        x
    except: # ! (5, 37:10)
        y

# case: bare_except_after_typed_handler
try:
    x
except ValueError:
    y
except: # ! (1, 45:6)
    z

# case: bare_except_after_multiple_typed_handlers
try:
    x
except ValueError:
    y
except KeyError as e:
    y
except: # ! (1, 55:6)
    z

# case: bare_except_with_else
try:
    x
except: # ! (1, 61:6)
    y
else:
    z

# case: bare_except_with_finally
try:
    x
except: # ! (1, 69:6)
    y
finally:
    z

# case: bare_except_with_else_and_finally
try:
    x
except: # ! (1, 77:6)
    y
else:
    z
finally:
    w

# case: nested_bare_except_in_try_body
try:
    try:
        x
    except: # ! (5, 88:10)
        y
except ValueError:
    z

# case: nested_bare_except_in_handler
try:
    x
except: # ! (1, 99:10)
    try:
        y
    except: # ! (5, 99:10)
        z

# case: nested_bare_except_in_else
try:
    x
except ValueError:
    y
else:
    try:
        z
    except: # ! (5, 110:10)
        w

# case: nested_bare_except_in_finally
try:
    x
finally:
    try:
        y
    except: # ! (5, 119:10)
        z

# case: bare_except_in_nested_function
def outer():
    def inner():
        try:
            x
        except: # ! (9, 127:14)
            y
    return inner

# case: consecutive_try_statements
try:
    x
except: # ! (1, 134:6)
    y
try:
    z
except: # ! (1, 138:6)
    w

# case: typed_except_not_flagged
try:
    x
except Exception:
    y

# case: typed_except_with_name_not_flagged
try:
    x
except Exception as e:
    y

# case: dotted_except_not_flagged
try:
    x
except os.error:
    y

# case: dotted_except_with_name_not_flagged
try:
    x
except os.error as err:
    y

# case: try_finally_not_flagged
try:
    x
finally:
    y

# case: multiple_typed_handlers_not_flagged
try:
    x
except ValueError:
    y
except KeyError:
    z
