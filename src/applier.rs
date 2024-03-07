// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod glob_cache;

use crate::r#match::{Match, MatchResult};
use crate::ruleset::{CompiledRuleset, GlobCondition, RegexCondition, Rule};
use glob_cache::GlobMatchingCache;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use walkdir::WalkDir;

const IGNORE_MARKER: &str = "omnilinter: ignore";

fn check_regexes_condition(condition: &RegexCondition, line: &str) -> bool {
    condition.patterns.iter().any(|regex| regex.is_match(line))
        && !condition.excludes.iter().any(|regex| regex.is_match(line))
}

fn apply_content_rules(
    root: &Path,
    path: Rc<PathBuf>,
    mut rules_with_conditions: Vec<(&Rule, &GlobCondition)>,
    global_rule_statuses: &mut Vec<RuleMatchStatus>,
    global_condition_statuses: &mut Vec<bool>,
) -> Result<(), std::io::Error> {
    let file = File::open(root.join(path.as_path()))?;
    let reader = BufReader::new(file);

    let mut content_condition_statuses: Vec<bool> = vec![false; global_condition_statuses.len()];

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        rules_with_conditions.retain(|(rule, condition)| {
            if condition.nomatch.iter().any(|condition| {
                check_regexes_condition(condition, &line) && !line.contains(IGNORE_MARKER)
            }) {
                return false;
            }

            condition.match_.iter().for_each(|condition| {
                let is_matched = &mut content_condition_statuses[condition.number];
                if condition.is_reporting_target {
                    if check_regexes_condition(condition, &line) && !line.contains(IGNORE_MARKER) {
                        *is_matched = true;
                        global_rule_statuses[rule.number]
                            .matched_lines
                            .push((path.clone(), line_number));
                    }
                } else if !*is_matched {
                    *is_matched =
                        check_regexes_condition(condition, &line) && !line.contains(IGNORE_MARKER);
                }
            });
            true
        });
    }

    rules_with_conditions.iter().for_each(|(rule, condition)| {
        if !condition
            .match_
            .iter()
            .all(|condition| content_condition_statuses[condition.number])
        {
            return;
        }

        global_condition_statuses[condition.number] = true;

        if condition.is_reporting_target {
            global_rule_statuses[rule.number]
                .matched_files
                .push(path.clone());
        }
    });

    Ok(())
}

#[derive(Default, Debug)]
struct RuleMatchStatus<'a> {
    pub files_conditions_passed: Vec<bool>,

    pub content_checks: Vec<(&'a GlobCondition, Rc<PathBuf>)>,

    pub matched_files: Vec<Rc<PathBuf>>,
    pub matched_lines: Vec<(Rc<PathBuf>, usize)>,
}

impl<'a> RuleMatchStatus<'a> {
    pub fn new(rule: &'a Rule) -> RuleMatchStatus<'a> {
        RuleMatchStatus {
            files_conditions_passed: vec![false; rule.files.len()],
            content_checks: vec![],
            matched_files: vec![],
            matched_lines: vec![],
        }
    }
}

pub fn apply_ruleset<'a>(ruleset: &'a CompiledRuleset, root: &'a Path) -> MatchResult<'a> {
    let mut result: MatchResult = Default::default();

    let mut rule_statuses: Vec<RuleMatchStatus> =
        ruleset.rules.iter().map(RuleMatchStatus::new).collect();

    let mut files_condition_statuses: Vec<bool> = vec![false; ruleset.conditions_count];

    let mut rules: Vec<_> = ruleset.rules.iter().collect();

    rules.retain(|rule| {
        // NOTE: possible checks to tied to root's file hierarchy (such
        // as running a process on a whole root) may be implemented here
        if rule.files.is_empty() && rule.nofiles.is_empty() {
            // rules without any glob matchers always match on the root level
            result.matches.push(Match::for_root(rule, root));
            return false;
        }
        true
    });

    let mut match_options = glob::MatchOptions::new();
    match_options.require_literal_separator = true;

    WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| Rc::new(e.into_path().strip_prefix(root).unwrap().to_path_buf()))
        .for_each(|path| {
            let mut matching_cache = GlobMatchingCache::new(&path, match_options);

            rules.retain(|rule| {
                if rule
                    .nofiles
                    .iter()
                    .any(|condition| matching_cache.check_condition_match(condition))
                {
                    return false;
                }

                let rule_status = &mut rule_statuses[rule.number];

                rule.files
                    .iter()
                    .zip(rule_status.files_conditions_passed.iter_mut())
                    .for_each(|(condition, is_matched)| {
                        if condition.is_reporting_target
                            || !condition.match_.is_empty()
                            || !condition.nomatch.is_empty()
                        {
                            if matching_cache.check_condition_match(condition) {
                                *is_matched = true;
                                if !condition.match_.is_empty() || !condition.nomatch.is_empty() {
                                    rule_status.content_checks.push((condition, path.clone()));
                                } else if condition.is_reporting_target {
                                    rule_status.matched_files.push(path.clone());
                                }
                            }
                        } else if !*is_matched {
                            *is_matched = matching_cache.check_condition_match(condition);
                        }
                    });

                true
            });
        });

    let mut content_rules_by_path: HashMap<Rc<PathBuf>, Vec<(&Rule, &GlobCondition)>> =
        HashMap::new();

    rules.retain(|rule| {
        let rule_status = &mut rule_statuses[rule.number];

        if !rule_status
            .files_conditions_passed
            .iter()
            .all(|passed| *passed)
        {
            return false;
        }

        for (condition, path) in rule_status.content_checks.iter() {
            content_rules_by_path
                .entry(path.clone())
                .or_default()
                .push((rule, condition));
        }
        true
    });

    for (path, rules_with_conditions) in content_rules_by_path.into_iter() {
        if let Err(err) = apply_content_rules(
            root,
            path.clone(),
            rules_with_conditions,
            &mut rule_statuses,
            &mut files_condition_statuses,
        ) {
            eprintln!("failed to process {}: {}", path.display(), err);
        }
    }

    rules.iter().for_each(|rule| {
        if !rule.files.iter().all(|condition| {
            condition.match_.is_empty() && condition.nomatch.is_empty()
                || files_condition_statuses[condition.number]
        }) {
            return;
        }

        if rule.is_reporting_target {
            result.matches.push(Match::for_root(rule, root));
        }

        let rule_status = &rule_statuses[rule.number];

        for path in &rule_status.matched_files {
            result
                .matches
                .push(Match::for_file(rule, root, path.clone()))
        }
        for (path, line_number) in &rule_status.matched_lines {
            result
                .matches
                .push(Match::for_line(rule, root, path.clone(), *line_number))
        }
    });

    result
}
