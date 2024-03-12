// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{lines, TestCase};

#[test]
fn match_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "a")
        .add_rule(lines!["files a", "match /a/"])
        .run()
        .assert_matches(vec!["a:1"]);
}

#[test]
fn match_satisfied_multiple() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "b", "c"])
        .add_rule(lines!["files a", "match /a/ /b/ /d/"])
        .run()
        .assert_matches(vec!["a:1", "a:2"]);
}

#[test]
fn match_satisfied_multiple_files() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "b", "c"])
        .add_file("b", lines!["a", "b", "c"])
        .add_rule(lines!["files *", "match /a/ /b/ /d/"])
        .run()
        .assert_matches(vec!["a:1", "a:2", "b:1", "b:2"]);
}

#[test]
fn match_not_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "a")
        .add_rule(lines!["files a", "match /b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "a")
        .add_rule(lines!["files a", "nomatch /a/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch_satisfied_multiple() {
    TestCase::new_for_json_tests()
        .add_file("a", "a")
        .add_rule(lines!["files a", "nomatch /a/ /b/ /c/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch_not_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "a")
        .add_rule(lines!["files a", "nomatch /b/"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn nomatch_not_satisfied_multiple() {
    TestCase::new_for_json_tests()
        .add_file("a", "a")
        .add_rule(lines!["files a", "nomatch /b/ /c/ /d/"])
        .run()
        .assert_matches(vec!["a"]);
}
