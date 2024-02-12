use crate::location::*;
use crate::reporter::Reporter;
use crate::ruleset::{Rule, Ruleset};
use glob;
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

fn apply_rule_to_path(loc: &FileMatchLocation, rule: &Rule, reporter: &mut dyn Reporter) {
    let text = fs::read_to_string(loc.root.join(loc.file)).unwrap();

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
                reporter.report(
                    &MatchLocation::Line(LineMatchLocation::from_file(loc, nline + 1)),
                    &rule.title,
                );
            }
        }
    } else {
        reporter.report(&MatchLocation::File(*loc), &rule.title);
    }
}

fn apply_rule_to_root(loc: &RootMatchLocation, rule: &Rule, reporter: &mut dyn Reporter) {
    let mut match_options = glob::MatchOptions::new();
    match_options.require_literal_separator = true;

    if let Some(antiglobs) = &rule.antiglobs {
        for path in WalkDir::new(loc.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
        {
            let path = path.strip_prefix(&loc.root).unwrap();

            if antiglobs
                .iter()
                .any(|glob| glob.matches_path_with(path, match_options))
            {
                return;
            }
        }
    }

    if let Some(globs) = &rule.globs {
        for path in WalkDir::new(loc.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
        {
            let path = path.strip_prefix(&loc.root).unwrap();

            if globs
                .iter()
                .any(|glob| glob.matches_path_with(path, match_options))
            {
                apply_rule_to_path(&FileMatchLocation::from_root(loc, path), rule, reporter);
            }
        }
    } else {
        reporter.report(&MatchLocation::Root(*loc), &rule.title);
    }
}

impl Applier<'_> {
    pub fn apply_to_root(&mut self, root: &Path) {
        let loc = &RootMatchLocation { root };

        for rule in &self.ruleset.rules {
            if !self.options.required_tags.is_empty()
                && self.options.required_tags.is_disjoint(&rule.tags)
                || !self.options.ignored_tags.is_disjoint(&rule.tags)
            {
                continue;
            }
            apply_rule_to_root(&loc, &rule, self.reporter);
        }
    }
}
