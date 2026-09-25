# case: no_args
def f():
    pass

# case: one_arg
def f(a):
    pass

# case: two_args
def f(a, b):
    pass

# case: trailing_comma
def f(a, b,):
    pass

# case: default_arg
def f(a=1):
    pass

# case: defaults_mixed
def f(a, b=1, c=2):
    pass

# case: default_expression
def f(a=1 + 2):
    pass

# case: annotated
def f(a: int):
    pass

# case: annotated_default
def f(a: int = 1):
    pass

# case: annotation_expression
def f(a: list[int]):
    pass

# case: return_annotation
def f() -> int:
    pass

# case: return_annotation_expression
def f() -> list[int]:
    pass

# case: vararg
def f(*args):
    pass

# case: vararg_annotated
def f(*args: int):
    pass

# case: kwarg
def f(**kw):
    pass

# case: vararg_and_kwarg
def f(a, *args, **kw):
    pass

# case: kwonly
def f(a, *, b):
    pass

# case: kwonly_default
def f(*, b=1):
    pass

# case: kwonly_after_vararg
def f(*args, b, c=2):
    pass

# case: kwonly_then_kwarg
def f(*, b, **kw):
    pass

# case: posonly
def f(a, b, /, c):
    pass

# case: posonly_only
def f(a, /):
    pass

# case: posonly_with_defaults
def f(a, b=1, /, c=2):
    pass

# case: everything
def f(a, /, b, c=1, *rest, d, e=2, **kw) -> int:
    return a

# case: inline_body
def f(): pass

# case: inline_body_two_statements
def f(): a; b

# case: multi_statement_body
def f():
    a
    b

# case: body_with_return
def f(a):
    return a

# case: docstring_body
def f():
    "doc"

# case: nested_def
def f():
    def g():
        pass

# case: def_then_statement
def f():
    pass
x

# case: def_in_if
if a:
    def f():
        pass

# case: decorated
@dec
def f():
    pass

# case: two_decorators
@a
@b
def f():
    pass

# case: decorator_call
@app.route("/")
def f():
    pass

# case: async_def
async def f():
    pass

# case: missing_name
def ():
    pass

# case: missing_parens
def f:
    pass

# case: missing_colon
def f()
    pass

# case: missing_body
def f():
pass

# case: positional_after_default
def f(a=1, b):
    pass

# case: duplicate_vararg
def f(*a, *b):
    pass

# case: kwarg_not_last
def f(**kw, a):
    pass

# case: two_posonly_markers
def f(a, /, b, /):
    pass

# case: posonly_marker_first
def f(/, a):
    pass

# case: bare_star_last
def f(a, *):
    pass

# case: literal_parameter
def f(1):
    pass

# case: duplicate_parameter
def f(a, a):
    pass

# case: unclosed_parens
def f(a:
    pass
