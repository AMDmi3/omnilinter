// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{lines, TestCase};

#[test]
fn files_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_rule(lines!["files a"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn files_satisfied_multiple() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_rule(lines!["files a b c"])
        .run()
        .assert_matches(vec!["a", "b"]);
}

#[test]
fn files_not_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_rule(lines!["files b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nofiles_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_rule(lines!["nofiles a"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nofiles_satisfied_multiple() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_rule(lines!["nofiles a b c"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nofiles_not_satisfied() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_rule(lines!["nofiles b"])
        .run()
        .assert_matches(vec![""]);
}

#[test]
fn nofiles_not_satisfied_multiple() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_rule(lines!["nofiles b c d"])
        .run()
        .assert_matches(vec![""]);
}
