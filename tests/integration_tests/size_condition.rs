// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{lines, TestCase};

#[test]
fn greater_equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size >= 10"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn greater_equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size >= 11"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn greater_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size > 9"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn greater_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size > 10"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn less_equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size <= 10"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn less_equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size <= 9"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn less_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size < 11"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn less_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size < 10"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size == 10"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size == 11"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn not_equal_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size != 11"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn not_equal_not_matching() {
    TestCase::new_for_json_tests()
        .add_file("a", "0123456789")
        .add_rule(lines!["files a", "size != 10"])
        .run()
        .assert_matches(vec![]);
}

#[test]
#[ignore]
fn binary_file() {
    TestCase::new_for_json_tests()
        .add_binary_file("a", &[255u8, 255u8, 255u8, 255u8])
        .add_rule(lines!["files a", "size = 4"])
        .run()
        .assert_matches(vec!["a"]);
}
