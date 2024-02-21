// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod context;

use crate::reporter::Reporter;
use crate::ruleset::{Glob, GlobCondition, RegexCondition, Rule, Ruleset};
use context::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const IGNORE_MARKER: &str = "omnilinter: ignore";

pub struct ApplierOptions {
    pub required_tags: HashSet<String>,
    pub ignored_tags: HashSet<String>,
}

pub struct Applier<'a> {
    ruleset: &'a Ruleset,
    reporter: &'a mut dyn Reporter,
    options: ApplierOptions,
}

impl Applier<'_> {
    pub fn new<'a>(
        ruleset: &'a Ruleset,
        reporter: &'a mut dyn Reporter,
        options: ApplierOptions,
    ) -> Applier<'a> {
        Applier {
            ruleset,
            reporter,
            options,
        }
    }
}

fn check_regexes_condition(condition: &RegexCondition, line: &str) -> bool {
    condition.patterns.iter().any(|regex| regex.is_match(line))
        && !condition.excludes.iter().any(|regex| regex.is_match(line))
}

fn apply_rule_to_path(context: &FileMatchContext, rule: &Rule, reporter: &mut dyn Reporter) {
    let text = fs::read_to_string(context.root.join(context.file)).unwrap();

    if let Some(nomatch_cond) = &rule.nomatch {
        for line in text.lines() {
            if check_regexes_condition(nomatch_cond, line) {
                return;
            }
        }
    }

    if let Some(match_cond) = &rule.match_ {
        for (nline, line) in text.lines().enumerate() {
            if check_regexes_condition(match_cond, line) && !line.contains(IGNORE_MARKER) {
                reporter.report(&context.to_location_with_line(nline), &rule.title);
            }
        }
    } else {
        reporter.report(&context.to_location(), &rule.title);
    }
}

fn is_tags_allowed(
    rule_tags: &HashSet<String>,
    required_tags: &HashSet<String>,
    ignored_tags: &HashSet<String>,
) -> bool {
    rule_tags.is_disjoint(ignored_tags)
        && (required_tags.is_empty() || !rule_tags.is_disjoint(required_tags))
}

struct GlobMatchingCache<'a> {
    path: &'a Path,
    match_options: glob::MatchOptions,
    glob_matches: HashMap<&'a Glob, bool>,
}

impl<'a> GlobMatchingCache<'a> {
    pub fn new(path: &'a Path, match_options: glob::MatchOptions) -> Self {
        GlobMatchingCache {
            path,
            match_options,
            glob_matches: Default::default(),
        }
    }

    pub fn check_glob_match(&mut self, glob: &'a Glob) -> bool {
        // XXX: benchmarks shows that the cache yields 2x performance
        // regression compared to straightforward glob matching in all
        // cases except "multiple rules with same pattern". This is
        // quite expected as we still have to match each pattern from
        // scratch plus we have cache overhead. In practice, howerver
        // rules are expected to have same patterns (such as a lot of
        // rules for *.py), so the cache still makes sence. Also, the
        // regression can be fixed by caching by pre-grouping globs
        // (during some kind of ruleset compilation phase) and assiging
        // unique incrementing ids to them, then indexing this cache
        // by these ids instead of computing hashes.
        *self
            .glob_matches
            .entry(glob)
            .or_insert_with(|| glob.matches_path_with(self.path, self.match_options))
    }

    pub fn check_condition_match(&mut self, condition: &'a GlobCondition) -> bool {
        condition
            .patterns
            .iter()
            .any(|glob| self.check_glob_match(glob))
            && !condition
                .excludes
                .iter()
                .any(|glob| self.check_glob_match(glob))
    }
}

impl Applier<'_> {
    pub fn apply_to_root(&mut self, root: &Path) {
        let root_context = &RootMatchContext { root };

        let mut active_rules: Vec<_> = self
            .ruleset
            .rules
            .iter()
            .filter(|rule| {
                is_tags_allowed(
                    &rule.tags,
                    &self.options.required_tags,
                    &self.options.ignored_tags,
                )
            })
            .collect();

        active_rules.retain(|rule| {
            // NOTE: possible checks to tied to root's file hierarchy (such
            // as running a process on a whole root) may be implemented here
            if rule.files.is_none() && rule.nofiles.is_none() {
                // rules without any glob matchers always match on the root level
                self.reporter
                    .report(&root_context.to_location(), &rule.title);
                return false;
            }
            true
        });

        let mut match_options = glob::MatchOptions::new();
        match_options.require_literal_separator = true;

        for path in WalkDir::new(root_context.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
        {
            let path = path.strip_prefix(root_context.root).unwrap();

            let mut matching_cache = GlobMatchingCache::new(&path, match_options);

            active_rules.retain(|rule| {
                if let Some(condition) = &rule.nofiles {
                    if matching_cache.check_condition_match(condition) {
                        // when nofiles matches processing for this rule stops immediately
                        return false;
                    }
                }

                if let Some(condition) = &rule.files {
                    if matching_cache.check_condition_match(condition) {
                        apply_rule_to_path(
                            &FileMatchContext::from_root(root_context, path),
                            rule,
                            self.reporter,
                        );
                    }
                }
                true
            });
        }

        active_rules.iter().for_each(|rule| {
            if rule.files.is_none() {
                // Rules which end up here are rules with `nofiles` condition
                // which hasn't matched and no `files` conditions. So, these
                // match on the root level
                self.reporter
                    .report(&root_context.to_location(), &rule.title);
            }
        });
    }
}
