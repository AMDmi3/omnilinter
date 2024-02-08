use crate::ruleset::{Rule, Ruleset};
use glob;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn apply_rule_to_path(rule: &Rule, path: &Path) {
    let text = fs::read_to_string(path).unwrap();

    for (nline, line) in text.lines().enumerate() {
        if rule.regex.is_match(line) {
            println!("{}:{}: {}", path.display(), nline + 1, rule.title);
        }
    }
}

fn apply_rule_to_target(rule: &Rule, target: &Path) {
    let mut match_options = glob::MatchOptions::new();
    match_options.require_literal_separator = true;

    for path in WalkDir::new(target)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
    {
        let path = path.strip_prefix(&target).unwrap();
        if rule.glob.matches_path_with(path, match_options) {
            apply_rule_to_path(rule, path);
        }
    }
}

pub fn apply_ruleset_to_target(ruleset: &Ruleset, target: &Path) {
    for rule in &ruleset.rules {
        apply_rule_to_target(&rule, target);
    }
}
