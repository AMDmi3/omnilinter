// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use utils::TestCase;

mod tags {
    use super::*;

    #[test]
    fn required() {
        TestCase::new_for_json_tests()
            .add_file("a.py", "")
            .add_file("b.py", "")
            .add_arg("--tags=MYTAG")
            .add_rule("files a.py")
            .add_rule("tags MYTAG\nfiles b.py")
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
            .add_rule("tags MYTAG\nfiles b.py")
            .run()
            .assert_matches(vec!["a.py"]);
    }
}

mod error_exitcode {
    use super::*;

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
}

mod stdout {
    use super::*;

    #[test]
    fn simple() {
        TestCase::new_for_stdout_tests()
            .add_file("a.py", "b\na")
            .add_rule("files *.py\nmatch /a/")
            .run()
            .assert_stdout_contains("a.py:2");
    }
}

mod parsing {
    use super::*;

    #[test]
    fn multiple_conditions() {
        TestCase::new_for_stdout_tests()
            .add_rule("files *.py\nfiles *.py")
            .run()
            .assert_failure();
    }
}
