# case: no_bases
class C:
    pass

# case: empty_parens
class C():
    pass

# case: one_base
class C(Base):
    pass

# case: two_bases
class C(A, B):
    pass

# case: trailing_comma
class C(A, B,):
    pass

# case: dotted_base
class C(mod.Base):
    pass

# case: subscript_base
class C(list[int]):
    pass

# case: expression_base
class C(A + B):
    pass

# case: keyword_base
class C(A, metaclass=M):
    pass

# case: star_bases
class C(A, *rest, **kw):
    pass

# case: inline_body
class C: pass

# case: inline_body_two_statements
class C: a; b

# case: multi_statement_body
class C:
    a
    b

# case: body_with_assign
class C:
    x = 1

# case: method
class C:
    def f(self):
        pass

# case: nested_class
class C:
    class D:
        pass

# case: class_then_statement
class C:
    pass
x

# case: class_in_if
if a:
    class C:
        pass

# case: decorated
@dec
class C:
    pass

# case: decorated_with_base
@a.b
@c()
class C(Base):
    pass

# case: decorated_method
class C:
    @dec
    def f(self):
        pass

# case: missing_name
class:
    pass

# case: literal_name
class 1:
    pass

# case: dotted_name
class C.D:
    pass

# case: missing_colon
class C
    pass

# case: missing_body
class C:
pass

# case: unclosed_parens
class C(Base:
    pass

# case: positional_after_keyword
class C(metaclass=M, A):
    pass

# case: posonly_marker
class C(A, /):
    pass

# case: bare_star
class C(A, *):
    pass
