// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{lines, TestCase};

#[test]
fn includes() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_file("b.py", "")
        .add_file("c.py", "")
        .add_raw_file(
            "omnilinter.conf",
            lines!["include subdir/second.conf", "[first]", "files a.py"],
        )
        .add_raw_file(
            "subdir/second.conf",
            lines!["include ../third.conf", "[second]", "files b.py"],
        )
        .add_raw_file("third.conf", lines!["[third]", "files c.py"])
        .run()
        .assert_matches(vec!["a.py", "b.py", "c.py"]);
}

#[test]
fn include_loop() {
    TestCase::new_for_json_tests()
        .add_file("a.py", "")
        .add_raw_file("omnilinter.conf", lines!["include second.conf"])
        .add_raw_file("second.conf", lines!["include omnilinter.conf"])
        .run()
        .assert_failure(); //.assert_stderr_contains("failed to parse config file omnilinter.conf")
                           //.assert_stderr_contains("--> omnilinter.conf");
}
