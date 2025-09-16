// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{TestCase, lines};

#[test]
fn last_match() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a"])
        .add_file("b.py", lines!["b"])
        .add_rule(lines![
            "files a.py",
            "match /a/",
            "nofiles nonexisting",
            "files b.py",
            "nomatch /nonexisting/",
            "match /b/"
        ])
        .run()
        .assert_matches(vec!["b.py:1"]);
}

#[test]
fn non_last_match() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a"])
        .add_file("b.py", lines!["b"])
        .add_rule(lines![
            "files a.py",
            "match /a/",
            "nofiles nonexisting",
            "files b.py",
            "match /b/",
            "nomatch /nonexisting/"
        ])
        .run()
        .assert_matches(vec!["b.py"]);
}

#[test]
fn last_files() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a"])
        .add_file("b.py", lines!["b"])
        .add_rule(lines![
            "files a.py",
            "match /a/",
            "nofiles nonexisting",
            "files b.py"
        ])
        .run()
        .assert_matches(vec!["b.py"]);
}

#[test]
fn non_last_files() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a"])
        .add_file("b.py", lines!["b"])
        .add_rule(lines![
            "files a.py",
            "match /a/",
            "files b.py",
            "nofiles nonexisting"
        ])
        .run()
        .assert_matches(vec![""]);
}
