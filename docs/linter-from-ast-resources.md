# Implementing a linter on top of an AST: reading list

Resources that discuss how a linter is built once a parser has produced an
abstract syntax tree. Grouped by what each one is most useful for.

## Building a linter from an AST

- [Learn Python ASTs by building your own linter (DeepSource)](https://deepsource.com/blog/python-asts-by-building-your-own-linter),
  also at [tush.ar](https://tush.ar/post/ast/). Walks from parsing to a working
  `NodeVisitor`-based linter with several rules.
- [A Code Linter (Software Design by Example in Python)](https://third-bit.com/sdxpy/lint/).
  Book chapter that builds a linter with a visitor, then adds scope tracking to
  find unused and undefined variables.
- [How to write a linter in Python (Gui Commits)](https://guicommits.com/how-to-write-linter-python/).
  Short end-to-end example with rule classes and a reporter.
- [Build Your Own Custom Python Linter (Learn Modern Python)](https://learnmodernpython.com/build-your-own-custom-python-linter/).
- [Visiting an Abstract Syntax Tree (Pat Shaughnessy)](https://patshaughnessy.net/2022/1/22/visiting-an-abstract-syntax-tree).
  Explains the visitor pattern on a Ruby AST from the tool author's angle.
- [ASTs Are Awesome](https://astsareawesome.com/). Interactive site on ASTs and
  linting.

## How production linters are structured

- [ESLint Architecture](https://eslint.org/docs/latest/contribute/architecture/).
  The traversal emits an event per node type on entry and an `:exit` event on
  the way back up. Rules subscribe to those events.
- [ESLint Selectors](https://eslint.org/docs/latest/extend/selectors). CSS-like
  matching of AST nodes, and the
  [pull request that introduced it](https://github.com/eslint/eslint/pull/7833).
- [ESLint ScopeManager interface](https://eslint.org/docs/latest/extend/scope-manager-interface)
  and [eslint-scope](https://www.npmjs.com/package/eslint-scope). The scope and
  reference model built from the AST before rules run.
- [Pylint: How to Write a Checker](https://pylint.pycqa.org/en/stable/how_tos/custom_checkers.html).
  Checkers are visitors with `visit_<node>` and `leave_<node>` methods. A
  companion [blog walkthrough](http://atodorov.org/blog/2018/01/05/how-to-write-pylint-checker-plugins/).
- [pyflakes `checker.py`](https://github.com/PyCQA/pyflakes/blob/main/pyflakes/checker.py).
  One file that models bindings and scopes and reports unused imports and
  undefined names. Small enough to read in full.
- [Ruff Linter (DeepWiki)](https://deepwiki.com/astral-sh/ruff/3-ruff-linter) and
  [Adding Lint Rules (DeepWiki)](https://deepwiki.com/astral-sh/ruff/9.4-adding-lint-rules).
  Describes the bind, traverse, clean up, analyze cycle in the AST checker and
  the registration steps for a rule.
- [Clippy: Adding Lints](https://doc.rust-lang.org/stable/clippy/development/adding_lints.html)
  and [Lint Passes](https://doc.rust-lang.org/nightly/clippy/development/lint_passes.html).
  Early passes see only syntax, late passes see types. Also
  [Making a small Clippy lint (Erk)](https://erk.dev/2025/08/21/clippy-lint).
- [Using go/analysis to write a custom linter (Fatih Arslan)](https://arslan.io/2019/06/13/using-go-analysis-to-write-a-custom-linter/)
  and the [go/analysis package docs](https://pkg.go.dev/golang.org/x/tools/go/analysis).
  Analyzers declare dependencies on other analyzers and share results such as a
  prebuilt inspector.
- [How to create your own Go static analyzer (PVS-Studio)](https://pvs-studio.com/en/blog/posts/go/1329/).

## Linters written in Rust

- [Writing your own Rust linter (Guillaume Gomez)](https://blog.guillaume-gomez.fr/articles/2024-01-18+Writing+your+own+Rust+linter).
- [Write Rust lints without forking Clippy (Trail of Bits)](https://blog.trailofbits.com/2021/11/09/write-rust-lints-without-forking-clippy/).
- [Building a Linter (Build JavaScript Development Tools with Rust)](https://rusty-ecma.github.io/rusty-ecma-book/02.ress/2.html).
- [Learn Rust Closures By Building a Tiny Rule-Based Linter](https://blog.sheerluck.dev/posts/learn-rust-closures-by-building-a-tiny-linter/).

## Scope and name resolution background

- [Let's Build a Simple Interpreter, Part 14: Nested Scopes (Ruslan Spivak)](https://ruslanspivak.com/lsbasi-part14/).
  Chained symbol tables pushed and popped while walking the tree, which is the
  model pyflakes and ruff use.
- [Scope analysis utilities (eslint-utils)](https://eslint-community.github.io/eslint-utils/api/scope-utils.html).

## Where ruff does each of these things

All paths are under `crates/` in [astral-sh/ruff](https://github.com/astral-sh/ruff).

| Concern | Location |
| --- | --- |
| Entry point per file | `ruff_linter/src/linter.rs`: `check_path`, `lint_only`, `lint_fix` |
| Checker kinds | `ruff_linter/src/checkers/{tokens,physical_lines,logical_lines,imports,filesystem,noqa}.rs` and `checkers/ast/mod.rs` |
| AST walk | `checkers/ast/mod.rs`: struct `Checker`, `impl Visitor for Checker`, `check_ast` |
| Per-node rule dispatch | `checkers/ast/analyze/statement.rs` and `analyze/expression.rs`, guarded by `checker.is_rule_enabled(Rule::X)` |
| Scope-level rules after the walk | `checkers/ast/analyze/deferred_scopes.rs`, `bindings.rs`, `unresolved_references.rs`, `definitions.rs` |
| Semantic model | `ruff_python_semantic/src/model.rs`: `SemanticModel`, plus `binding.rs`, `scope.rs`, `reference.rs` |
| Rule definition | `ruff_linter/src/violation.rs`: `Violation`, `AlwaysFixableViolation`, `ViolationMetadata` |
| Rule registry and codes | `ruff_linter/src/codes.rs`, `registry.rs`, `rule_selector.rs`, `settings/rule_table.rs` |
| Example rule | `ruff_linter/src/rules/pylint/rules/comparison_with_itself.rs` |
| Example fixable rule | `ruff_linter/src/rules/pyupgrade/rules/useless_metaclass_type.rs` |
| Reporting and fixes | `checkers/ast/mod.rs`: `report_diagnostic`, `DiagnosticGuard::set_fix`; `ruff_diagnostics/src/{edit,fix}.rs`; `ruff_linter/src/fix/mod.rs` |
| Suppression comments | `ruff_linter/src/noqa.rs`, `checkers/noqa.rs`, `directives.rs` |
| Rule tests | `ruff_linter/src/test.rs`, fixtures in `ruff_linter/resources/test/fixtures/`, snapshots next to each rule family |
