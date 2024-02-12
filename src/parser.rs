use crate::ruleset::{Glob, Regex, Rule, Ruleset};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ParsedRule {
    pub title: String,
    pub files: Option<String>,
    pub nofiles: Option<String>,
    #[serde(rename(serialize = "match", deserialize = "match"))]
    pub pattern: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ParsedConfig {
    pub rules: Option<Vec<ParsedRule>>,
    pub roots: Option<Vec<PathBuf>>,
}

impl ParsedConfig {
    fn from_str(s: &str) -> Result<ParsedConfig, serde_yaml::Error> {
        serde_yaml::from_str(&s)
    }
}

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

    pub fn append_from_str(&mut self, s: &str) {
        let parsed = ParsedConfig::from_str(s).unwrap();

        // XXX: switch to collect_into when that's stabilized
        if let Some(rules) = parsed.rules {
            self.ruleset.rules.extend(
                rules
                    .into_iter()
                    .map(|parsed_rule| Rule {
                        title: parsed_rule.title,
                        globs: parsed_rule.files.map(|g| vec![Glob::new(&g).unwrap()]),
                        antiglobs: parsed_rule.nofiles.map(|g| vec![Glob::new(&g).unwrap()]),
                        regex: parsed_rule.pattern.map(|p| Regex::new(&p).unwrap()),
                    })
                    .collect::<Vec<_>>(),
            );
        }

        if let Some(roots) = parsed.roots {
            self.roots.extend(roots);
        }
    }

    pub fn append_from_file(&mut self, path: &Path) {
        let text = fs::read_to_string(path).unwrap();

        self.append_from_str(&text)
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
