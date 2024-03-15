// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(clippy::result_large_err)] // config parser is not performance critical
#![allow(clippy::empty_docs)] // fires on ConfigParser, probably comes from pest

#[cfg(test)]
mod tests;

use crate::config::Config;
use crate::ruleset::Rule as RulesetRule;
use crate::ruleset::{
    ConditionLogic, ContentCondition, ContentConditionNode, Glob, GlobCondition, Regex,
    RegexCondition,
};
use anyhow::{Context, Error};
use pest::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(pest_derive::Parser)]
#[grammar = "config/parser/omnilinter.pest"]
pub struct ConfigParser;

type PestError = pest::error::Error<Rule>;

fn glob_pattern_error_into_pest_error(
    e: glob::PatternError,
    item: &pest::iterators::Pair<Rule>,
) -> PestError {
    let span = item.as_span();
    let pos = pest::Position::new(span.get_input(), span.start() + e.pos).unwrap();
    PestError::new_from_pos(
        pest::error::ErrorVariant::<Rule>::CustomError {
            message: e.msg.to_owned(),
        },
        pos,
    )
}

fn regex_error_into_pest_error(e: regex::Error, item: &pest::iterators::Pair<Rule>) -> PestError {
    PestError::new_from_span(
        pest::error::ErrorVariant::<Rule>::CustomError {
            message: e.to_string(),
        },
        item.as_span(),
    )
}

fn glob_error_into_pest_error(e: glob::GlobError, item: &pest::iterators::Pair<Rule>) -> PestError {
    PestError::new_from_span(
        pest::error::ErrorVariant::<Rule>::CustomError {
            message: e.to_string(),
        },
        item.as_span(),
    )
}

fn parse_tags(pair: pest::iterators::Pair<Rule>) -> HashSet<String> {
    pair.into_inner()
        .map(|tag| tag.as_str().to_lowercase())
        .collect()
}

fn parse_globs_condition(
    pair: pest::iterators::Pair<Rule>,
    logic: ConditionLogic,
) -> Result<GlobCondition, PestError> {
    let mut cond = GlobCondition {
        logic,
        ..Default::default()
    };
    for item in pair.into_inner() {
        let item_text = item.as_str();
        if let Some(item_text) = item_text.strip_prefix('!') {
            cond.excludes.push(
                Glob::new(item_text).map_err(|e| glob_pattern_error_into_pest_error(e, &item))?,
            );
        } else {
            cond.patterns.push(
                Glob::new(item_text).map_err(|e| glob_pattern_error_into_pest_error(e, &item))?,
            );
        }
    }
    Ok(cond)
}

fn parse_title(s: &str) -> String {
    s[1..s.len() - 1].replace("]]", "]")
}

fn parse_regex_str(s: &str) -> Result<Regex, regex::Error> {
    let framing_char_length = s
        .chars()
        .next()
        .expect("framing characters presence is enforced by grammar")
        .len_utf8();
    Regex::new(&s[framing_char_length..s.len() - framing_char_length])
}

fn parse_regexes_condition(pair: pest::iterators::Pair<Rule>) -> Result<RegexCondition, PestError> {
    let mut cond: RegexCondition = Default::default();
    for item in pair.into_inner() {
        let item_text = item.as_str();
        if let Some(item_text) = item_text.strip_prefix('!') {
            cond.excludes.push(
                parse_regex_str(item_text).map_err(|e| regex_error_into_pest_error(e, &item))?,
            );
        } else {
            cond.patterns.push(
                parse_regex_str(item_text).map_err(|e| regex_error_into_pest_error(e, &item))?,
            );
        }
    }
    Ok(cond)
}

fn parse_files_condition(pair: pest::iterators::Pair<Rule>) -> Result<GlobCondition, PestError> {
    let mut condition: GlobCondition = Default::default();

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::rule_directive_files_inner => {
                condition = parse_globs_condition(
                    item.into_inner().next().unwrap(),
                    ConditionLogic::Positive,
                )?;
            }
            Rule::rule_directive_match => {
                condition.content_conditions.push(ContentConditionNode::new(
                    ContentCondition::Match(parse_regexes_condition(
                        item.into_inner().next().unwrap(),
                    )?),
                ));
            }
            Rule::rule_directive_nomatch => {
                condition.content_conditions.push(ContentConditionNode::new(
                    ContentCondition::NoMatch(parse_regexes_condition(
                        item.into_inner().next().unwrap(),
                    )?),
                ));
            }
            _ => unreachable!(
                "unexpected parser rule type in parse_files_condition {:#?}",
                item
            ),
        }
    }

    Ok(condition)
}

fn parse_rule(
    pair: pest::iterators::Pair<Rule>,
    rule_number: usize,
    source_desc: &str,
) -> Result<RulesetRule, PestError> {
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
                rule.path_conditions.push(parse_files_condition(item)?);
            }
            Rule::rule_directive_nofiles => {
                rule.path_conditions.push(parse_globs_condition(
                    item.into_inner().next().unwrap(),
                    ConditionLogic::Negative,
                )?);
            }
            _ => unreachable!("unexpected parser rule type in parse_rule {:#?}", item),
        }
    }

    Ok(rule)
}

fn parse_file(s: &str, source_desc: &str) -> Result<Config, PestError> {
    let mut config: Config = Default::default();

    let file = ConfigParser::parse(Rule::file, s)
        .map_err(|err| {
            err.renamed_rules(|parser_rule| match *parser_rule {
                Rule::EOI => "end of file".to_owned(),
                Rule::config_directive_root => "\"root\" directive".to_owned(),
                Rule::excluded_glob => {
                    "exclusion glob pattern prefixed with exclamation mark".to_owned()
                }
                Rule::excluded_regexp => {
                    "exclusion regexp pattern prefixed with exclamation mark".to_owned()
                }
                Rule::included_glob => "glob pattern".to_owned(),
                Rule::included_regexp => "regexp pattern".to_owned(),
                Rule::rule_directive_files_inner => "\"files\" condition".to_owned(),
                Rule::rule_directive_match => "\"match\" condition".to_owned(),
                Rule::rule_directive_nofiles => "\"nofiles\" condition".to_owned(),
                Rule::rule_directive_nomatch => "\"nomatch\" condition".to_owned(),
                Rule::rule_directive_tags => "\"tags\" directive".to_owned(),
                Rule::rule_title_outer => "rule title in brackets".to_owned(),
                Rule::simple_glob => "glob pattern".to_owned(),
                // XXX: how to make pest always descend into main rule?
                Rule::file => "omnilinter configuration file".to_owned(),
                other => format!("{:?}", other),
            })
        })?
        .next()
        .unwrap();

    for item in file.into_inner() {
        match item.as_rule() {
            Rule::config_directive_root => {
                let root_pattern = item.into_inner().next().unwrap();
                let root_pattern_text = root_pattern.as_str();
                let mut root_paths = Vec::new();
                for path_or_error in glob::glob(root_pattern_text)
                    .map_err(|e| glob_pattern_error_into_pest_error(e, &root_pattern))?
                {
                    // XXX: convert this loop into try_collect when stabilized
                    root_paths.push(
                        path_or_error.map_err(|e| glob_error_into_pest_error(e, &root_pattern))?,
                    );
                }
                root_paths.sort();
                config.roots.append(&mut root_paths);
            }
            Rule::rule => {
                config.ruleset.rules.push(parse_rule(
                    item,
                    config.ruleset.rules.len(),
                    source_desc,
                )?);
            }
            Rule::EOI => (),
            _ => unreachable!("unexpected parser rule type in from_str {:#?}", item),
        }
    }

    Ok(config)
}

impl Config {
    #[cfg(test)]
    pub fn from_str(s: &str) -> Result<Config, Error> {
        Ok(parse_file(s, "???")?)
    }

    pub fn from_file(path: &Path) -> Result<Config, Error> {
        let path = path.display().to_string();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {}", path))?;

        parse_file(&content, &path)
            .map_err(|e| e.with_path(&path))
            .with_context(|| format!("failed to parse config file {}", path))
    }
}
