// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod de;

use self::de::*;
use crate::config::Config;
use crate::ruleset::{Glob, Regex, Rule};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ParsedRule {
    title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_sequence")]
    tags: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string_sequence")]
    files: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_string_sequence")]
    nofiles: Option<Vec<String>>,
    #[serde(rename(serialize = "match", deserialize = "match"))]
    pattern: Option<String>,
    nomatch: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ParsedConfig {
    rules: Option<Vec<ParsedRule>>,
    roots: Option<Vec<String>>,
}

impl ParsedConfig {
    pub fn from_str(s: &str) -> Result<ParsedConfig, serde_yaml::Error> {
        serde_yaml::from_str(&s)
    }

    pub fn from_file(path: &Path) -> Result<ParsedConfig, serde_yaml::Error> {
        Self::from_str(&fs::read_to_string(path).unwrap())
    }

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

        let config = ParsedConfig::from_str(text).unwrap().into_config();

        assert_eq!(config.ruleset.rules.len(), 1);
    }
}
