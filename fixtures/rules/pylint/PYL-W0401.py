# case: wildcard_import

from x import * # ! (15, 16)
from x.y import * # ! (17, 18)
from x.y.z import * # ! (19, 20)