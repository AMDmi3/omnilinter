// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use super::path_conditions::{ConditionLogic, GlobCondition};
use std::collections::HashSet;

#[derive(Default)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct Rule {
    pub number: usize,
    pub title: String,
    pub tags: HashSet<String>,
    pub path_conditions: Vec<GlobCondition>,
    pub is_reporting_target: bool,
}

fn prepend_to_vec<T: Clone>(target: &mut Vec<T>, source: Vec<T>) {
    let mut tmp = source.clone();
    std::mem::swap(target, &mut tmp);
    target.extend(tmp);
}

impl Rule {
    pub fn apply_template(&mut self, template: &Rule) {
        template.tags.iter().for_each(|tag| {
            self.tags.insert(tag.clone());
        });
        prepend_to_vec(&mut self.path_conditions, template.path_conditions.clone());
    }

    pub fn are_all_positive_conditions_satisfied(&self, mask: &[bool]) -> bool {
        !self
            .path_conditions
            .iter()
            .any(|condition| condition.logic == ConditionLogic::Positive && !mask[condition.number])
    }
}
