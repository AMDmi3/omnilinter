use crate::location::*;
use crate::reporter::Reporter;
use crate::ruleset::{Rule, Ruleset};
use glob;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn apply_rule_to_path(loc: &FileMatchLocation, rule: &Rule, reporter: &mut Reporter) {
    if let Some(regex) = &rule.regex {
        let text = fs::read_to_string(loc.root.join(loc.file)).unwrap();

        for (nline, line) in text.lines().enumerate() {
            if regex.is_match(line) {
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

fn apply_rule_to_root(loc: &RootMatchLocation, rule: &Rule, reporter: &mut Reporter) {
    let mut match_options = glob::MatchOptions::new();
    match_options.require_literal_separator = true;

    for path in WalkDir::new(loc.root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
    {
        let path = path.strip_prefix(&loc.root).unwrap();
        if rule.glob.matches_path_with(path, match_options) {
            apply_rule_to_path(&FileMatchLocation::from_root(loc, path), rule, reporter);
        }
    }
}

pub fn apply_ruleset_to_root(ruleset: &Ruleset, root: &Path, reporter: &mut Reporter) {
    let loc = &RootMatchLocation { root };
    for rule in &ruleset.rules {
        apply_rule_to_root(&loc, &rule, reporter);
    }
}
