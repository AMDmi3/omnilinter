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
