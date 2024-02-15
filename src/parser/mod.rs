// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use self::yaml::ParsedConfig;
use crate::ruleset::{Glob, Regex, Rule, Ruleset};
use std::fs;
use std::path::{Path, PathBuf};

mod yaml;

pub struct Config {
    pub ruleset: Ruleset,
    pub roots: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            ruleset: Ruleset {
                rules: Default::default(),
            },
            roots: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Config {
        let mut config = Config::new();
        config.append_from_str(s);
        config
    }

    #[allow(dead_code)]
    pub fn from_file(path: &Path) -> Config {
        let mut config = Config::new();
        config.append_from_file(path);
        config
    }

    pub fn append_from_str_with_description(&mut self, s: &str, source_description: &str) {
        let parsed = ParsedConfig::from_str(s).unwrap();

        // XXX: switch to collect_into when that's stabilized
        if let Some(rules) = parsed.rules {
            self.ruleset.rules.extend(
                rules
                    .into_iter()
                    .enumerate()
                    .map(|(rule_num, parsed_rule)| Rule {
                        title: parsed_rule.title.unwrap_or_else(|| {
                            format!("rule #{} from {}", rule_num + 1, source_description)
                        }),
                        tags: parsed_rule.tags.into_iter().collect(),
                        globs: parsed_rule.files.map(|patterns| {
                            patterns
                                .iter()
                                .map(|pattern| Glob::new(&pattern).unwrap())
                                .collect()
                        }),
                        antiglobs: parsed_rule.nofiles.map(|patterns| {
                            patterns
                                .iter()
                                .map(|pattern| Glob::new(&pattern).unwrap())
                                .collect()
                        }),
                        regex: parsed_rule.pattern.map(|p| Regex::new(&p).unwrap()),
                        antiregex: parsed_rule.nomatch.map(|p| Regex::new(&p).unwrap()),
                    })
                    .collect::<Vec<_>>(),
            );
        }

        if let Some(roots) = parsed.roots {
            for root in roots {
                let mut paths: Vec<_> = glob::glob(&root)
                    .unwrap()
                    .map(|item| item.unwrap())
                    .collect();
                paths.sort();
                self.roots.append(&mut paths);
            }
        }
    }

    pub fn append_from_str(&mut self, s: &str) {
        self.append_from_str_with_description(s, "???")
    }

    pub fn append_from_file(&mut self, path: &Path) {
        let text = fs::read_to_string(path).unwrap();

        self.append_from_str_with_description(&text, &path.display().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let text = "
            rules:
            - title: 'test rule'
              files: '*.*'
              match: 'abc'
        ";

        let config = Config::from_str(text);

        assert_eq!(config.ruleset.rules.len(), 1);
    }
}
