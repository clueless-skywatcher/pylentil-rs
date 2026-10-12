class Person:
    __slots__ = ("age", "name")

    def _init_(self, name): # ! (9, 15)
        self.name = name

    def __abcd__(self): # ! (9, 17)
        pass

    def __eq__(self, other):
        return True

    def __init__(self, name):
        self.name = name

    def good(self):
        pass
