// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{TestCase, lines};

fn check_parsing_error(rule: &str) {
    TestCase::new_for_json_tests()
        .add_raw_rule(rule)
        .silence_stderr() // comment to see omnilinter outputs for all kinds of parsing errors
        .run()
        .assert_failure()
        // here we basically check that a readable error is reported and not a panic,
        // and we also check that config file is properly fille both in the outer
        // error and in the inner pest error
        .assert_stderr_contains("failed to parse config file omnilinter.conf")
        .assert_stderr_contains("--> omnilinter.conf");
}

// top-level config structures

#[test]
fn garbage_at_start() {
    check_parsing_error(lines!["xxx"]);
}

#[test]
fn root_without_args() {
    check_parsing_error(lines!["root"]);
}

#[test]
fn root_with_incorrect_args() {
    check_parsing_error(lines!["root a b"]);
}

#[test]
fn root_with_invalid_glob_syntax() {
    check_parsing_error(lines!["root ***"]);
}

// in rule

#[test]
fn unclosed_title() {
    check_parsing_error(lines!["[rule"]);
}

#[test]
fn garbage_in_rule() {
    check_parsing_error(lines!["[rule]", "xxx"]);
}

#[test]
fn tags_without_args() {
    check_parsing_error(lines!["[rule]", "tags"]);
}

#[test]
fn tags_with_incorrect_args() {
    check_parsing_error(lines!["[rule]", "tags foo,"]);
}

#[test]
fn files_without_args() {
    check_parsing_error(lines!["[rule]", "files"]);
}

#[test]
fn files_with_invalid_pattern_syntax() {
    check_parsing_error(lines!["[rule]", "files ***"]);
}

#[test]
fn files_with_exclusion_pattern_only() {
    check_parsing_error(lines!["[rule]", "files !*"]);
}

#[test]
fn files_with_pattern_after_exclusion() {
    check_parsing_error(lines!["[rule]", "files * !* *"]);
}

#[test]
fn nofiles_with_content_conditions() {
    check_parsing_error(lines!["[rule]", "nofiles *", "match /./"]);
}

// in rule->files

#[test]
fn garbage_in_files() {
    check_parsing_error(lines!["[rule]", "files *", "xxx"]);
}

#[test]
fn match_without_args() {
    check_parsing_error(lines!["[rule]", "files *", "match"]);
}

#[test]
fn match_with_invalid_args_syntax() {
    check_parsing_error(lines!["[rule]", "files *", "match //"]);
}

#[test]
fn match_with_invalid_pattern_syntax() {
    check_parsing_error(lines!["[rule]", "files *", "match /[/"]);
}

#[test]
fn match_with_exclusion_pattern_only() {
    check_parsing_error(lines!["[rule]", "files *", "match !/./"]);
}

#[test]
fn match_with_pattern_after_exclusion() {
    check_parsing_error(lines!["[rule]", "files *", "match /./ !/./ /./"]);
}
