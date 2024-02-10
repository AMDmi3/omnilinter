# omnilinter

Define file pattern and regular expression rules and match against all
your repositories/projects/codebases at once. Omnilinter helps you to
push and uphold good practices and fix problems all over your code, even
in not actively maintained projects.

## Example

Note: ruleset syntax is not final.

```
% cat omnilinter.conf
rules:
- title: convert deprecated auto_ptr to unique_ptr
  files: "*.cpp"
  match: auto_ptr

- title: convert setup.py to pyptoject.toml
  files: setup.py

- title: change indentation to spaces
  files: "*.py"
  match: "^	"

- title: add license information
  files: "*.py *.c* *.h* *.rs"
  nomatch: "^..? SPDX-FileCopyrightText:"
  nomatch: "^..? SPDX-License-Identifier:"

- title: add README.md
  nofiles: README.md
% omnilinter -c omnilinter.conf projects/*
projects/my_python_project:
- setup.py: convert setup.py to pyptoject.toml
- src/__init__.py: add license information
- src/__init__.py:1: change indentation to spaces
- src/__init__.py:2: change indentation to spaces
- src/__init__.py:3: change indentation to spaces
project/my_cpp_lib:
- add README.md
- src/main.cpp: add license information
- src/main.cpp:17: convert deprecated auto_ptr to unique_ptr
- src/main.cpp:49: convert deprecated auto_ptr to unique_ptr
```
