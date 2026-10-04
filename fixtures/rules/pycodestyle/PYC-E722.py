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

# case: bare_except_in_elif
if a == 1:
    pass
elif a == 2:
    try:
        x
    except: # !
        y

# case: bare_except_in_else
if a == 1:
    pass
else:
    try:
        x
    except: # !
        y

# case: bare_except_after_typed_handler
try:
    x
except ValueError:
    y
except: # !
    z

# case: bare_except_after_multiple_typed_handlers
try:
    x
except ValueError:
    y
except KeyError as e:
    y
except: # !
    z

# case: bare_except_with_else
try:
    x
except: # !
    y
else:
    z

# case: bare_except_with_finally
try:
    x
except: # !
    y
finally:
    z

# case: bare_except_with_else_and_finally
try:
    x
except: # !
    y
else:
    z
finally:
    w

# case: nested_bare_except_in_try_body
try:
    try:
        x
    except: # !
        y
except ValueError:
    z

# case: nested_bare_except_in_handler
try:
    x
except: # !
    try:
        y
    except: # !
        z

# case: nested_bare_except_in_else
try:
    x
except ValueError:
    y
else:
    try:
        z
    except: # !
        w

# case: nested_bare_except_in_finally
try:
    x
finally:
    try:
        y
    except: # !
        z

# case: bare_except_in_nested_function
def outer():
    def inner():
        try:
            x
        except: # !
            y
    return inner

# case: consecutive_try_statements
try:
    x
except: # !
    y
try:
    z
except: # !
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
