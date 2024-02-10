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
#[ignore]
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
}

#[test]
fn nofiles() {
    TestCase::new()
        .add_file("a.py", "")
        .run_with_rule(
            "
            - title: nofiles which matches
              nofiles: '*.txt'
            ",
        )
        .assert_matches(vec![""])
        .run_with_rule(
            "
            - title: nofiles which doesn't match
              nofiles: '*.py'
            ",
        )
        .assert_matches(vec![]);
}
