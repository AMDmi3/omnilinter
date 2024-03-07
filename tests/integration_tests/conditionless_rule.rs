// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::utils::TestCase;

#[test]
fn conditionless_rule() {
    TestCase::new_for_json_tests()
        .add_rule("")
        .run()
        .assert_matches(vec![""]);
}
