// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::lines;
use crate::utils::TestCase;

#[test]
fn match_without_marker() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["foo", "bar"])
        .add_rule(lines!["files *.py", "match /foo|bar/"])
        .run()
        .assert_matches(vec!["a.py:1", "a.py:2"]);
}

#[test]
fn match_with_marker() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["foo", "bar  # omnilinter: ignore"])
        .add_rule(lines!["files *.py", "match /foo|bar/"])
        .run()
        .assert_matches(vec!["a.py:1"]);
}

#[test]
fn nomatch_without_marker() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["foo", "bar"])
        .add_rule(lines!["files *.py", "nomatch /bar/"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch_with_marker() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["foo", "bar  # omnilinter: ignore"])
        .add_rule(lines!["files *.py", "nomatch /bar/"])
        .run()
        .assert_matches(vec!["a.py"]);
}
