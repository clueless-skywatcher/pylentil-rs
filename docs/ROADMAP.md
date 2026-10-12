# Roadmap to 1.0

**1.0** is a binary you can point at a real package: span-accurate diagnostics, a stable rule set, config, suppressions, and safe fixes. Not Ruff parity.

Shipped in 0.1: lexer, Pratt parser, spans, lint registry + `create_pylentil_lint!`, and fixture tests. The CLI hardcodes one fixture. `possible_fix` is a static sentence. `PySpan`'s `PartialEq` always returns true, so location tests do not check offsets.

| Code | Based on |
|---|---|
| `PYC-E722` | pycodestyle E722 do not use bare `except` |
| `PYL-W0101` | pylint W0101 unreachable |
| `PYL-W0127` | pylint W0127 self-assigning-variable |
| `PYL-W0401` | pylint W0401 wildcard-import |
| `PYL-W0711` | pylint W0711 binary-op-exception |
| `PYL-W3201` | pylint W3201 bad-dunder-name |
| `PYL-R1711` | pylint R1711 useless-return |

## Engine and tool

| # | Feature | Gap today | Done when |
|---|---|---|---|
| 1 | Real spans in tests | `PySpan` compares equal to every other span | A wrong offset fails the fixture test |
| 2 | Expression visitor | Rules only run from per-statement arms in `Pylentil`; `check_expr` is empty | A rule matches a nested expression without a new engine arm |
| 3 | Located parse errors | `PylentilError` describes what failed and never where | A syntax error prints `file:line:col` |
| 4 | Parse recovery | The first bad token aborts the file | Later statements in that file still lint |
| 5 | CLI | `main` reads `fixtures/somefile.py` and a fixed rule list | `pylentil <paths>` walks dirs, honors `.gitignore`, exits 1 on error |
| 6 | Config | Rules are selected in code | `pyproject.toml` `[tool.pylentil]` sets select, ignore, and severity |
| 7 | Human and machine output | `println` of code + message | Text with a source line, plus a stable JSON/text stream |
| 8 | Suppressions | None | `# noqa: CODE` on a line, and a file-level disable |
| 9 | Safe fixes | Fix text is not an edit | `--fix` applies non-overlapping edits right-to-left; a second run is clean |
| 10 | Scope analysis | No bindings or references | Each name is a binding or a use, including shadowing and closures |
| 11 | Starter rules | Seven rules | The pack below, each with a fixture that asserts the span |
| 12 | Public contract | No README, no rule index | README, `pylentil --explain CODE`, changelog, tagged `1.0.0` binary |

## Starter rules still to write

| Rule | Based on | Needs |
|---|---|---|
| Compare to `None` / `True` / `False` with `==` | pylint C0121 singleton-comparison; pycodestyle E711, E712 | Expression visitor |
| `assert` on a tuple | pylint W0199 assert-on-tuple; pyflakes F631 | Expression visitor |
| Duplicate dict keys | pylint W0109 duplicate-key; pyflakes F601 | Expression visitor |
| `len(x) == 0` / `len(x) > 0` | pylint C1802 use-implicit-booleaness-not-len | Expression visitor |
| `eval` / `exec` | pylint W0123 eval-used, W0122 exec-used | Expression visitor |
| Empty `if` / `for` / `while` / `except` / `with` body | pylint W0107 unnecessary-pass | AST |
| Mutable default argument | pylint W0102 dangerous-default-value | AST |
| Inconsistent `return` (value mixed with bare `return`) | pylint R1710 inconsistent-return-statements | AST |
| Useless expression statement | pylint W0104 pointless-statement, W0106 expression-not-assigned | AST |
| Too many arguments | pylint R0913 too-many-arguments | AST |
| `except Exception` / `except BaseException` | pylint W0718 broad-exception-caught | AST |
| Import not at module top | pylint C0415 import-outside-toplevel; pycodestyle E402 | AST |
| CapWords / snake_case / `UPPER_CASE` | pylint C0103 invalid-name | AST |
| Ambiguous `l` / `O` / `I` | pycodestyle E741 ambiguous variable name | AST |
| Redefined builtin | pylint W0622 redefined-builtin | Scopes |
| Undefined name | pylint E0602 undefined-variable; pyflakes F821 | Scopes |
| Unused import | pylint W0611 unused-import; pyflakes F401 | Scopes |
| Unused local | pylint W0612 unused-variable; pyflakes F841 | Scopes |
| Unused argument (`self`, `cls`, `_…` exempt) | pylint W0613 unused-argument | Scopes |

## After 1.0

| Feature | Why later |
|---|---|
| LSP (diagnostics + code actions) | Needs recovery and a stable diagnostic shape first |
| Parallel files and a benchmark | Correctness before speed |
| Cross-file imports and unused exports | Needs a module graph |
| Control-flow / dataflow beyond `PYL-W0101` | Current unreachable check is a straight-line scan |
| Type-aware rules | A type checker, or a deliberate non-goal |
| Formatter, plugins | Separate products |
