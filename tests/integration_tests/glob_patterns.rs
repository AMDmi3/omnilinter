// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::TestCase;

#[test]
fn multiple_patterns() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("a.txt", "")
        .add_file("a.rs", "")
        .add_rule("files *.py **/*.txt /*.rs")
        .run()
        .assert_matches(vec!["a.py", "a.txt", "a.rs"]);
}

#[test]
fn exclusion_patterns() {
    TestCase::new_for_json_tests()
        .add_file("a", "")
        .add_file("b", "")
        .add_rule("files * ![bc]")
        .run()
        .assert_matches(vec!["a"]);
}
