// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::Config;
use crate::ruleset::Rule as RulesetRule;
use crate::ruleset::{Glob, GlobCondition, Regex, RegexCondition};
use pest::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(pest_derive::Parser)]
#[grammar = "parser/omnilinter.pest"]
pub struct ConfigParser;

fn parse_tags(pair: pest::iterators::Pair<Rule>) -> HashSet<String> {
    pair.into_inner()
        .map(|tag| tag.as_str().to_string())
        .collect()
}

fn parse_globs_condition(pair: pest::iterators::Pair<Rule>) -> GlobCondition {
    let mut cond: GlobCondition = Default::default();
    for item in pair.into_inner() {
        let item = item.as_str();
        if let Some(item) = item.strip_prefix('!') {
            cond.excludes.push(Glob::new(item).unwrap());
        } else {
            cond.patterns.push(Glob::new(item).unwrap());
        }
    }
    cond
}

fn parse_regex_str(s: &str) -> Regex {
    let quote_char = s.chars().next().unwrap();

    let mut output: String = String::with_capacity(s.len() - 2);
    let mut escaped = false;
    for c in s[1..s.len() - 1].chars() {
        if escaped {
            debug_assert!(c == '\\' || c == quote_char); //enforced by the parser
            output.push(c);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else {
            output.push(c);
        }
    }

    Regex::new(&output).unwrap()
}

fn parse_regexes_condition(pair: pest::iterators::Pair<Rule>) -> RegexCondition {
    let mut cond: RegexCondition = Default::default();
    for item in pair.into_inner() {
        let item = item.as_str();
        if let Some(item) = item.strip_prefix('!') {
            cond.excludes.push(parse_regex_str(&item[1..]));
        } else {
            cond.patterns.push(parse_regex_str(item));
        }
    }
    cond
}

fn parse_rule(
    pair: pest::iterators::Pair<Rule>,
    rule_number: usize,
    source_desc: &str,
) -> RulesetRule {
    let mut rule: RulesetRule = Default::default();

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::rule_title => {
                let line_number = item.as_span().start_pos().line_col().0;
                let title = item.into_inner().next().unwrap().as_str();
                if title.is_empty() {
                    rule.title = format!(
                        "rule from {}:{} (#{})",
                        source_desc, line_number, rule_number
                    );
                } else {
                    rule.title = title.to_string();
                }
            }
            Rule::rule_directive_tags => {
                if !rule.tags.is_empty() {
                    panic!("tags specified multiple times");
                }
                rule.tags = parse_tags(item.into_inner().next().unwrap())
            }
            Rule::rule_directive_files => {
                if rule.files.is_some() {
                    panic!("files condition specified multiple times");
                }
                rule.files = Some(parse_globs_condition(item.into_inner().next().unwrap()));
            }
            Rule::rule_directive_nofiles => {
                if rule.nofiles.is_some() {
                    panic!("nofiles condition specified multiple times");
                }
                rule.nofiles = Some(parse_globs_condition(item.into_inner().next().unwrap()));
            }
            Rule::rule_directive_match => {
                if rule.match_.is_some() {
                    panic!("match condition specified multiple times");
                }
                rule.match_ = Some(parse_regexes_condition(item.into_inner().next().unwrap()));
            }
            Rule::rule_directive_nomatch => {
                if rule.nomatch.is_some() {
                    panic!("nomatch condition specified multiple times");
                }
                rule.nomatch = Some(parse_regexes_condition(item.into_inner().next().unwrap()));
            }
            _ => unreachable!("unexpected parser rule type in parse_rule {:#?}", item),
        }
    }

    rule
}

impl Config {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Result<Config, ()> {
        Self::from_str_with_desc(s, "???")
    }

    pub fn from_str_with_desc(s: &str, source_desc: &str) -> Result<Config, ()> {
        let mut config: Config = Default::default();

        let file = ConfigParser::parse(Rule::file, s)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let mut rule_number: usize = 0;
        for item in file.into_inner() {
            match item.as_rule() {
                Rule::config_directive_root => {
                    let root_pattern = item.into_inner().next().unwrap().as_str();
                    let mut root_paths: Vec<_> = glob::glob(root_pattern)
                        .unwrap()
                        .map(|item| item.unwrap())
                        .collect();
                    root_paths.sort();
                    config.roots.append(&mut root_paths);
                }
                Rule::rule => {
                    rule_number += 1;
                    config
                        .ruleset
                        .rules
                        .push(parse_rule(item, rule_number, source_desc));
                }
                Rule::EOI => (),
                _ => unreachable!("unexpected parser rule type in from_str {:#?}", item),
            }
        }

        Ok(config)
    }

    pub fn from_file(path: &Path) -> Result<Config, ()> {
        Self::from_str_with_desc(
            &fs::read_to_string(path).unwrap(),
            &path.display().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let text = "
            [test rule]
            files *.*
            match /abc/
        ";

        let config = Config::from_str(text).unwrap();

        assert_eq!(config.ruleset.rules.len(), 1);
    }
}
