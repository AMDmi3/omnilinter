# omnilinter

Define file pattern and regular expression rules and match against all
your repositories/projects/codebases at once. Omnilinter helps you to
push and uphold good practices and fix problems all over your code, even
in not actively maintained projects.

## Example

Note: ruleset syntax is not final.

- Ruleset:

  ```
  # checks for "auto_ptr" occurances in all .cpp files
  [convert deprecated auto_ptr to unique_ptr]
      files *.cpp
          match /auto_ptr/

  # checks for whether setup.py is present, but not pyproject.toml
  [convert setup.py to pyproject.toml]
      files setup.py
      nofiles pyproject.toml

  [change indentation to spaces]
      files *.py
          match /^	/
  
  [add license information]
      files *.py *.c* *.h* *.rs
          nomatch /^..? SPDX-FileCopyrightText:/

  # checks for absence of any kind of README
  [add README]
      nofiles README README.txt README.md README.rst
  ```

  See also [config](.omnilinter.conf) used to check omnilinter's own repository.

- Command:

  ```
  % omnilinter -c omnilinter.conf projects/*
  ```

- Output:

  ```
  projects/my_python_project
    setup.py: convert setup.py to pyptoject.toml
    src/__init__.py: add license information
    src/__init__.py:1: change indentation to spaces
    src/__init__.py:2: change indentation to spaces
    src/__init__.py:3: change indentation to spaces
  project/my_cpp_lib
    add README.md
    src/main.cpp: add license information
    src/main.cpp:17: convert deprecated auto_ptr to unique_ptr
    src/main.cpp:49: convert deprecated auto_ptr to unique_ptr
  ```

## Running

```
Usage: omnilinter [OPTIONS] [TARGET_DIR]...

Arguments:
  [TARGET_DIR]...
          Directories to operate on

Options:
  -c, --config <CONFIG_PATH>
          Path(s) to configuration file(s)

  -t, --tags <TAGS>
          Only process rules tagged with these values

      --skip-tags <TAGS>
          Ignore rules tagged with these values

  -f, --format <FORMAT>
          Output format
          
          [default: by-root]

          Possible values:
          - by-root:    Plain text output, grouped by root
          - full-paths: Plain text output, full paths
          - by-rule:    Plain text output, grouped by rule
          - by-path:    Plain text output, grouped by path
          - json:       JSON output

      --color <MODE>
          Coloring
          
          [default: auto]
          [possible values: auto, always, never]

      --palette <PALETTE>
          Palette to use for rule coloring
          
          [default: simple]

          Possible values:
          - none:       No specific rule coloring
          - simple:     Use one of 4 neutral colors (green, blue, magenta, cyan) + their bright variants for each rule
          - severity:   Use color based on severity guessed from tags (error/fatal/critical; warning; minor)
          - true-color: Use wider true color palette

      --error-exitcode <EXITCODE>
          If any matches are found, exit with given code

  -j, --jobs <JOBS>
          Number of target directories to process simultaneously
```

The most basic usage is to specify `-c/--config` with path to your
config/ruleset and a list of directory paths (we call these _roots_) to
operate on. You may specify multiple configs in which case these are used
together (effectively concatenated).

If you don't specify any configs at all, omnilinter tries to read its default
config file, usually from `~/.config/omnilinter/omnilinter.conf`. Since you may
specify _roots_ in the config as well, this allows you to to run omnilinter
without any arguments to process your default list of roots with your default
ruleset.

Use `--tags` and `--skip-tags` to include only subset of rules or to exclude
a subset of rules. Use `--format`, `--color` and `--palette` to tune output.
Use `--error-exitcode` if you want omnilinter to exit with non-zero code if
any rules match, e.g. to use in CI or scripts.

## Config format

```
root /path/to/project1
root /path/to/other_projects/*

[rule title]
	tags tag1,tag2
	nofiles /README* !/README.txt
	files *.py !*.pyi
		match /Object/ !/^class /
		nomatch "^/usr/share/.*"
```

`root` directives specify default directories to operate on. These are
only used if no roots are specified on the command line.

Each rule starts with a bracketed `[rule title]`, which is used when
reporting matches. It's advised to use lowercase incentives here,
for matches to conveniently look as call to action, such as `main.cpp:1:
 fix include dirtive`.

Next you may specify optional list of `tags` (separated by whitespace
or commas) by which you may enable and disable rules from the command line.

Finally, you need to specify a list of conditions for rule to match on
(if you don't, rule always matches). First kind of conditions matches
file paths:
- `files` requires specified paths to exist in the root for rule to match
- `nofiles` requires specified paths to not exist in the repository

Each condition requires a list of shell patterns, some of which may be
prefixed by `!` to act as exclusion.

Patterns without path separators in them (`*.py`) match file names anywhere
in directory hierarchy. Otherwise patterns match paths relative to root,
for instance `/README*` matches files only at the root level of processed
directory.

Each kind of condition may be specified multiple times. For example,

```
files /setup.py
files __init.py__
nofiles /README
nofiles /README.md
```

matches when both `setup.py` and `__init__.py` files are present,
but neither `README` nor `README.md`.

After each `files` directive you may specify another kind of conditions,
which matches file content:
- `match` requires specified regexp patterns to match in the file
- `nomatch` requires specified regexp patterns not to match in the file

Note that these conditions are tied to `files` condition and are checked
against files matched by it. There may be multiple `files`, each with
its own `match`/`nomatch` sets, and these are checked independently.

Similarly to path conditions, each content condition required a list of
regexp patterns, some of which may be prefixed by `!` to act as exclusion.
Patterns are framed by any symbol which is useful to avoid escaping.
For instance, `/foo/`, `"foo"`, `|foo|` all match `foo`.

Condition order is important as it defines which conditions will only
perform checks and which condition will report matches. The reporting
condition is either the very last `match`, or otherwise the last `files`.

So the following reports `license` matches in `*.py` files, but only
when there are `License` matches in the `README.md` file:
```
files README.md
    match License
files *.py
    match license
```
and the following reports `License` matches in `README.md`:
```
files *.py
    match license
files README.md
    match License
```

Whitespace is not important in the config, but it's advised to use
indentation make condition hierarchy more apparent and rules visually
more distinctive, as demonstrated in this document.

## Author

* [Dmitry Marakasov](https://github.com/AMDmi3) <amdmi3@amdmi3.ru>

## License

* [GPL-3.0-or-later](LICENSE)
