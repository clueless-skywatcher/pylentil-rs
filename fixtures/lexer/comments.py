# case: full_line
# just a comment

# case: trailing
x = 1  # assign one

# case: hash_inside_comment
# a comment with # inside it

# case: comment_between_statements
a = 1
# in between
b = 2

# case: indented_comment
if a:
    # explain
    b

# case: comment_at_module_indent_inside_block
if a:
# dedented comment
    b

# case: comment_only_file
# nothing but this

# case: shebang
#!/usr/bin/env python

# case: coding_declaration
# -*- coding: utf-8 -*-

# case: hash_in_string_not_comment
x = "# not a comment"

# case: comment_after_colon
if a:  # trailing on a block opener
    b
