// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use testutils::{lines, TestCase};

#[test]
fn mixing_up_matches_bug() {
    // test for actual bug where rules produced false matches from another rules
    // which produced vec!["a:1", "a:1", "a:2", "a:2"] matches
    TestCase::new_for_json_tests()
        .add_file("a", lines!["a", "b"])
        .add_rule(lines!["files *", "match /a/"])
        .add_rule(lines!["files *", "match /b/"])
        .run()
        .assert_matches(vec!["a:1", "a:2"]);
}
