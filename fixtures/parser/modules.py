# case: single_statement
a

# case: two_statements
a
b

# case: three_statements
a
b
c

# case: statements_with_blank_lines
a

b

# case: leading_blank_lines
a

# case: assignment_then_expression
x = 1
x

# case: expression_then_assignment
x
x = 1

# case: two_assignments
x = 1
y = 2

# case: statement_block_statement
a
if b:
    c
d

# case: block_then_block
if a:
    b
if c:
    d

# case: block_last
a
if b:
    c

# case: consecutive_blank_lines
a



b

# case: many_statements
a = 1
b = 2
c = a + b
d = c * 2
e = d - 1
