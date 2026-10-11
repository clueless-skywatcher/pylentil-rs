class Person:
    __slots__ = ("age", "name")

    def _init_(self, name):
        self.name = name

    @property
    def age(self):
        return self._age

x = 2