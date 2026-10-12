# case: self_assigning_variable

x = x   # ! (1, 6)

# case: self_assigning_with_annotation

x: int = x # ! (1, 11)
