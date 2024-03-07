// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use rstest::{fixture, rstest};
use utils::TestCase;

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

    #[rstest]
    fn matches_at_end(mut test_case: TestCase) {
        // the point is to check that despite nofiles doesn't match on
        // the first file (a.py), it should still match on the last file
        test_case
            .add_rule("nofiles c.py")
            .run()
            .assert_matches(vec![]);
    }
}

#[test]
fn files_before_nofiles() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("b.py", "")
        .add_rule(lines!["files a.py", "nofiles b.py"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn files_with_content_before_nofiles() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "a")
        .add_file("b.py", "")
        .add_rule(lines!["files a.py", "match /a/", "nofiles b.py"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn files_after_files_order_a() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("b.py", "")
        .add_rule(lines!["files a.py", "files b.py"])
        .run()
        .assert_matches(vec!["b.py"]);
}

#[test]
fn files_after_files_order_b() {
    TestCase::new_for_json_tests()
        .add_file("b.py", "")
        .add_file("a.py", "")
        .add_rule(lines!["files a.py", "files b.py"])
        .run()
        .assert_matches(vec!["b.py"]);
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
