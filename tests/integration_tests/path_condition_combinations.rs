// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::lines;
use crate::utils::TestCase;

#[test]
fn multiple_files_all_match() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_rule(lines!["files a", "files b"])
        .run()
        .assert_matches(vec!["b"]);
}

#[test]
fn multiple_files_not_all_match_a() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_rule(lines!["files a", "files b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn multiple_files_not_all_match_b() {
    TestCase::new_for_json_tests()
        .add_file("b", "")
        .add_rule(lines!["files a", "files b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn multiple_files_shared() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_rule(lines!["files *", "files *"])
        .run()
        .assert_matches(vec!["a", "b"]);
}

#[test]
fn multiple_nofiles_none_match() {
    TestCase::new_for_json_tests()
        .add_file("not_a", "")
        .add_file("not_b", "")
        .add_rule(lines!["nofiles a", "nofiles b"])
        .run()
        .assert_matches(vec![""]);
}

#[test]
fn multiple_nofiles_some_match_a() {
    TestCase::new_for_json_tests()
        .add_file("not_a", "")
        .add_file("b", "")
        .add_rule(lines!["nofiles a", "nofiles b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn multiple_nofiles_some_match_b() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("not_b", "")
        .add_rule(lines!["nofiles a", "nofiles b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn files_after_nofiles_a() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_rule(lines!["nofiles a", "files b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn files_after_nofiles_b() {
    TestCase::new_for_json_tests()
        .add_file("not_a", "")
        .add_file("b", "")
        .add_rule(lines!["nofiles a", "files b"])
        .run()
        .assert_matches(vec!["b"]);
}

#[test]
fn files_after_nofiles_c() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("not_b", "")
        .add_rule(lines!["nofiles a", "files b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn files_after_nofiles_d() {
    TestCase::new_for_json_tests()
        .add_file("not_a", "")
        .add_file("not_b", "")
        .add_rule(lines!["nofiles a", "files b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nofiles_after_files_a() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_rule(lines!["files a", "nofiles b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nofiles_after_files_b() {
    TestCase::new_for_json_tests()
        .add_file("not_a", "")
        .add_file("b", "")
        .add_rule(lines!["files a", "nofiles b"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nofiles_after_files_c() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("not_b", "")
        .add_rule(lines!["files a", "nofiles b"])
        .run()
        .assert_matches(vec![""]);
}

#[test]
fn nofiles_after_files_d() {
    TestCase::new_for_json_tests()
        .add_file("not_a", "")
        .add_file("not_b", "")
        .add_rule(lines!["files a", "nofiles b"])
        .run()
        .assert_matches(vec![]);
}
