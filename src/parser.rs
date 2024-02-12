use crate::ruleset::{Glob, Regex, Rule, Ruleset};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// serde visitor for flexible string sequences
///
/// Visitor for deserialization of string sequences, represented either
/// as a single string with whitespace separators (`'foo bar baz'`), or
/// a sequence of strings (`['foo', 'bar', 'baz']`).
struct StringSequenceVisitor;

impl<'de> serde::de::Visitor<'de> for StringSequenceVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string with whitespace separated values or sequence of strings")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let res: Vec<_> = s.split_whitespace().map(|s| s.to_string()).collect();
        if res.is_empty() {
            Err(E::custom("empty string not allowed"))
        } else {
            Ok(res)
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut res = Vec::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(value) = seq.next_element::<String>()? {
            res.push(value);
        }

        if res.is_empty() {
            Err(A::Error::custom("empty sequence not allowed"))
        } else {
            Ok(res)
        }
    }
}

fn deserialize_optional_string_sequence<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserializer.deserialize_any(StringSequenceVisitor)?))
}

fn deserialize_string_sequence<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(deserializer.deserialize_any(StringSequenceVisitor)?)
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ParsedRule {
    pub title: String,
    #[serde(default, deserialize_with = "deserialize_string_sequence")]
    pub tags: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string_sequence")]
    pub files: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_string_sequence")]
    pub nofiles: Option<Vec<String>>,
    #[serde(rename(serialize = "match", deserialize = "match"))]
    pub pattern: Option<String>,
    pub nomatch: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ParsedConfig {
    pub rules: Option<Vec<ParsedRule>>,
    pub roots: Option<Vec<String>>,
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
