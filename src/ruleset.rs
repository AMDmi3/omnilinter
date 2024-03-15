// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod compile;
pub mod enumerator;

use crate::ruleset::enumerator::Enumerator;
use std::collections::HashSet;
use std::path::Path;

#[derive(PartialEq, Eq)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
enum GlobScope {
    Filenames,
    Paths,
}

pub struct Glob {
    pattern: glob::Pattern,
    scope: GlobScope,
    unique_id: usize,
}

#[cfg(not(feature = "coverage"))]
impl std::fmt::Debug for Glob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Glob")
            .field("pattern", &self.pattern.as_str())
            .field("scope", &self.scope)
            .finish()
    }
}

impl Glob {
    pub fn new(pattern: &str) -> Result<Self, glob::PatternError> {
        Ok(Self {
            pattern: glob::Pattern::new(pattern.trim_start_matches(std::path::is_separator))?,
            scope: if pattern.chars().any(std::path::is_separator) {
                GlobScope::Paths
            } else {
                GlobScope::Filenames
            },
            unique_id: usize::MAX,
        })
    }

    pub fn matches_path_with(&self, path: &Path, options: glob::MatchOptions) -> bool {
        self.pattern.matches_path_with(
            match self.scope {
                GlobScope::Paths => path,
                GlobScope::Filenames => Path::new(
                    path.file_name()
                        .expect("valid path is expected when matching"),
                ),
            },
            options,
        )
    }

    pub fn as_str(&self) -> &str {
        self.pattern.as_str()
    }

    pub fn enumerate_with(&mut self, enumerator: &mut Enumerator) {
        self.unique_id = enumerator.get_id(self.pattern.as_str());
    }

    #[cfg_attr(not(feature = "matching-cache"), allow(dead_code))]
    pub fn get_unique_id(&self) -> usize {
        debug_assert!(self.unique_id != usize::MAX, "Glob is not enumerated");
        self.unique_id
    }
}

#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct Regex {
    regex: regex::Regex,
    unique_id: usize,
}

impl Regex {
    pub fn new(re: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: regex::Regex::new(re)?,
            unique_id: usize::MAX,
        })
    }

    pub fn is_match(&self, haystack: &str) -> bool {
        self.regex.is_match(haystack)
    }

    pub fn as_str(&self) -> &str {
        self.regex.as_str()
    }

    pub fn enumerate_with(&mut self, enumerator: &mut Enumerator) {
        self.unique_id = enumerator.get_id(self.regex.as_str());
    }

    #[cfg_attr(not(feature = "matching-cache"), allow(dead_code))]
    pub fn get_unique_id(&self) -> usize {
        debug_assert!(self.unique_id != usize::MAX, "Regex is not enumerated");
        self.unique_id
    }
}

#[derive(Default, PartialEq, Eq)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub enum ConditionLogic {
    #[default]
    Positive,
    Negative,
}

#[derive(Default)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct GlobCondition {
    pub number: usize,
    pub logic: ConditionLogic,
    pub patterns: Vec<Glob>,
    pub excludes: Vec<Glob>,
    pub content_conditions: Vec<ContentConditionNode>,
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

#[derive(Default)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct RegexCondition {
    pub patterns: Vec<Regex>,
    pub excludes: Vec<Regex>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SizeOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
}

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
}

#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub enum ContentCondition {
    Match(RegexCondition),
    NoMatch(RegexCondition),
    Size(SizeCondition),
    Lines(SizeCondition),
}

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

#[derive(Default)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
pub struct Rule {
    pub number: usize,
    pub title: String,
    pub tags: HashSet<String>,
    pub path_conditions: Vec<GlobCondition>,
    pub is_reporting_target: bool,
}

impl Rule {
    pub fn are_all_positive_conditions_satisfied(&self, mask: &[bool]) -> bool {
        !self
            .path_conditions
            .iter()
            .any(|condition| condition.logic == ConditionLogic::Positive && !mask[condition.number])
    }
}

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
