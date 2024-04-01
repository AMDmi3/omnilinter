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
    RegexCondition, SizeCondition, SizeOperator,
};
use anyhow::{Context, Error};
use pest::Parser;
use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

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

fn error_into_pest_error<T: std::error::Error>(
    e: T,
    item: &pest::iterators::Pair<Rule>,
) -> PestError {
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

fn parse_glob_str(s: &str) -> Result<Glob, glob::PatternError> {
    // see https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_02

    #[derive(PartialEq, Eq)]
    enum QuoteMode {
        None,
        Single,
        Double,
    }

    let mut output = String::new();
    let mut quoted_fragment = String::new();
    let mut escaped = false;
    let mut quoted = QuoteMode::None;

    for c in s.chars() {
        match quoted {
            QuoteMode::Single => {
                if c == '\'' {
                    output.push_str(&glob::Pattern::escape(&quoted_fragment));
                    quoted = QuoteMode::None;
                } else {
                    quoted_fragment.push(c);
                }
            }
            QuoteMode::Double => {
                if escaped {
                    quoted_fragment.push(c);
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    output.push_str(&glob::Pattern::escape(&quoted_fragment));
                    quoted = QuoteMode::None;
                } else {
                    quoted_fragment.push(c);
                }
            }
            QuoteMode::None => {
                if escaped {
                    output.push_str(&glob::Pattern::escape(&c.to_string()));
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '\'' {
                    quoted_fragment = String::new();
                    quoted = QuoteMode::Single;
                } else if c == '"' {
                    quoted_fragment = String::new();
                    quoted = QuoteMode::Double;
                } else {
                    output.push(c);
                }
            }
        }
    }

    debug_assert!(
        !escaped && quoted == QuoteMode::None,
        "quoting and escaping consistency is expected to be guranteed by the grammar"
    );
    Glob::new(&output)
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
                parse_glob_str(item_text)
                    .map_err(|e| glob_pattern_error_into_pest_error(e, &item))?,
            );
        } else {
            cond.patterns.push(
                parse_glob_str(item_text)
                    .map_err(|e| glob_pattern_error_into_pest_error(e, &item))?,
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
        .expect("framing characters presence expected to be guranteed by the grammar")
        .len_utf8();
    Regex::new(&s[framing_char_length..s.len() - framing_char_length])
}

fn parse_regexes_condition(pair: pest::iterators::Pair<Rule>) -> Result<RegexCondition, PestError> {
    let mut cond: RegexCondition = Default::default();
    for item in pair.into_inner() {
        let item_text = item.as_str();
        if let Some(item_text) = item_text.strip_prefix('!') {
            cond.excludes
                .push(parse_regex_str(item_text).map_err(|e| error_into_pest_error(e, &item))?);
        } else {
            cond.patterns
                .push(parse_regex_str(item_text).map_err(|e| error_into_pest_error(e, &item))?);
        }
    }
    Ok(cond)
}

fn parse_size_condition(pair: pest::iterators::Pair<Rule>) -> Result<SizeCondition, PestError> {
    let mut iter = pair.into_inner().into_iter();
    let operator = iter.next().unwrap();
    let value = iter.next().unwrap();

    Ok(SizeCondition {
        operator: match operator.as_str() {
            ">=" => SizeOperator::GreaterEqual,
            ">" => SizeOperator::Greater,
            "<=" => SizeOperator::LessEqual,
            "<" => SizeOperator::Less,
            "=" => SizeOperator::Equal,
            "==" => SizeOperator::Equal,
            "!=" => SizeOperator::NotEqual,
            "<>" => SizeOperator::NotEqual,
            other => unreachable!("unexpected size operator {other}",),
        },
        value: value
            .as_str()
            .parse()
            .map_err(|e| error_into_pest_error(e, &value))?,
    })
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
            Rule::rule_directive_size => {
                condition.content_conditions.push(ContentConditionNode::new(
                    ContentCondition::Size(parse_size_condition(
                        item.into_inner().next().unwrap(),
                    )?),
                ));
            }
            Rule::rule_directive_lines => {
                condition.content_conditions.push(ContentConditionNode::new(
                    ContentCondition::Lines(parse_size_condition(
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
    config_path: &Path,
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
                        config_path.display().to_string(),
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

fn expand_config_directive_glob(
    pattern: &str,
    current_path: &Path,
) -> Result<glob::Paths, glob::PatternError> {
    let pattern = if let Some(pattern_relative_to_home) = pattern.strip_prefix('~') {
        if let Ok(home) = std::env::var("HOME") {
            Cow::from(home + pattern_relative_to_home)
        } else {
            return Err(glob::PatternError {
                pos: 0,
                msg: "cannot expand tilde (is $HOME set?)",
            });
        }
    } else {
        Cow::from(pattern)
    };

    let as_path = Path::new(&*pattern);
    if as_path.is_absolute() {
        glob::glob(&pattern)
    } else {
        glob::glob(
            current_path
                .parent()
                .expect("parent path for the current config should be extractible")
                .join(as_path)
                .to_str()
                .expect("included path should be valid UTF-8 string"),
        )
    }
}

fn parse_config_directive_glob(
    pair: pest::iterators::Pair<Rule>,
    current_path: &Path,
) -> Result<Vec<PathBuf>, PestError> {
    let mut res = Vec::new();
    for path_or_error in expand_config_directive_glob(pair.as_str(), current_path)
        .map_err(|e| glob_pattern_error_into_pest_error(e, &pair))?
    {
        // XXX: convert this loop into try_collect when stabilized
        res.push(path_or_error.map_err(|e| error_into_pest_error(e, &pair))?);
    }
    res.sort();
    Ok(res)
}

fn parse_file(config_text: &str, config_path: &Path) -> Result<Config, PestError> {
    let mut config: Config = Default::default();

    let file = ConfigParser::parse(Rule::file, config_text)
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
                config.roots.append(&mut parse_config_directive_glob(
                    item.into_inner().next().unwrap(),
                    config_path,
                )?);
            }
            Rule::config_directive_include => {
                config.includes.append(&mut parse_config_directive_glob(
                    item.into_inner().next().unwrap(),
                    config_path,
                )?);
            }
            Rule::rule => {
                config.ruleset.rules.push(parse_rule(
                    item,
                    config.ruleset.rules.len(),
                    config_path,
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
        Ok(parse_file(s, Path::new("???"))?)
    }

    pub fn from_file(path: &Path) -> Result<Config, Error> {
        let content = fs::read_to_string(&path).with_context(|| {
            format!("failed to read config file {}", path.display().to_string())
        })?;

        parse_file(&content, &path)
            .map_err(|e| e.with_path(&path.display().to_string()))
            .with_context(|| format!("failed to parse config file {}", path.display().to_string()))
    }

    pub fn from_file_expand_includes(path: &Path) -> Result<Config, Error> {
        let mut config = Config::new();
        let mut queue = VecDeque::new();
        let mut seen_paths = HashSet::new();

        queue.push_back(path.to_path_buf());

        while let Some(current_path) = queue.pop_front() {
            config.merge_from(Self::from_file(&current_path)?);
            config.includes.drain(0..).for_each(|include| {
                if seen_paths.insert(include.clone()) {
                    queue.push_back(include);
                }
            });
        }

        Ok(config)
    }
}
