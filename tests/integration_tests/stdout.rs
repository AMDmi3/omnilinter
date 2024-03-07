// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::lines;
use crate::utils::TestCase;

#[test]
fn simple() {
    TestCase::new_for_stdout_tests()
        .add_file("a.py", lines!["b", "a"])
        .add_rule(lines!["files *.py", "match /a/"])
        .run()
        .assert_stdout_contains("a.py:2");
}
