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
            .add_file("a.txt", lines!["foo", "bar", "baz"])
            .add_file("b.txt", lines!["foo", "bar", "baz"])
            .add_file("c.txt", lines!["foo", "bar", "baz"]);
        test_case
    }

    #[rstest]
    fn simple_matching_no_match(mut test_case: TestCase) {
        test_case
            .add_rule(lines!["files d.txt", "match /bar/"])
            .run()
            .assert_matches(vec![]);
    }

    #[rstest]
    fn simple_matching_single_match(mut test_case: TestCase) {
        test_case
            .add_rule(lines!["files a.txt", "match /bar/"])
            .run()
            .assert_matches(vec!["a.txt:2"]);
    }

    #[rstest]
    fn simple_matching_multiple_files(mut test_case: TestCase) {
        test_case
            .add_rule(lines!["files *.txt", "match /bar/"])
            .run()
            .assert_matches(vec!["a.txt:2", "b.txt:2", "c.txt:2"]);
    }

    #[rstest]
    fn simple_matching_multiple_lines(mut test_case: TestCase) {
        test_case
            .add_rule(lines!["files a.txt", "match /.../"])
            .run()
            .assert_matches(vec!["a.txt:1", "a.txt:2", "a.txt:3"]);
    }
}

#[test]
fn ignore_marker() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["foo", "bar  # omnilinter: ignore"])
        .add_rule(lines!["files *.py", "match /foo|bar/"])
        .run()
        .assert_matches(vec!["a.py:1"]);
}

#[test]
fn matches_multiple_patterns() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b", "c"])
        .add_rule(lines!["files *.py", "match /a/ /b/"])
        .run()
        .assert_matches(vec!["a.py:1", "a.py:2"]);
}

#[test]
fn multiple_match() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b"])
        .add_rule(lines!["files *.py", "match /a/", "match /b/"])
        .run()
        .assert_matches(vec!["a.py:2"]);

    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["c", "b"])
        .add_rule(lines!["files *.py", "match /a/", "match /b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn matches_exclusions() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b", "c"])
        .add_rule(lines!["files *.py", "match /./ !/^b/"])
        .run()
        .assert_matches(vec!["a.py:1", "a.py:3"]);
}

#[test]
fn nomatch_() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b", "", "c"])
        .add_file(
            "b.py",
            lines!["# SPDX-License-Identifier: GPLv3", "a", "b", "", "c"],
        )
        .add_rule(lines![
            "files *.py",
            "nomatch /# SPDX-License-Identifier: GPLv3/"
        ])
        .run()
        .assert_matches(vec!["a.py"]);
}

#[test]
fn match_before_nomatch_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b"])
        .add_rule(lines!["files *.py", "match /a/", "nomatch /b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
#[ignore] // should be fixed
fn match_before_nomatch_matching() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a"])
        .add_rule(lines!["files *.py", "match /a/", "nomatch /b/"])
        .run()
        .assert_matches(vec!["a.py:1"]);
}

#[test]
fn match_after_nomatch() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b"])
        .add_rule(lines!["files *.py", "match /a/", "nomatch /b/"])
        .run()
        .assert_matches(vec![]);
}
