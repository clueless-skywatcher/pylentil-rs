# case: pass
pass

# case: break
break

# case: continue
continue

# case: return_value
def f():
    return 1

# case: return_bare
def f():
    return

# case: while_loop
while a:
    b

# case: while_else
while a:
    b
else:
    c

# case: for_loop
for i in items:
    f(i)

# case: for_unpacking
for k, v in pairs:
    f(k)

# case: function_definition
def f(a, b):
    return a

# case: function_no_arguments
def f():
    pass

# case: function_default_argument
def f(a=1):
    pass

# case: function_annotated
def f(a: int) -> int:
    return a

# case: class_definition
class C:
    pass

# case: class_with_base
class C(Base):
    pass

# case: import
import os

# case: import_dotted
import os.path

# case: import_as
import numpy as np

# case: from_import
from os import path

# case: from_import_star
from os import *

# case: relative_import
from . import sibling

# case: del
del a

# case: global
global x

# case: nonlocal
nonlocal x

# case: assert
assert a

# case: assert_with_message
assert a, "boom"

# case: raise
raise ValueError

# case: raise_from
raise ValueError from err

# case: bare_raise
raise

# case: try_except
try:
    a
except E:
    b

# case: try_except_as
try:
    a
except E as err:
    b

# case: try_finally
try:
    a
finally:
    b

# case: try_except_else_finally
try:
    a
except E:
    b
else:
    c
finally:
    d

# case: with_statement
with open(f) as fh:
    pass

# case: with_multiple_items
with a as x, b as y:
    pass

# case: lambda
f = lambda x: x

# case: lambda_no_args
f = lambda: 1

# case: decorated_function
@dec
def f():
    pass

# case: async_function
async def f():
    await g()

# case: async_for
async def f():
    async for x in y:
        pass

# case: yield_value
def f():
    yield 1

# case: yield_from
def f():
    yield from g()

# case: match_statement
match x:
    case 1:
        pass

# case: semicolon_separated
a = 1; b = 2

# case: semicolon_trailing
a = 1;

# case: type_alias
type Alias = int
