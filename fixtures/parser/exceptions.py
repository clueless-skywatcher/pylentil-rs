# case: name
try:
    a
except E:
    b

# case: name_as
try:
    a
except E as err:
    b

# case: parenthesized_name
try:
    a
except (E):
    b

# case: tuple
try:
    a
except (E, F):
    b

# case: tuple_as
try:
    a
except (E, F) as err:
    b

# case: single_element_tuple
try:
    a
except (E,):
    b

# case: attribute
try:
    a
except os.error:
    b

# case: nested_attribute
try:
    a
except pkg.mod.Error:
    b

# case: attribute_as
try:
    a
except os.error as err:
    b

# case: call
try:
    a
except errors():
    b

# case: call_with_argument
try:
    a
except errors(E):
    b

# case: subscript
try:
    a
except errors[E]:
    b

# case: subscript_as
try:
    a
except errors[E] as err:
    b

# case: or_expression
try:
    a
except E or F:
    b

# case: and_expression
try:
    a
except E and F:
    b

# case: bitwise_or
try:
    a
except E | F:
    b

# case: or_expression_as
try:
    a
except E or F as err:
    b

# case: parenthesized_or
try:
    a
except (E or F):
    b

# case: mixed_handlers
try:
    a
except os.error as err:
    b
except (E, F):
    c
except G or H:
    d
