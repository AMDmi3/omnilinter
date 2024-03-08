// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(test)]
mod tests;

use crate::config::Config;
use crate::ruleset::Rule as RulesetRule;
use crate::ruleset::{ConditionLogic, Glob, GlobCondition, Regex, RegexCondition};
use pest::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(pest_derive::Parser)]
#[grammar = "config/parser/omnilinter.pest"]
pub struct ConfigParser;

fn parse_tags(pair: pest::iterators::Pair<Rule>) -> HashSet<String> {
    pair.into_inner()
        .map(|tag| tag.as_str().to_lowercase())
        .collect()
}

fn parse_globs_condition(
    pair: pest::iterators::Pair<Rule>,
    logic: ConditionLogic,
) -> GlobCondition {
    let mut cond: GlobCondition = Default::default();
    cond.logic = logic;
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

fn parse_title(s: &str) -> String {
    s[1..s.len() - 1].replace("]]", "]")
}

fn parse_regex_str(s: &str) -> Regex {
    let framing_char_length = s
        .chars()
        .next()
        .expect("framing characters presence is enforced by grammar")
        .len_utf8();
    Regex::new(&s[framing_char_length..s.len() - framing_char_length]).unwrap()
}

fn parse_regexes_condition(
    pair: pest::iterators::Pair<Rule>,
    logic: ConditionLogic,
) -> RegexCondition {
    let mut cond: RegexCondition = Default::default();
    cond.logic = logic;
    for item in pair.into_inner() {
        let item = item.as_str();
        if let Some(item) = item.strip_prefix('!') {
            cond.excludes.push(parse_regex_str(item));
        } else {
            cond.patterns.push(parse_regex_str(item));
        }
    }
    cond
}

fn parse_files_condition(pair: pest::iterators::Pair<Rule>) -> GlobCondition {
    let mut condition: GlobCondition = Default::default();

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::rule_directive_files_inner => {
                condition = parse_globs_condition(
                    item.into_inner().next().unwrap(),
                    ConditionLogic::Positive,
                );
            }
            Rule::rule_directive_match => {
                condition.content_conditions.push(parse_regexes_condition(
                    item.into_inner().next().unwrap(),
                    ConditionLogic::Positive,
                ));
            }
            Rule::rule_directive_nomatch => {
                condition.content_conditions.push(parse_regexes_condition(
                    item.into_inner().next().unwrap(),
                    ConditionLogic::Negative,
                ));
            }
            _ => unreachable!(
                "unexpected parser rule type in parse_files_condition {:#?}",
                item
            ),
        }
    }

    condition
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
                        source_desc,
                        line_number,
                        rule_number + 1
                    );
                } else {
                    rule.title = parse_title(title);
                }
            }
            Rule::rule_directive_tags => rule.tags = parse_tags(item.into_inner().next().unwrap()),
            Rule::rule_directive_files => {
                rule.path_conditions.push(parse_files_condition(item));
            }
            Rule::rule_directive_nofiles => {
                rule.path_conditions.push(parse_globs_condition(
                    item.into_inner().next().unwrap(),
                    ConditionLogic::Negative,
                ));
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
                    config.ruleset.rules.push(parse_rule(
                        item,
                        config.ruleset.rules.len(),
                        source_desc,
                    ));
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
