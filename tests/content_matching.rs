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

#[test]
fn ignore_marker() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "foo\nbar  # omnilinter: ignore")
        .add_rule("files *.py\nmatch /foo|bar/")
        .run()
        .assert_matches(vec!["a.py:1"]);
}

#[test]
fn matches_multiple_patterns() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "a\nb\nc")
        .add_rule("files *.py\nmatch /a/ /b/")
        .run()
        .assert_matches(vec!["a.py:1", "a.py:2"]);
}

#[test]
fn matches_exclusions() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "a\nb\nc")
        .add_rule("files *.py\nmatch /./ !/^b/")
        .run()
        .assert_matches(vec!["a.py:1", "a.py:3"]);
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
