class Person:
    __slots__ = ("age", "name")

    def _init_(self, name): # !
        self.name = name

    def __abcd__(self): # !
        pass

    def __eq__(self, other):
        return True

    def __init__(self, name):
        self.name = name

    def good(self):
        pass
