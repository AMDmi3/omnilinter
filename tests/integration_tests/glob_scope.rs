// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use rstest::{fixture, rstest};
use testutils::{TestCase, paths};

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
        paths!("dir1/b.py"),
        paths!("dir1/dir2/c.py"),
    ]);
}

#[rstest]
fn path_pattern_matched_everywhere(mut test_case: TestCase) {
    test_case
        .add_rule("files **/*.py")
        .run()
        .assert_matches(vec!["a.py", paths!("dir1/b.py"), paths!("dir1/dir2/c.py")]);
}

#[rstest]
fn path_pattern_with_leading_slash_matched_everywhere(mut test_case: TestCase) {
    test_case
        .add_rule("files /**/*.py")
        .run()
        .assert_matches(vec!["a.py", paths!("dir1/b.py"), paths!("dir1/dir2/c.py")]);
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
        .assert_matches(vec![paths!("dir1/b.py")]);
}

#[rstest]
fn path_pattern_with_leading_slash_matched_in_subdir(mut test_case: TestCase) {
    test_case
        .add_rule("files /dir1/*.py")
        .run()
        .assert_matches(vec![paths!("dir1/b.py")]);
}

#[rstest]
fn path_pattern_different_scope_bug(mut test_case: TestCase) {
    // internally, there are two different glob patterns here, "b.py" with
    // path match and "b.py" with filename match; due to a bug in how unique
    // ids for these are generated, these may be aliased as a same patter,
    // negative match for /b.py cached, and b.py match not accounted
    test_case
        .add_rule("files /b.py")
        .add_rule("files b.py")
        .run()
        .assert_matches(vec![paths!("dir1/b.py")]);
}
