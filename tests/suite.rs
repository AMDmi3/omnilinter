mod utils;

use rstest::{fixture, rstest};
use utils::TestCase;

mod simple_matching {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new();
        test_case
            .add_file("a.txt", "foo\nbar\nbaz\n")
            .add_file("b.txt", "foo\nbar\nbaz\n")
            .add_file("c.txt", "foo\nbar\nbaz\n");
        test_case
    }

    #[rstest]
    fn simple_matching_no_match(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: 'd.txt'\n  match: 'bar'")
            .assert_matches(vec![]);
    }

    #[rstest]
    fn simple_matching_single_match(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: 'a.txt'\n  match: 'bar'")
            .assert_matches(vec!["a.txt:2"]);
    }

    #[rstest]
    fn simple_matching_multiple_files(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: '*.txt'\n  match: 'bar'")
            .assert_matches(vec!["a.txt:2", "b.txt:2", "c.txt:2"]);
    }

    #[rstest]
    fn simple_matching_multiple_lines(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: 'a.txt'\n  match: '...' ")
            .assert_matches(vec!["a.txt:1", "a.txt:2", "a.txt:3"]);
    }
}

mod match_file_only {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new();
        test_case
            .add_file("a.txt", "")
            .add_file("b.txt", "")
            .add_file("c.txt", "");
        test_case
    }

    #[rstest]
    fn single_match(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: 'a.txt'")
            .assert_matches(vec!["a.txt"]);
    }

    #[rstest]
    fn multiple_matches(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: '*.txt'")
            .assert_matches(vec!["a.txt", "b.txt", "c.txt"]);
    }
}

mod glob_scope {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new();
        test_case
            .add_file("a.py", "")
            .add_file("dir1/b.py", "")
            .add_file("dir1/dir2/c.py", "");
        test_case
    }

    #[rstest]
    fn filename_pattern(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: '*.py'")
            .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"]);
    }

    #[rstest]
    fn path_pattern_matched_everywhere(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: '**/*.py'")
            .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"]);
    }

    #[rstest]
    fn path_pattern_with_leading_slash_matched_everywhere(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: '/**/*.py'")
            .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"]);
    }

    #[rstest]
    fn path_pattern_with_leading_slash_matched_at_root(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: '/*.py'")
            .assert_matches(vec!["a.py"]);
    }

    #[rstest]
    fn path_pattern_matched_in_subdir(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: 'dir1/*.py'")
            .assert_matches(vec!["dir1/b.py"]);
    }

    #[rstest]
    fn path_pattern_with_leading_slash_matched_in_subdir(mut test_case: TestCase) {
        test_case
            .run_with_rule("- files: '/dir1/*.py'")
            .assert_matches(vec!["dir1/b.py"]);
    }
}

#[test]
fn multiple_globs() {
    TestCase::new()
        .add_file("a.py", "")
        .add_file("a.txt", "")
        .add_file("a.rs", "")
        .run_with_rule("- files: '*.py **/*.txt /*.rs'")
        .assert_matches(vec!["a.py", "a.txt", "a.rs"]);
}

mod nofiles {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new();
        test_case
            .add_file("a.py", "")
            .add_file("b.py", "")
            .add_file("c.py", "");
        test_case
    }

    #[rstest]
    fn matches(mut test_case: TestCase) {
        // note that we also check that this matches once, not per every file
        test_case
            .run_with_rule("- nofiles: '*.txt'")
            .assert_matches(vec![""]);
    }

    #[rstest]
    fn no_matches(mut test_case: TestCase) {
        test_case
            .run_with_rule("- nofiles: '*.py' ")
            .assert_matches(vec![]);
    }
}

mod nofiles_multiple_globs {
    use super::*;

    #[test]
    fn should_not_match_if_any_pattern_matches() {
        TestCase::new()
            .add_file("README", "")
            .run_with_rule("- nofiles: 'README README.txt'")
            .assert_matches(vec![]);
    }

    #[test]
    fn should_match_if_neither_pattern_matches() {
        TestCase::new()
            .add_file("README", "")
            .run_with_rule("- nofiles: 'README.txt README.md'")
            .assert_matches(vec![""]);
    }
}

mod empty_glob {
    use super::*;

    #[test]
    #[should_panic]
    fn string() {
        TestCase::new().run_with_rule("- files: '     '");
    }

    #[test]
    #[should_panic]
    fn seq() {
        TestCase::new().run_with_rule("- files: []");
    }
}

#[test]
fn nomatch() {
    TestCase::new()
        .add_file("a.py", "a\nb\n\nc\n")
        .add_file("b.py", "# SPDX-License-Identifier: GPLv3\na\nb\n\nc\n")
        .run_with_rule("- files: '*.py'\n  nomatch: '# SPDX-License-Identifier: GPLv3'")
        .assert_matches(vec!["a.py"]);
}

mod tags {
    use super::*;

    #[test]
    fn required_not_matching() {
        TestCase::new()
            .add_file("a.py", "")
            .add_arg("--tags=MYTAG")
            .run_with_rule("- files: '*.py'")
            .assert_matches(vec![]);
    }

    #[test]
    fn required_matching() {
        TestCase::new()
            .add_file("a.py", "")
            .add_arg("--tags=MYTAG")
            .run_with_rule("- tags: 'MYTAG'\n  files: '*.py'")
            .assert_matches(vec!["a.py"]);
    }

    #[test]
    fn skipped_not_matching() {
        TestCase::new()
            .add_file("a.py", "")
            .add_arg("--skip-tags=MYTAG")
            .run_with_rule("- files: '*.py'")
            .assert_matches(vec!["a.py"]);
    }

    #[test]
    fn skipped_matching() {
        TestCase::new()
            .add_file("a.py", "")
            .add_arg("--skip-tags=MYTAG")
            .run_with_rule("- tags: 'MYTAG'\n  files: '*.py'")
            .assert_matches(vec![]);
    }
}

#[test]
fn ignore_marker() {
    TestCase::new()
        .add_file("a.py", "foo\nbar  # omnilinter: ignore")
        .run_with_rule("- files: '*.py'\n  match: 'foo|bar'")
        .assert_matches(vec!["a.py:1"]);
}

mod error_exitcode {
    use super::*;

    #[test]
    fn zero() {
        TestCase::new()
            .add_file("a.py", "")
            .add_arg("--error-exitcode=0")
            .run_with_rule("- files: '*.py'"); // should not panic
    }

    #[test]
    #[should_panic]
    fn nonzero() {
        TestCase::new()
            .add_file("a.py", "")
            .add_arg("--error-exitcode=1")
            .run_with_rule("- files: '*.py'");
    }
}
