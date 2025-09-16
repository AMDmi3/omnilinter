// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{TestCase, lines, paths};

#[test]
fn simple() {
    TestCase::new_for_stdout_tests()
        .add_file("a.py", lines!["b", "a"])
        .add_rule(lines!["files *.py", "match /a/"])
        .run()
        .assert_stdout_contains("a.py:2");
}

#[test]
fn format_by_root() {
    TestCase::new_for_stdout_tests()
        .add_arg("--format=by-root")
        .add_file("file", lines!["line"])
        .add_named_rule("rootrule", "")
        .add_named_rule("filerule", lines!["files *"])
        .add_named_rule("linerule", lines!["files *", "match /./"])
        .run()
        .assert_stdout(lines![
            "root",
            "  rootrule",
            "  file: filerule",
            "  file:1: linerule"
        ]);
}

#[test]
fn format_full_paths() {
    TestCase::new_for_stdout_tests()
        .add_arg("--format=full-paths")
        .add_file("file", lines!["line"])
        .add_named_rule("rootrule", "")
        .add_named_rule("filerule", lines!["files *"])
        .add_named_rule("linerule", lines!["files *", "match /./"])
        .run()
        .assert_stdout(paths!(lines![
            "root: rootrule",
            "root/file: filerule",
            "root/file:1: linerule"
        ]));
}

#[test]
fn format_by_rule() {
    TestCase::new_for_stdout_tests()
        .add_arg("--format=by-rule")
        .add_file("file", lines!["line"])
        .add_named_rule("rootrule", "")
        .add_named_rule("filerule", lines!["files *"])
        .add_named_rule("linerule", lines!["files *", "match /./"])
        .run()
        .assert_stdout(paths!(lines![
            "rootrule",
            "  root",
            "filerule",
            "  root/file",
            "linerule",
            "  root/file:1"
        ]));
}

#[test]
fn format_by_path() {
    TestCase::new_for_stdout_tests()
        .add_arg("--format=by-path")
        .add_file("file", lines!["line"])
        .add_named_rule("rootrule", "")
        .add_named_rule("filerule", lines!["files *"])
        .add_named_rule("linerule", lines!["files *", "match /./"])
        .run()
        .assert_stdout(paths!(lines![
            "root",
            "  rootrule",
            "root/file",
            "  filerule",
            "  line 1: linerule"
        ]));
}
