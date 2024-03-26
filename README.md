# omnilinter

[![CI](https://github.com/AMDmi3/omnilinter/actions/workflows/ci.yml/badge.svg)](https://github.com/AMDmi3/omnilinter/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/AMDmi3/omnilinter/graph/badge.svg?token=LZC12KUQ0M)](https://codecov.io/gh/AMDmi3/omnilinter)
[![GitHub commits (since latest release)](https://img.shields.io/github/commits-since/AMDmi3/omnilinter/latest.svg)](https://github.com/AMDmi3/omnilinter)

Define path pattern and regular expression rules and match against all
your repositories/projects/codebases at once. Use that to push and uphold
good practices, chase deprecations, and fix common problems all over
your code.

## Example

```
% cat omnilinter.conf
[convert deprecated auto_ptr to unique_ptr]
files *.cpp
match /auto_ptr/

[convert setup.py to pyproject.toml]
files setup.py
nofiles pyproject.toml

[add license information]
files *.py *.c* *.h* *.rs
nomatch /^..? SPDX-FileCopyrightText:/

[add CI workflow]
files *.py *.c* *.h* *.rs
nofiles .github/workflows/*.yml

[add project README]
nofiles /README*
```
```
% omnilinter -c omnilinter.conf my_projects/*
my_projects/my_python_project
  setup.py: convert setup.py to pyproject.toml
  src/__init__.py: add license information
my_project/my_cpp_lib
  add project README
  add CI workflow
  src/main.cpp: add license information
  src/main.cpp:17: convert deprecated auto_ptr to unique_ptr
```

See omnilinter's own [config](.omnilinter.conf) for more examples.

## Running

At the very least, you need to specify path to config file and paths to directories to check:

```
omnilinter -c <path to omnilinter.conf> <directory to check> ...
```

or, if you set directories to check right in the config, and place it in the default location (`~/.config/omnilinter/omnilinter.conf`) you can just run

```
omnilinter
```

### Useful options

- `--tags`, `--skip-tags` - limit operation with a subset of rules.
- `--format by-root|full-paths|by-rule|by-path` - specify output format.
- `--color`, `--palette` - tweak output coloring.
- `--error-exitcode` - exit with specified code if any rule matches, useful for CI and scripts.

See `omnilinter --help` for all options.

## Config file format

Example `omnilinter.conf`:
```
root /path/to/project1
root /path/to/other_projects/*  # patterns are allowed

[rule title]
tags tag1,tag2                  # used with --tags, --exclude-tags
nofiles /README* !/README.txt   # require absence of file
files *.py !*.pyi               # or require presence of a file, in which...
match /Object/ !/^class /       # ...require pattern match...
nomatch "^/usr/share/.*"        # ...or absence of pattern match

[next rule]
...
```

At the beginning of the file, config directives are allowed:

* `root` specifies default directories to operate on. These are only
used if no roots are specified on the command line.

Ruleset follows next, in which each rule consists of:

* Bracketed title which is used when reporting matches. Use `]]` if you
want to include closing bracket in the title. All other parts are optional.

* `tags` directive with a comma or space separated list of tags to filter
rules with `--tags` and `--exclude-tags` command line options.

* Path conditions:
  * `files` which require presence of specific path patterns in the directory.
  * `nofiles` which require absence thereof.

  Each requires one or more shell pattern (e.g. `*.py` or `/src/*.c*` or
  `**/tests/*.rs`) and allows exclusions (prefixed by `!`). Backslash
  escaping and quotes are allowed like in shell (`"program output "\[[0-9]\].txt`
  to match `program output [1].txt`).  Patterns without path separators match
  everywhere (`*.py` matches both `setup.py` and `src/mymodule/__init__.py`),
  while patterns with path separators only match relative to root.

* Content conditions (only allowed after `files` and only apply to
  files matched by that specific `files` condition):
  * `match` requires match of given regular expression pattern in a file.
  * `nomatch` requires absence of such match.

  These require one or more regular expressions enclosed in (almost) any
  character (e.g. `/.*/`, `".*"`, `|.*|` all work, so escaping can be avoided)
  and also allow `!`-prefixed exclusions.

  * `size` checks file size with an operator (`>`, `>=`, `<`, '<=`, `=`
  or `==`, `!=` or `<>`) against given amount of bytes (e.g. `size >= 1024`).
  * `lines` checks number of lines the same way.

You may build rather complex trees out of these conditions, for example:

```
[too big readme for such small rust library]
# match when there's src/lib.rs...
files src/lib.rs
# ...but no other .rs files under src/, which along with the
# previous condition suggest it's a single-file rust library
nofiles src/**/*.rs !src/lib.rs
# if there's README file of any kind,...
files /README*
# ...and it's longer than 25 lines...
lines >= 25
# ...unless there's Example: header (implied that it may contain a lot of code)
nomatch /^#* Example:/
```

When all rule conditions are satisfied, the rule match is reported:

```
README.md: too big readme for such small rust library
```

The match may include context:
* If the very last condition in a rule is `match`, file and line would
  be reported.
* Otherwise, if the very last of _path_ conditions was `files`, file
  would be reported (like in example above).
* Otherwise, there's no specific context, and the report is for the checked
  directory in general.

Therefore rule order matters, so preconditions should be specified first, and
conditions which point to concrete problematic places last.

## Author

* [Dmitry Marakasov](https://github.com/AMDmi3) <amdmi3@amdmi3.ru>

## License

* [GPL-3.0-or-later](LICENSE)
