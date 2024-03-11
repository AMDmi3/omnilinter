// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ruleset::enumerator::Enumerator;
use crate::ruleset::{ConditionLogic, Rule, Ruleset};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CompiledRuleset {
    pub rules: Vec<Rule>,
    pub conditions_count: usize,
    pub globs_count: usize,
    pub regexes_count: usize,
}

fn set_reporting_target(rule: &mut Rule) {
    if let Some(last_path_condition) = rule.path_conditions.last_mut() {
        if let Some(last_content_condition) = last_path_condition.content_conditions.last_mut() {
            if last_content_condition.logic == ConditionLogic::Positive {
                last_content_condition.is_reporting_target = true;
                return;
            }
        }
        if last_path_condition.logic == ConditionLogic::Positive {
            last_path_condition.is_reporting_target = true;
            return;
        }
    }
    rule.is_reporting_target = true;
}

fn enumerate_items(
    rule: &mut Rule,
    counter: &mut usize,
    glob_enumerator: &mut Enumerator,
    regex_enumerator: &mut Enumerator,
) {
    let mut count = || {
        let prev = *counter;
        *counter += 1;
        prev
    };

    rule.path_conditions.iter_mut().for_each(|path_condition| {
        path_condition.number = count();
        path_condition
            .patterns
            .iter_mut()
            .for_each(|glob| glob.enumerate_with(glob_enumerator));
        path_condition
            .excludes
            .iter_mut()
            .for_each(|glob| glob.enumerate_with(glob_enumerator));

        path_condition
            .content_conditions
            .iter_mut()
            .for_each(|content_condition| {
                content_condition.number = count();
                content_condition
                    .patterns
                    .iter_mut()
                    .for_each(|regex| regex.enumerate_with(regex_enumerator));
                content_condition
                    .excludes
                    .iter_mut()
                    .for_each(|regex| regex.enumerate_with(regex_enumerator));
            });
    });
}

impl Ruleset {
    pub fn compile(self) -> CompiledRuleset {
        let mut rules = self.rules;
        let mut conditions_count: usize = 0;
        let mut glob_enumerator = Enumerator::new();
        let mut regex_enumerator = Enumerator::new();

        rules
            .iter_mut()
            .enumerate()
            .for_each(|(rule_number, rule)| {
                rule.number = rule_number;
                enumerate_items(
                    rule,
                    &mut conditions_count,
                    &mut glob_enumerator,
                    &mut regex_enumerator,
                );
                set_reporting_target(rule);
            });

        CompiledRuleset {
            rules,
            conditions_count,
            globs_count: glob_enumerator.get_count(),
            regexes_count: regex_enumerator.get_count(),
        }
    }
}
