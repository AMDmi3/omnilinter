// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::parser::{ParsedConfig, ParsedRule};
use pest::Parser;
use std::fs;
use std::path::Path;

#[derive(pest_derive::Parser)]
#[grammar = "parser/omnilinter.pest"]
pub struct ConfigParserPest;

impl ParsedConfig {
    fn parse_tags(pair: pest::iterators::Pair<Rule>) -> Vec<String> {
        pair.into_inner()
            .map(|tag| tag.as_str().to_string())
            .collect()
    }

    fn parse_files(pair: pest::iterators::Pair<Rule>) -> Vec<String> {
        pair.into_inner()
            .map(|glob| glob.as_str().to_string())
            .collect()
    }

    fn parse_match(pair: pest::iterators::Pair<Rule>) -> String {
        let regexp_expr = pair.as_str();
        let regexp_quote_char = regexp_expr.chars().nth(0).unwrap();

        let mut regexp: String = String::with_capacity(regexp_expr.len() - 2);

        let mut escaped = false;
        for c in regexp_expr[1..regexp_expr.len() - 1].chars() {
            if escaped {
                if c != '\\' && c != regexp_quote_char {
                    regexp.push('\\');
                }
                regexp.push(c);
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                regexp.push(c);
            }
        }

        regexp
    }

    fn parse_rule(pair: pest::iterators::Pair<Rule>) -> ParsedRule {
        let mut rule: ParsedRule = Default::default();
        for item in pair.into_inner() {
            match item.as_rule() {
                Rule::rule_title => {
                    rule.title = Some(item.into_inner().next().unwrap().as_str().to_string());
                },
                Rule::rule_directive_tags => {
                    rule.tags = Some(
                        Self::parse_tags(item.into_inner().next().unwrap())
                    );
                },
                Rule::rule_directive_files => {
                    rule.files = Some(
                        Self::parse_files(item.into_inner().next().unwrap())
                    );
                },
                Rule::rule_directive_nofiles => {
                    rule.nofiles = Some(
                        Self::parse_files(item.into_inner().next().unwrap())
                    );
                },
                Rule::rule_directive_match => {
                    rule.pattern = Some(
                        Self::parse_match(item.into_inner().next().unwrap())
                    );
                },
                Rule::rule_directive_nomatch => {
                    rule.nomatch = Some(
                        Self::parse_match(item.into_inner().next().unwrap())
                    );
                },
                _ => unreachable!("unexpected parser rule type in parse_rule {:#?}, partially constructed rule {:#?}", item, rule),
            }
        }

        rule
    }

    pub fn from_str(s: &str) -> Result<ParsedConfig, ()> {
        let mut config = ParsedConfig {
            rules: Default::default(),
            roots: Default::default(),
        };

        let file = ConfigParserPest::parse(Rule::file, &s)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        for item in file.into_inner() {
            match item.as_rule() {
                Rule::config_directive_root => {
                    config
                        .roots
                        .get_or_insert_default()
                        .push(item.into_inner().next().unwrap().as_str().to_string());
                }
                Rule::rule => {
                    config
                        .rules
                        .get_or_insert_default()
                        .push(Self::parse_rule(item));
                }
                Rule::EOI => (),
                _ => unreachable!("unexpected parser rule type in from_str {:#?}", item),
            }
        }

        Ok(config)
    }

    pub fn from_file(path: &Path) -> Result<ParsedConfig, ()> {
        Self::from_str(&fs::read_to_string(path).unwrap())
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

        let config = ParsedConfig::from_str(text).unwrap().into_config();

        assert_eq!(config.ruleset.rules.len(), 1);
    }
}
