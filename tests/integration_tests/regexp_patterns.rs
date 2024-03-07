// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::lines;
use crate::utils::TestCase;

#[test]
fn multiple_patterns() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b", "c"])
        .add_rule(lines!["files *.py", "match /a/ /b/"])
        .run()
        .assert_matches(vec!["a.py:1", "a.py:2"]);
}

#[test]
fn exclusion_patterns() {
    TestCase::new_for_json_tests()
        .add_file("a.py", lines!["a", "b", "c"])
        .add_rule(lines!["files *.py", "match /./ !/^b/"])
        .run()
        .assert_matches(vec!["a.py:1", "a.py:3"]);
}
