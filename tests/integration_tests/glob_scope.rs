// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::utils::TestCase;
use rstest::{fixture, rstest};

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
