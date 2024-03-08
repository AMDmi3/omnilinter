// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ruleset::{ConditionLogic, Rule, Ruleset};

#[derive(Debug)]
pub struct CompiledRuleset {
    pub rules: Vec<Rule>,
    pub conditions_count: usize,
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

fn enumerate_conditions(rule: &mut Rule, counter: &mut usize) {
    let mut count = || {
        let prev = *counter;
        *counter += 1;
        prev
    };

    rule.path_conditions.iter_mut().for_each(|path_condition| {
        path_condition.number = count();

        path_condition
            .content_conditions
            .iter_mut()
            .for_each(|content_condition| {
                content_condition.number = count();
            });
    });
}

impl Ruleset {
    pub fn compile(self) -> CompiledRuleset {
        let mut rules = self.rules;
        let mut conditions_count: usize = 0;

        rules
            .iter_mut()
            .enumerate()
            .for_each(|(rule_number, rule)| {
                rule.number = rule_number;
                enumerate_conditions(rule, &mut conditions_count);
                set_reporting_target(rule);
            });

        CompiledRuleset {
            rules,
            conditions_count,
        }
    }
}
