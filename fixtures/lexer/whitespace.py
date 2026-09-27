# case: tab_then_spaces
if a:
	    b

# case: spaces_then_tab
if a:
    	b

# case: tabs_for_first_block_spaces_for_second
if a:
	b
if c:
    d

# case: explicit_line_continuation
x = 1 + \
    2

# case: continuation_in_condition
if a and \
   b:
    c

# case: implicit_continuation_parens
x = (1 +
     2)

# case: implicit_continuation_brackets
x = [1,
     2]

# case: implicit_continuation_braces
x = {
    "a": 1,
}

# case: implicit_continuation_call
f(
    a,
    b,
)

# case: indented_continuation_line_looks_like_block
x = (
        1
)

# case: multiple_spaces_between_tokens
a    +    b

# case: no_spaces_between_tokens
a+b

# case: space_before_colon
if a :
    b
