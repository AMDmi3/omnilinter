// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod matching_caches;

use crate::r#match::{Match, MatchResult};
use crate::ruleset::compile::CompiledRuleset;
use crate::ruleset::{ConditionLogic, ContentCondition, GlobCondition, Rule};
use matching_caches::{GlobMatchingCache, RegexMatchingCache};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use walkdir::WalkDir;

fn apply_file_metadata_conditions(
    root: &Path,
    path: &Rc<PathBuf>,
    rules_with_conditions: &mut Vec<(&Rule, &GlobCondition)>,
) -> Result<(), std::io::Error> {
    let size = std::fs::metadata(root.join(path.as_path()))?.len();

    rules_with_conditions.retain(|(_, path_condition)| {
        for content_condition_node in &path_condition.content_conditions {
            if let ContentCondition::Size(size_condition) = &content_condition_node.condition
                && !size_condition.check(size)
            {
                return false;
            }
        }
        true
    });

    Ok(())
}

fn apply_file_line_conditions(
    num_lines: u64,
    rules_with_conditions: &mut Vec<(&Rule, &GlobCondition)>,
    rules_with_conditions_to_finalize: &mut HashMap<usize, (&Rule, &GlobCondition)>,
) {
    rules_with_conditions.retain(|(_, path_condition)| {
        for content_condition_node in &path_condition.content_conditions {
            if let ContentCondition::Lines(size_condition) = &content_condition_node.condition
                && !size_condition.check(num_lines)
            {
                rules_with_conditions_to_finalize.remove(&path_condition.number);
                return false;
            }
        }
        true
    });
}

fn apply_content_rules(
    ruleset: &CompiledRuleset,
    root: &Path,
    path: Rc<PathBuf>,
    mut rules_with_conditions: Vec<(&Rule, &GlobCondition)>,
    global_rule_statuses: &mut [RuleMatchStatus],
    global_condition_statuses: &mut [bool],
) -> Result<(), std::io::Error> {
    apply_file_metadata_conditions(root, &path, &mut rules_with_conditions)?;

    if rules_with_conditions.is_empty() {
        return Ok(());
    }

    let file = File::open(root.join(path.as_path()))?;
    let reader = BufReader::new(file);

    let mut local_condition_statuses: Vec<bool> = vec![false; global_condition_statuses.len()];
    let mut matched_lines: Vec<Vec<u64>> = vec![Default::default(); global_rule_statuses.len()];

    let mut rules_with_conditions_to_finalize: HashMap<usize, (&Rule, &GlobCondition)> =
        rules_with_conditions
            .iter()
            .map(|(rule, condition)| (condition.number, (*rule, *condition)))
            .collect();

    let mut line_number: u64 = 0;
    for line in reader.lines() {
        let line = line?;

        let mut matching_cache = RegexMatchingCache::new(&line, ruleset.regexes_count);

        rules_with_conditions.retain(|(rule, path_condition)| {
            let mut num_satisfied_content_conditions = 0;
            for content_condition_node in &path_condition.content_conditions {
                match &content_condition_node.condition {
                    ContentCondition::NoMatch(regex_condition) => {
                        if matching_cache.check_condition_match(regex_condition) {
                            rules_with_conditions_to_finalize.remove(&path_condition.number);
                            return false;
                        }
                    }
                    ContentCondition::Match(regex_condition) => {
                        let is_matched =
                            &mut local_condition_statuses[content_condition_node.number];
                        if content_condition_node.is_reporting_target {
                            if matching_cache.check_condition_match(regex_condition) {
                                *is_matched = true;
                                matched_lines[rule.number].push(line_number);
                            }
                        } else {
                            if !*is_matched {
                                *is_matched = matching_cache.check_condition_match(regex_condition);
                            }
                            if *is_matched {
                                num_satisfied_content_conditions += 1;
                            }
                        }
                    }
                    ContentCondition::Lines(size_condition) => {
                        if size_condition.check_for_this_and_above(line_number) {
                            num_satisfied_content_conditions += 1;
                        }
                    }
                    _ => {}
                }
            }

            // we don't need to do any more checks if all conditions are already satisfied
            num_satisfied_content_conditions != path_condition.linewise_content_conditions_count
        });

        // interrupt processing this file if all condition statuses are already known
        if rules_with_conditions.is_empty() {
            break;
        }

        line_number += 1;
    }

    apply_file_line_conditions(
        line_number,
        &mut rules_with_conditions,
        &mut rules_with_conditions_to_finalize,
    );

    rules_with_conditions_to_finalize
        .into_values()
        .for_each(|(rule, condition)| {
            if !condition.are_all_positive_conditions_satisfied(&local_condition_statuses) {
                return;
            }

            global_condition_statuses[condition.number] = true;

            if condition.has_reporting_target {
                global_rule_statuses[rule.number].matched_lines.extend(
                    matched_lines[rule.number]
                        .iter()
                        .map(|line_number| (path.clone(), *line_number)),
                );
            }

            if condition.is_reporting_target {
                global_rule_statuses[rule.number]
                    .matched_files
                    .push(path.clone());
            }
        });

    Ok(())
}

#[derive(Default)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
struct RuleMatchStatus<'a> {
    pub content_checks: Vec<(&'a GlobCondition, Rc<PathBuf>)>,
    pub matched_files: Vec<Rc<PathBuf>>,
    pub matched_lines: Vec<(Rc<PathBuf>, u64)>,
}

pub fn apply_ruleset<'a>(ruleset: &'a CompiledRuleset, root: &'a Path) -> MatchResult<'a> {
    let mut result: MatchResult = Default::default();

    let mut rule_statuses: Vec<RuleMatchStatus> =
        ruleset.rules.iter().map(|_| Default::default()).collect();

    let mut files_condition_statuses: Vec<bool> = vec![false; ruleset.conditions_count];

    let mut rules: Vec<_> = ruleset.rules.iter().collect();

    rules.retain(|rule| {
        // NOTE: possible checks to tied to root's file hierarchy (such
        // as running a process on a whole root) may be implemented here
        if rule.path_conditions.is_empty() {
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
            let mut matching_cache =
                GlobMatchingCache::new(&path, match_options, ruleset.globs_count);

            rules.retain(|rule| {
                for path_condition in &rule.path_conditions {
                    match path_condition.logic {
                        ConditionLogic::Negative => {
                            if matching_cache.check_condition_match(path_condition) {
                                return false;
                            }
                        }
                        ConditionLogic::Positive => {
                            let is_matched = &mut files_condition_statuses[path_condition.number];
                            if path_condition.is_reporting_target
                                || !path_condition.content_conditions.is_empty()
                            {
                                if matching_cache.check_condition_match(path_condition) {
                                    *is_matched = true;
                                    let rule_status = &mut rule_statuses[rule.number];
                                    if !path_condition.content_conditions.is_empty() {
                                        rule_status
                                            .content_checks
                                            .push((path_condition, path.clone()));
                                    } else if path_condition.is_reporting_target {
                                        rule_status.matched_files.push(path.clone());
                                    }
                                }
                            } else if !*is_matched {
                                *is_matched = matching_cache.check_condition_match(path_condition);
                            }
                        }
                    }
                }

                true
            });
        });

    let mut content_rules_by_path: HashMap<Rc<PathBuf>, Vec<(&Rule, &GlobCondition)>> =
        HashMap::new();

    rules.retain(|rule| {
        if !rule.are_all_positive_conditions_satisfied(&files_condition_statuses) {
            return false;
        }

        let rule_status = &mut rule_statuses[rule.number];

        for (condition, path) in rule_status.content_checks.iter() {
            files_condition_statuses[condition.number] = false;
            content_rules_by_path
                .entry(path.clone())
                .or_default()
                .push((rule, condition));
        }
        true
    });

    for (path, rules_with_conditions) in content_rules_by_path.into_iter() {
        if let Err(err) = apply_content_rules(
            ruleset,
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
        if !rule.are_all_positive_conditions_satisfied(&files_condition_statuses) {
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
