// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod compile;
pub mod enumerator;
pub mod parts;

use std::collections::HashSet;

pub use parts::*;

#[derive(Default)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct Ruleset {
    pub rules: Vec<Rule>,
}

impl Ruleset {
    pub fn filter_by_tags(
        &mut self,
        required_tags: &HashSet<String>,
        ignored_tags: &HashSet<String>,
    ) {
        self.rules.retain(|rule| {
            rule.tags.is_disjoint(ignored_tags)
                && (required_tags.is_empty() || !rule.tags.is_disjoint(required_tags))
        })
    }
}
