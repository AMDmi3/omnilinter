use crate::ruleset::{Glob, Regex, Rule, Ruleset};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ParsedRule {
    pub title: String,
    pub files: String,
    #[serde(rename(serialize = "match", deserialize = "match"))]
    pub pattern: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ParsedConfig {
    pub rules: Vec<ParsedRule>,
}

impl ParsedConfig {
    fn from_str(s: &str) -> Result<ParsedConfig, serde_yaml::Error> {
        serde_yaml::from_str(&s)
    }
}

pub fn parse_config_from_str(s: &str) -> Ruleset {
    let parsed = ParsedConfig::from_str(s).unwrap();

    Ruleset {
        rules: parsed
            .rules
            .into_iter()
            .map(|parsed_rule| Rule {
                title: parsed_rule.title,
                glob: Glob::new(&parsed_rule.files).unwrap(),
                regex: parsed_rule.pattern.map(|p| Regex::new(&p).unwrap()),
            })
            .collect(),
    }
}

pub fn parse_config_from_file(path: &Path) -> Ruleset {
    let text = fs::read_to_string(path).unwrap();

    parse_config_from_str(&text)
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

        let parsed_yaml = ParsedConfig::from_str(text).unwrap();

        assert_eq!(
            parsed_yaml,
            ParsedConfig {
                rules: vec![ParsedRule {
                    title: String::from("test rule"),
                    files: String::from("*.*"),
                    pattern: String::from("abc"),
                }]
            }
        );

        let parsed_config = parse_config_from_str(text);

        assert_eq!(parsed_config.rules.len(), 1);
    }
}
