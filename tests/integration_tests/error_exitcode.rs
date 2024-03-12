// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::TestCase;

#[test]
fn default_matching() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_arg("--error-exitcode=0")
        .add_rule("files *.py")
        .run()
        .assert_success();
}

#[test]
fn default_non_matching() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_arg("--error-exitcode=0")
        .add_rule("files *.txt")
        .run()
        .assert_success();
}

#[test]
fn zero() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_arg("--error-exitcode=0")
        .add_rule("files *.py")
        .run()
        .assert_success();
}

#[test]
fn nonzero() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_arg("--error-exitcode=3")
        .add_rule("files *.py")
        .run()
        .assert_exit_code(3);
}
