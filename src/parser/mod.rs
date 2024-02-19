// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod pest;

use crate::config::Config;
use crate::ruleset::{Glob, Regex, Rule};

#[derive(PartialEq, Debug, Default)]
struct ParsedRule {
    title: Option<String>,
    tags: Option<Vec<String>>,
    files: Option<Vec<String>>,
    nofiles: Option<Vec<String>>,
    pattern: Option<String>,
    nomatch: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct ParsedConfig {
    rules: Option<Vec<ParsedRule>>,
    roots: Option<Vec<String>>,
}

impl ParsedConfig {
    pub fn append_into_config_with_description(
        self,
        config: &mut Config,
        source_description: &str,
    ) {
        // XXX: switch to collect_into when that's stabilized
        if let Some(rules) = self.rules {
            config.ruleset.rules.extend(
                rules
                    .into_iter()
                    .enumerate()
                    .map(|(rule_num, parsed_rule)| Rule {
                        title: parsed_rule.title.unwrap_or_else(|| {
                            format!("rule #{} from {}", rule_num + 1, source_description)
                        }),
                        tags: parsed_rule.tags.unwrap_or_default().into_iter().collect(),
                        globs: parsed_rule.files.map(|seq| {
                            seq.into_iter()
                                .map(|pattern| Glob::new(&pattern).unwrap())
                                .collect()
                        }),
                        antiglobs: parsed_rule.nofiles.map(|seq| {
                            seq.into_iter()
                                .map(|pattern| Glob::new(&pattern).unwrap())
                                .collect()
                        }),
                        regex: parsed_rule.pattern.map(|p| Regex::new(&p).unwrap()),
                        antiregex: parsed_rule.nomatch.map(|p| Regex::new(&p).unwrap()),
                    })
                    .collect::<Vec<_>>(),
            );
        }

        if let Some(roots) = self.roots {
            for root in roots {
                let mut paths: Vec<_> = glob::glob(&root)
                    .unwrap()
                    .map(|item| item.unwrap())
                    .collect();
                paths.sort();
                config.roots.append(&mut paths);
            }
        }
    }

    pub fn append_into_config(self, config: &mut Config) {
        self.append_into_config_with_description(config, "???")
    }

    #[allow(dead_code)]
    pub fn into_config_with_description(self, source_description: &str) -> Config {
        let mut config = Config::new();
        self.append_into_config_with_description(&mut config, source_description);
        config
    }

    #[allow(dead_code)]
    pub fn into_config(self) -> Config {
        let mut config = Config::new();
        self.append_into_config(&mut config);
        config
    }
}
