// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod context;

use crate::reporter::Reporter;
use crate::ruleset::{Rule, Ruleset};
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

fn apply_rule_to_path(context: &FileMatchContext, rule: &Rule, reporter: &mut dyn Reporter) {
    let text = fs::read_to_string(context.root.join(context.file)).unwrap();

    if let Some(antiregex) = &rule.antiregex {
        for line in text.lines() {
            if antiregex.is_match(line) {
                return;
            }
        }
    }

    if let Some(regex) = &rule.regex {
        for (nline, line) in text.lines().enumerate() {
            if regex.is_match(line) && !line.contains(IGNORE_MARKER) {
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

    if let Some(antiglobs) = &rule.antiglobs {
        for path in WalkDir::new(context.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
        {
            let path = path.strip_prefix(&context.root).unwrap();

            if antiglobs
                .iter()
                .any(|glob| glob.matches_path_with(path, match_options))
            {
                return;
            }
        }
    }

    if let Some(globs) = &rule.globs {
        for path in WalkDir::new(context.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
        {
            let path = path.strip_prefix(&context.root).unwrap();

            if globs
                .iter()
                .any(|glob| glob.matches_path_with(path, match_options))
            {
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
