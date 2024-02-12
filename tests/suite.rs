mod utils;

use utils::TestCase;

#[test]
fn simple_matching() {
    TestCase::new()
        .add_file("a.txt", "foo\nbar\nbaz\n")
        .add_file("b.txt", "foo\nbar\nbaz\n")
        .add_file("c.txt", "foo\nbar\nbaz\n")
        .run_with_rule(
            "
            - title: no match
              files: 'd.txt'
              match: 'bar'
            ",
        )
        .assert_matches(vec![])
        .run_with_rule(
            "
            - title: single match
              files: 'a.txt'
              match: 'bar'
            ",
        )
        .assert_matches(vec!["a.txt:2"])
        .run_with_rule(
            "
            - title: multiple files
              files: '*.txt'
              match: 'bar'
            ",
        )
        .assert_matches(vec!["a.txt:2", "b.txt:2", "c.txt:2"])
        .run_with_rule(
            "
            - title: multiple lines
              files: 'a.txt'
              match: '...'
            ",
        )
        .assert_matches(vec!["a.txt:1", "a.txt:2", "a.txt:3"]);
}

#[test]
fn match_file_only() {
    TestCase::new()
        .add_file("a.txt", "")
        .add_file("b.txt", "")
        .add_file("c.txt", "")
        .run_with_rule(
            "
            - title: single match
              files: 'a.txt'
            ",
        )
        .assert_matches(vec!["a.txt"])
        .run_with_rule(
            "
            - title: single match
              files: '*.txt'
            ",
        )
        .assert_matches(vec!["a.txt", "b.txt", "c.txt"]);
}

#[test]
fn glob_scope() {
    TestCase::new()
        .add_file("a.py", "")
        .add_file("dir1/b.py", "")
        .add_file("dir1/dir2/c.py", "")
        .run_with_rule(
            "
            - title: filename pattern (matches everywhere)
              files: '*.py'
            ",
        )
        .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"])
        .run_with_rule(
            "
            - title: path pattern (matches everywhere)
              files: '**/*.py'
            ",
        )
        .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"])
        .run_with_rule(
            "
            - title: path pattern with leading slash (matches everywhere)
              files: '/**/*.py'
            ",
        )
        .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"])
        .run_with_rule(
            "
            - title: path pattern with leading slash (matches in root only)
              files: '/*.py'
            ",
        )
        .assert_matches(vec!["a.py"])
        .run_with_rule(
            "
            - title: path pattern (matches in dir1 only)
              files: 'dir1/*.py'
            ",
        )
        .assert_matches(vec!["dir1/b.py"])
        .run_with_rule(
            "
            - title: path pattern with leading slash (matches in dir1 only)
              files: '/dir1/*.py'
            ",
        )
        .assert_matches(vec!["dir1/b.py"]);
}

#[test]
fn multiple_globs() {
    TestCase::new()
        .add_file("a.py", "")
        .add_file("a.txt", "")
        .add_file("a.rs", "")
        .run_with_rule(
            "
            - title: multiple patterns
              files: '*.py **/*.txt /*.rs'
            ",
        )
        .assert_matches(vec!["a.py", "a.txt", "a.rs"]);

    TestCase::new()
        .add_file("README", "")
        .run_with_rule(
            "
            - title: nofiles should not match if any pattern matches
              nofiles: 'README README.txt'
            ",
        )
        .assert_matches(vec![])
        .run_with_rule(
            "
            - title: nofiles should match if neither pattern matches
              nofiles: 'README.txt README.md'
            ",
        )
        .assert_matches(vec![""]);
}

#[test]
fn nofiles() {
    TestCase::new()
        .add_file("a.py", "")
        .add_file("b.py", "")
        .add_file("c.py", "")
        .run_with_rule(
            "
            - title: nofiles which matches
              nofiles: '*.txt'
            ",
        )
        .assert_matches(vec![""]) // note also that this must match once, not per every file
        .run_with_rule(
            "
            - title: nofiles which doesn't match
              nofiles: '*.py'
            ",
        )
        .assert_matches(vec![]);
}

#[test]
#[should_panic]
fn empty_globs_string() {
    TestCase::new().run_with_rule(
        "
            - title: nofiles which matches
              files: '    '
            ",
    );
}

#[test]
#[should_panic]
fn empty_globs_seq() {
    TestCase::new().run_with_rule(
        "
            - title: nofiles which matches
              files: []
            ",
    );
}

#[test]
fn nomatch() {
    TestCase::new()
        .add_file("a.py", "a\nb\n\nc\n")
        .add_file("b.py", "# SPDX-License-Identifier: GPLv3\na\nb\n\nc\n")
        .run_with_rule(
            "
            - title: nomatch
              files: '*.py'
              nomatch: '# SPDX-License-Identifier: GPLv3'
            ",
        )
        .assert_matches(vec!["a.py"]);
}

#[test]
fn tags() {
    TestCase::new()
        .add_file("a.py", "")
        .add_arg("--tags=MYTAG")
        .run_with_rule(
            "
            - title: tags test
              files: '*.py'
            ",
        )
        .assert_matches(vec![])
        .run_with_rule(
            "
            - title: tags test
              tags: 'MYTAG'
              files: '*.py'
            ",
        )
        .assert_matches(vec!["a.py"]);

    TestCase::new()
        .add_file("a.py", "")
        .add_arg("--skip-tags=MYTAG")
        .run_with_rule(
            "
            - title: tags test
              files: '*.py'
            ",
        )
        .assert_matches(vec!["a.py"])
        .run_with_rule(
            "
            - title: tags test
              tags: 'MYTAG'
              files: '*.py'
            ",
        )
        .assert_matches(vec![]);
}

#[test]
fn ignore_marker() {
    TestCase::new()
        .add_file("a.py", "foo\nbar  # omnilinter: ignore")
        .run_with_rule(
            "
            - title: test
              files: '*.py'
              match: 'foo|bar'
            ",
        )
        .assert_matches(vec!["a.py:1"]);
}

#[test]
#[should_panic]
fn error_exitcode() {
    TestCase::new()
        .add_file("a.py", "")
        .add_arg("--error-exitcode=1")
        .run_with_rule(
            "
            - title: test
              files: '*.py'
            ",
        );
}
