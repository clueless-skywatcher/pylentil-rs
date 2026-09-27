# case: double_quoted
"hello"

# case: single_quoted
'hello'

# case: empty
""

# case: escaped_quote
"she said \"hi\""

# case: escaped_backslash
"back\\slash"

# case: escape_newline
"line\nbreak"

# case: other_quote_inside
"it's fine"

# case: triple_double
"""triple quoted"""

# case: triple_single
'''triple quoted'''

# case: triple_spanning_lines
"""line one
line two"""

# case: f_string
f"value is {x}"

# case: raw_string
r"\d+"

# case: bytes_literal
b"raw bytes"

# case: non_ascii_contents
"héllo wörld"

# case: emoji_contents
"shipped 🚀"

# case: adjacent_concatenation
"foo" "bar"

# case: hash_inside_string
"# not a comment"

# case: quote_terminated_by_newline
"no closing quote

# case: unterminated_at_eof
"dangling

# case: trailing_escape_at_eof
"escape \
