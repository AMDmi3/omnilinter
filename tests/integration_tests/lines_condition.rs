// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{TestCase, lines};

#[test]
fn line_counting() {
    // 3 lines regardless of trailing newline
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc")
        .add_rule(lines!["files a", "lines == 3"])
        .run()
        .assert_matches(vec!["a"]);
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines == 3"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn greater_equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines >= 3"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn greater_equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines >= 4"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn greater_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines > 2"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn greater_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines > 3"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn less_equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines <= 3"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn less_equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines <= 2"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn less_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines < 4"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn less_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines < 3"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines == 3"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines == 4"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn not_equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines != 4"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn not_equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "a\nb\nc\n")
        .add_rule(lines!["files a", "lines != 3"])
        .run()
        .assert_matches(vec![]);
}
