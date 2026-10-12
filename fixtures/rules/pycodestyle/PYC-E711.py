# case: none_comparison

if x is None: # no error
    pass
elif y is not None: # no error
    print(y)

if y == None: # ! (9, 13)
    if z == None: # ! (13, 17)
        pass
    if z != None: # ! (13, 17)
        pass