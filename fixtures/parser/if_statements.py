# case: simple
if a:
    b

# case: multi_statement_body
if a:
    b
    c

# case: assignment_in_body
if a:
    x = 1

# case: two_assignments_in_body
if a:
    x = 1
    y = 2

# case: assignment_then_expression
if a:
    x = 1
    y

# case: expression_then_assignment
if a:
    y
    x = 1

# case: with_else
if a:
    b
else:
    c

# case: else_with_assignment
if a:
    x = 1
else:
    x = 2

# case: elif
if a:
    b
elif c:
    d

# case: elif_else
if a:
    b
elif c:
    d
else:
    e

# case: two_elifs
if a:
    b
elif c:
    d
elif e:
    f

# case: nested
if a:
    if b:
        c

# case: nested_with_else
if a:
    if b:
        c
    else:
        d

# case: nested_then_sibling_statement
if a:
    if b:
        c
    d

# case: comparison_condition
if x < 10:
    y = 1

# case: boolean_condition
if a and b:
    c

# case: tuple_condition
if a, b:
    c

# case: parenthesized_condition
if (a):
    b

# case: inline_body
if a: b

# case: inline_body_with_else
if a: b
else: c

# case: statement_after_block
if a:
    b
c

# case: two_if_statements
if a:
    b
if c:
    d

# case: blank_line_before_else
if a:
    b

else:
    c

# case: missing_colon
if a
    b

# case: missing_body
if a:
pass

# case: missing_condition
if :
    b

# case: else_without_if
else:
    b
