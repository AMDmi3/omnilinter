// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod compile;

pub use regex::Regex;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Debug)]
enum GlobScope {
    Filenames,
    Paths,
}

pub struct Glob {
    pattern: glob::Pattern,
    scope: GlobScope,
}

impl Hash for Glob {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pattern.as_str().hash(state);
        self.scope.hash(state);
    }
}

impl PartialEq for Glob {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.as_str() == other.pattern.as_str() && self.scope == other.scope
    }
}

impl Eq for Glob {}

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
            pattern: glob::Pattern::new(pattern.trim_start_matches(std::path::MAIN_SEPARATOR))?,
            scope: if pattern.contains(std::path::MAIN_SEPARATOR_STR) {
                GlobScope::Paths
            } else {
                GlobScope::Filenames
            },
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
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ConditionLogic {
    #[default]
    Positive,
    Negative,
}

#[derive(Default, Debug)]
pub struct GlobCondition {
    pub number: usize,
    pub logic: ConditionLogic,
    pub patterns: Vec<Glob>,
    pub excludes: Vec<Glob>,
    pub content_conditions: Vec<RegexCondition>,
    pub is_reporting_target: bool,
}

impl GlobCondition {
    pub fn are_all_positive_conditions_satisfied(&self, mask: &[bool]) -> bool {
        !self
            .content_conditions
            .iter()
            .any(|condition| condition.logic == ConditionLogic::Positive && !mask[condition.number])
    }
}

#[derive(Default, Debug)]
pub struct RegexCondition {
    pub number: usize,
    pub logic: ConditionLogic,
    pub patterns: Vec<Regex>,
    pub excludes: Vec<Regex>,
    pub is_reporting_target: bool,
}

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
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
