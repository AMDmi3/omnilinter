// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

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
}

#[derive(Default, Debug)]
pub struct GlobCondition {
    pub number: usize,
    pub patterns: Vec<Glob>,
    pub excludes: Vec<Glob>,
    pub match_: Vec<RegexCondition>,
    pub nomatch: Vec<RegexCondition>,
    pub is_reporting_target: bool,
}

#[derive(Default, Debug)]
pub struct RegexCondition {
    pub number: usize,
    pub patterns: Vec<Regex>,
    pub excludes: Vec<Regex>,
    pub is_reporting_target: bool,
}

#[derive(Default, Debug)]
pub struct Rule {
    pub number: usize,
    pub title: String,
    pub tags: HashSet<String>,
    pub files: Vec<GlobCondition>,
    pub nofiles: Vec<GlobCondition>,
    pub is_reporting_target: bool,
}

#[derive(Default, Debug)]
pub struct Ruleset {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct CompiledRuleset {
    pub rules: Vec<Rule>,
    pub conditions_count: usize,
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

                let mut last_glob_cond: Option<usize> = None;
                let mut last_files_cond: Option<usize> = None;
                let mut last_match_cond: Option<usize> = None;

                rule.files.iter_mut().for_each(|condition| {
                    condition.number = conditions_count;
                    last_glob_cond = Some(conditions_count);
                    last_files_cond = Some(conditions_count);
                    conditions_count += 1;

                    condition.match_.iter_mut().for_each(|condition| {
                        condition.number = conditions_count;
                        last_match_cond = Some(conditions_count);
                        conditions_count += 1;
                    });
                    condition.nomatch.iter_mut().for_each(|condition| {
                        condition.number = conditions_count;
                        conditions_count += 1;
                    });
                });
                rule.nofiles.iter_mut().for_each(|condition| {
                    condition.number = conditions_count;
                    last_glob_cond = Some(conditions_count);
                    conditions_count += 1;
                });

                let reporting_target_condition_number = if let Some(last_match_cond) =
                    last_match_cond
                    && last_match_cond == conditions_count - 1
                {
                    // match which is the last condition
                    last_match_cond
                } else if let (Some(last_files_cond), Some(last_glob_cond)) =
                    (last_files_cond, last_glob_cond)
                    && last_files_cond == last_glob_cond
                {
                    // files which is the last glob condition
                    last_glob_cond
                } else {
                    // otherwise reports at root level
                    rule.is_reporting_target = true;
                    return;
                };

                rule.files.iter_mut().for_each(|condition| {
                    if condition.number == reporting_target_condition_number {
                        condition.is_reporting_target = true;
                    }
                    condition.match_.iter_mut().for_each(|condition| {
                        if condition.number == reporting_target_condition_number {
                            condition.is_reporting_target = true;
                        }
                    });
                });
            });

        CompiledRuleset {
            rules,
            conditions_count,
        }
    }
}
