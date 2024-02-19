// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod context;

use crate::reporter::Reporter;
use crate::ruleset::{GlobCondition, RegexCondition, Rule, Ruleset};
use context::*;
use std::collections::HashSet;
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

fn check_globs_condition(
    condition: &GlobCondition,
    path: &Path,
    match_options: glob::MatchOptions,
) -> bool {
    condition
        .patterns
        .iter()
        .any(|glob| glob.matches_path_with(path, match_options))
        && !condition
            .excludes
            .iter()
            .any(|glob| glob.matches_path_with(path, match_options))
}

fn check_regexes_condition(condition: &RegexCondition, line: &str) -> bool {
    condition.patterns.iter().any(|regex| regex.is_match(&line))
        && !condition.excludes.iter().any(|regex| regex.is_match(&line))
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

fn apply_rule_to_root(context: &RootMatchContext, rule: &Rule, reporter: &mut dyn Reporter) {
    let mut match_options = glob::MatchOptions::new();
    match_options.require_literal_separator = true;

    if let Some(nofiles_cond) = &rule.nofiles {
        for path in WalkDir::new(context.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
        {
            let path = path.strip_prefix(&context.root).unwrap();

            if check_globs_condition(nofiles_cond, path, match_options) {
                return;
            }
        }
    }

    if let Some(files_cond) = &rule.files {
        for path in WalkDir::new(context.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
        {
            let path = path.strip_prefix(&context.root).unwrap();

            if check_globs_condition(files_cond, path, match_options) {
                apply_rule_to_path(&FileMatchContext::from_root(context, path), rule, reporter);
            }
        }
    } else {
        reporter.report(&context.to_location(), &rule.title);
    }
}

impl Applier<'_> {
    pub fn apply_to_root(&mut self, root: &Path) {
        let context = &RootMatchContext { root };

        for rule in &self.ruleset.rules {
            if !self.options.required_tags.is_empty()
                && self.options.required_tags.is_disjoint(&rule.tags)
                || !self.options.ignored_tags.is_disjoint(&rule.tags)
            {
                continue;
            }
            apply_rule_to_root(&context, &rule, self.reporter);
        }
    }
}
