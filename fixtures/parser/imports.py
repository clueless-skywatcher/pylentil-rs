# case: import
import os

# case: import_dotted
import os.path

# case: import_deeply_dotted
import a.b.c

# case: import_as
import numpy as np

# case: import_dotted_as
import os.path as p

# case: import_several
import os, sys

# case: import_several_with_alias
import os, numpy as np

# case: from_import
from os import path

# case: from_import_several
from os import path, sep

# case: from_import_as
from os import path as p

# case: from_import_several_with_alias
from os import path as p, sep

# case: from_dotted_import
from os.path import join

# case: from_import_star
from os import *

# case: from_import_parenthesized
from os import (path, sep)

# case: from_import_parenthesized_trailing_comma
from os import (path, sep,)

# case: from_import_parenthesized_multiline
from os import (
    path,
    sep,
)

# case: relative_import
from . import sibling

# case: relative_import_two_dots
from .. import parent

# case: relative_import_three_dots
from ... import grandparent

# case: relative_import_with_module
from .pkg import thing

# case: relative_import_deep
from ...pkg.sub import thing

# case: import_in_block
if a:
    import os

# case: import_then_statement
import os
x = 1

# case: imports_separated_by_semicolon
import os; import sys

# case: import_missing_name
import

# case: import_trailing_comma
import os,

# case: import_as_missing_name
import os as

# case: import_star_without_from
import *

# case: from_missing_import
from os

# case: from_missing_names
from os import

# case: from_import_unparenthesized_trailing_comma
from os import path,

# case: from_import_star_with_alias
from os import * as x

# case: from_import_star_mixed_with_names
from os import *, path

# case: from_import_unclosed_parenthesis
from os import (path, sep
