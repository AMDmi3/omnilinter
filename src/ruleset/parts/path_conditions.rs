// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use super::content_conditions::{ContentCondition, ContentConditionNode};
use super::glob::Glob;

#[derive(Default, PartialEq, Eq, Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub enum ConditionLogic {
    #[default]
    Positive,
    Negative,
}

#[derive(Default, Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct GlobCondition {
    pub number: usize,
    pub logic: ConditionLogic,
    pub patterns: Vec<Glob>,
    pub excludes: Vec<Glob>,
    pub content_conditions: Vec<ContentConditionNode>,
    pub linewise_content_conditions_count: usize,
    pub is_reporting_target: bool,
    pub has_reporting_target: bool,
}

impl GlobCondition {
    pub fn are_all_positive_conditions_satisfied(&self, mask: &[bool]) -> bool {
        !self
            .content_conditions
            .iter()
            .any(|condition_node| match condition_node.condition {
                ContentCondition::Match(_) => !mask[condition_node.number],
                _ => false,
            })
    }
}
