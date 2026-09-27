# case: single_level
if a:
    b

# case: two_levels
if a:
    if b:
        c

# case: dedent_one_level
if a:
    if b:
        c
    d

# case: dedent_to_module_level
if a:
    if b:
        c
d

# case: tab_indent
if a:
	b

# case: eight_space_indent
if a:
        b

# case: blank_line_inside_block
if a:
    b

    c

# case: blank_line_between_blocks
if a:
    b

if c:
    d

# case: indented_blank_line_inside_block
if a:
    b

    c

# case: first_line_indented
    x = 1

# case: inconsistent_dedent
if a:
        b
    c

# case: dedent_to_unknown_level
if a:
    if b:
            c
        d

# case: reindent_after_dedent
if a:
    b
if c:
    d

# case: block_at_eof
if a:
    b

# case: deep_nesting
if a:
    if b:
        if c:
            if d:
                e
