// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::lines;
use crate::utils::TestCase;

#[test]
fn multiple_match_all_match() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "b"])
        .add_rule(lines!["files a", "match /^a/", "match /^b/"])
        .run()
        .assert_matches(vec!["a:2"]);
}

#[test]
fn multiple_match_not_all_match_a() {
    TestCase::new_for_json_tests()
        .add_file("a", "a")
        .add_rule(lines!["files a", "match /^a/", "match /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn multiple_match_not_all_match_b() {
    TestCase::new_for_json_tests()
        .add_file("a", "b")
        .add_rule(lines!["files a", "match /^a/", "match /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn multiple_nomatch_none_match() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["not_a", "not_b"])
        .add_rule(lines!["files a", "nomatch /^a/", "nomatch /^b/"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn multiple_nomatch_some_match_a() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["not_a", "b"])
        .add_rule(lines!["files a", "nomatch /^a/", "nomatch /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn multiple_nomatch_some_match_b() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "not_b"])
        .add_rule(lines!["files a", "nomatch /^a/", "nomatch /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn match_after_nomatch_a() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "b"])
        .add_rule(lines!["files a", "nomatch /^a/", "match /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn match_after_nomatch_b() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["not_a", "b"])
        .add_rule(lines!["files a", "nomatch /^a/", "match /^b/"])
        .run()
        .assert_matches(vec!["a:2"]);
}

#[test]
fn match_after_nomatch_c() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "not_b"])
        .add_rule(lines!["files a", "nomatch /^a/", "match /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn match_after_nomatch_d() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["not_a", "not_b"])
        .add_rule(lines!["files a", "nomatch /^a/", "match /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch_after_match_a() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "b"])
        .add_rule(lines!["files a", "match /^a/", "nomatch /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch_after_match_b() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["not_a", "b"])
        .add_rule(lines!["files a", "match /^a/", "nomatch /^b/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch_after_match_c() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "not_b"])
        .add_rule(lines!["files a", "match /^a/", "nomatch /^b/"])
        .run()
        .assert_matches(vec!["a"]);
}

#[test]
fn nomatch_after_match_d() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["not_a", "not_b"])
        .add_rule(lines!["files a", "match /^a/", "nomatch /^b/"])
        .run()
        .assert_matches(vec![]);
}