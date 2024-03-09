## 0.4.0

- Fix incorrect match reporting level (files vs. lines) in some cases
- Fix incorrect line match reporting despite of `match` preconditions not
  satisfied
- Fix first parser eating away first character of exclude regexps
- Improve ruleset syntax
  - Don't use backslash escaping in rule titles, literal `]` may now be
    written as `]]`
  - Don't use backslash escaping in regexps - it's not really needed as
    (almost) any framing charactes are allowed
  - Support unicode framing characters in regexps
  - Disallow brackets as regexp framing characters to avoid confusion
- Implement parsed config dumping (may be useful in future for config
  format migration)
- Allow to delimit tags with commas in CLI (`--ignore-tags=foo,bar,baz`)
- Make tags case insensitive
- Improve glob and regexp matching performance
- Don't panic with unhelpful message when non-directory is specified as root
- No longer rely on rust unstable features and thus require rust nightly
- Make some features conditionally compiled

## 0.3.0

- Switch to custom config format
- Support filtering rules by tags
- Support negative conditions (`nofiles` and `nomatch`)
- Support much more complex condition structure consisting of multiple
  positive and negative glob checks with multiple positive and negative
  content checks
- Support multiple patterns (both globs and regexps) per condition
- Support exclusion patterns (both globs and regexps)
- Support JSON output
- Support multiple text output modes
- Support output coloring
- Optimize matching engine which now does both directory traversal
  and file content checking in single pass
- Support processing multiple roots in parallel, further improving the
  performance
- Support `--error-exit` option
- Support inline `omnilinter: ignore` markers

## 0.2.0

- Support checks for file existence only, without looking at content
- Support full path reporting
- Support default config

## 0.1.0

- Proof of concept release featuring YAML based config, single `files`
  and `match` conditions per rule, unoptimized engine and simple text
  output.
