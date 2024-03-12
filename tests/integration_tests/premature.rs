// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{lines, TestCase};

// check that negative conditions still apply, even if positive
// conditions are satisfied earlier

#[test]
fn nofiles() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_file("c", "")
        .add_rule(lines!["nofiles c", "files a"])
        .run()
        .assert_matches(vec![]);
}

#[test]
fn nomatch() {
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "b", "c"])
        .add_rule(lines!["files a", "nomatch /c/", "match /a/"])
        .run()
        .assert_matches(vec![]);
}
