// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use rstest::{fixture, rstest};
use utils::TestCase;

mod simple_matching {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new_for_json_tests();
        test_case
            .add_file("a.txt", "foo\nbar\nbaz\n")
            .add_file("b.txt", "foo\nbar\nbaz\n")
            .add_file("c.txt", "foo\nbar\nbaz\n");
        test_case
    }

    #[rstest]
    fn simple_matching_no_match(mut test_case: TestCase) {
        test_case
            .add_rule("files d.txt\nmatch /bar/")
            .run()
            .assert_matches(vec![]);
    }

    #[rstest]
    fn simple_matching_single_match(mut test_case: TestCase) {
        test_case
            .add_rule("files a.txt\nmatch /bar/")
            .run()
            .assert_matches(vec!["a.txt:2"]);
    }

    #[rstest]
    fn simple_matching_multiple_files(mut test_case: TestCase) {
        test_case
            .add_rule("files *.txt\nmatch /bar/")
            .run()
            .assert_matches(vec!["a.txt:2", "b.txt:2", "c.txt:2"]);
    }

    #[rstest]
    fn simple_matching_multiple_lines(mut test_case: TestCase) {
        test_case
            .add_rule("files a.txt\nmatch /.../")
            .run()
            .assert_matches(vec!["a.txt:1", "a.txt:2", "a.txt:3"]);
    }
}

mod match_file_only {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new_for_json_tests();
        test_case
            .add_file("a.txt", "")
            .add_file("b.txt", "")
            .add_file("c.txt", "");
        test_case
    }

    #[rstest]
    fn single_match(mut test_case: TestCase) {
        test_case
            .add_rule("files a.txt")
            .run()
            .assert_matches(vec!["a.txt"]);
    }

    #[rstest]
    fn multiple_matches(mut test_case: TestCase) {
        test_case
            .add_rule("files *.txt")
            .run()
            .assert_matches(vec!["a.txt", "b.txt", "c.txt"]);
    }
}

mod glob_scope {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new_for_json_tests();
        test_case
            .add_file("a.py", "")
            .add_file("dir1/b.py", "")
            .add_file("dir1/dir2/c.py", "");
        test_case
    }

    #[rstest]
    fn filename_pattern(mut test_case: TestCase) {
        test_case.add_rule("files *.py").run().assert_matches(vec![
            "a.py",
            "dir1/b.py",
            "dir1/dir2/c.py",
        ]);
    }

    #[rstest]
    fn path_pattern_matched_everywhere(mut test_case: TestCase) {
        test_case
            .add_rule("files **/*.py")
            .run()
            .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"]);
    }

    #[rstest]
    fn path_pattern_with_leading_slash_matched_everywhere(mut test_case: TestCase) {
        test_case
            .add_rule("files /**/*.py")
            .run()
            .assert_matches(vec!["a.py", "dir1/b.py", "dir1/dir2/c.py"]);
    }

    #[rstest]
    fn path_pattern_with_leading_slash_matched_at_root(mut test_case: TestCase) {
        test_case
            .add_rule("files /*.py")
            .run()
            .assert_matches(vec!["a.py"]);
    }

    #[rstest]
    fn path_pattern_matched_in_subdir(mut test_case: TestCase) {
        test_case
            .add_rule("files dir1/*.py")
            .run()
            .assert_matches(vec!["dir1/b.py"]);
    }

    #[rstest]
    fn path_pattern_with_leading_slash_matched_in_subdir(mut test_case: TestCase) {
        test_case
            .add_rule("files /dir1/*.py")
            .run()
            .assert_matches(vec!["dir1/b.py"]);
    }
}

#[test]
fn multiple_globs() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("a.txt", "")
        .add_file("a.rs", "")
        .add_rule("files *.py **/*.txt /*.rs")
        .run()
        .assert_matches(vec!["a.py", "a.txt", "a.rs"]);
}

#[test]
fn glob_exclusions() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_rule("files * ![bc]")
        .run()
        .assert_matches(vec!["a"]);
}

mod nofiles {
    use super::*;

    #[fixture]
    fn test_case() -> TestCase {
        let mut test_case = TestCase::new_for_json_tests();
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
            .add_rule("nofiles *.txt")
            .run()
            .assert_matches(vec![""]);
    }

    #[rstest]
    fn no_matches(mut test_case: TestCase) {
        test_case
            .add_rule("nofiles *.py ")
            .run()
            .assert_matches(vec![]);
    }
}

mod nofiles_multiple_globs {
    use super::*;

    #[test]
    fn should_not_match_if_any_pattern_matches() {
        TestCase::new_for_json_tests()
            .add_file("README", "")
            .add_rule("nofiles README README.txt")
            .run()
            .assert_matches(vec![]);
    }

    #[test]
    fn should_match_if_neither_pattern_matches() {
        TestCase::new_for_json_tests()
            .add_file("README", "")
            .add_rule("nofiles README.txt README.md")
            .run()
            .assert_matches(vec![""]);
    }
}

#[test]
fn nomatch() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "a\nb\n\nc\n")
        .add_file("b.py", "# SPDX-License-Identifier: GPLv3\na\nb\n\nc\n")
        .add_rule("files *.py\nnomatch /# SPDX-License-Identifier: GPLv3/")
        .run()
        .assert_matches(vec!["a.py"]);
}

mod tags {
    use super::*;

    #[test]
    fn required() {
        TestCase::new_for_json_tests()
            .add_file("a.py", "")
            .add_file("b.py", "")
            .add_arg("--tags=MYTAG")
            .add_rule("files a.py")
            .add_rule("tags MYTAG\nfiles b.py")
            .run()
            .assert_matches(vec!["b.py"]);
    }

    #[test]
    fn skipped() {
        TestCase::new_for_json_tests()
            .add_file("a.py", "")
            .add_file("b.py", "")
            .add_arg("--skip-tags=MYTAG")
            .add_rule("files a.py")
            .add_rule("tags MYTAG\nfiles b.py")
            .run()
            .assert_matches(vec!["a.py"]);
    }
}

#[test]
fn ignore_marker() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "foo\nbar  # omnilinter: ignore")
        .add_rule("files *.py\nmatch /foo|bar/")
        .run()
        .assert_matches(vec!["a.py:1"]);
}

mod error_exitcode {
    use super::*;

    #[test]
    fn zero() {
        TestCase::new_for_json_tests()
            .add_file("a.py", "")
            .add_arg("--error-exitcode=0")
            .add_rule("files *.py")
            .run()
            .assert_success();
    }

    #[test]
    fn nonzero() {
        TestCase::new_for_json_tests()
            .add_file("a.py", "")
            .add_arg("--error-exitcode=3")
            .add_rule("files *.py")
            .run()
            .assert_exit_code(3);
    }
}
