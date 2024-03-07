// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::lines;
use crate::utils::TestCase;

#[test]
fn required() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("b.py", "")
        .add_arg("--tags=MYTAG")
        .add_rule("files a.py")
        .add_rule(lines!["tags MYTAG", "files b.py"])
        .run()
        .assert_matches(vec!["b.py"]);
}

#[test]
fn skipped() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("b.py", "")
        .add_arg("--skip-tags=MYTAG")
        .add_rule("files a.py")
        .add_rule(lines!["tags MYTAG", "files b.py"])
        .run()
        .assert_matches(vec!["a.py"]);
}

#[test]
#[ignore] // TODO, issue 45
fn lowercase() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("b.py", "")
        .add_arg("--tags=mytag")
        .add_rule("files a.py")
        .add_rule(lines!["tags MYTAG", "files b.py"])
        .run()
        .assert_matches(vec!["b.py"]);
}
