// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use super::regex::Regex;

#[derive(Default, Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct RegexCondition {
    pub patterns: Vec<Regex>,
    pub excludes: Vec<Regex>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SizeOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
}

#[derive(Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct SizeCondition {
    pub operator: SizeOperator,
    pub value: u64,
}

impl SizeCondition {
    pub fn check(&self, value: u64) -> bool {
        match self.operator {
            SizeOperator::GreaterEqual => value >= self.value,
            SizeOperator::Greater => value > self.value,
            SizeOperator::LessEqual => value <= self.value,
            SizeOperator::Less => value < self.value,
            SizeOperator::Equal => value == self.value,
            SizeOperator::NotEqual => value != self.value,
        }
    }

    pub fn check_for_this_and_above(&self, value: u64) -> bool {
        match self.operator {
            SizeOperator::GreaterEqual => value >= self.value,
            SizeOperator::Greater => value > self.value,
            SizeOperator::LessEqual => false,
            SizeOperator::Less => false,
            SizeOperator::Equal => false,
            SizeOperator::NotEqual => value > self.value,
        }
    }
}

#[derive(Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub enum ContentCondition {
    Match(RegexCondition),
    NoMatch(RegexCondition),
    Size(SizeCondition),
    Lines(SizeCondition),
}

#[derive(Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct ContentConditionNode {
    pub number: usize,
    pub is_reporting_target: bool,
    pub condition: ContentCondition,
}

impl ContentConditionNode {
    pub fn new(condition: ContentCondition) -> Self {
        ContentConditionNode {
            number: Default::default(),
            is_reporting_target: Default::default(),
            condition: condition,
        }
    }
}
