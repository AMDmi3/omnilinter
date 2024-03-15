// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::Config;
use crate::ruleset::{
    ConditionLogic, ContentCondition, ContentConditionNode, Glob, GlobCondition, Regex,
    RegexCondition, Rule, SizeCondition, SizeOperator,
};

fn dump_glob(glob: &Glob) {
    print!("{}", glob.as_str());
}

const REGEX_FRAMING_CHARACTERS: &str = "/\"'|#%*\\";

fn dump_regex(regex: &Regex) {
    let regex = regex.as_str();
    for framing_character in REGEX_FRAMING_CHARACTERS.chars() {
        if !regex.contains(framing_character) {
            print!("{}{}{}", framing_character, regex, framing_character);
            return;
        }
    }
    panic!("unable to dump regex {regex}, could not find suitable framing character");
}

fn dump_regex_condition_args(regex_condition: &RegexCondition) {
    regex_condition.patterns.iter().for_each(|regex| {
        print!(" ");
        dump_regex(regex);
    });
    regex_condition.excludes.iter().for_each(|regex| {
        print!(" !");
        dump_regex(regex);
    });
}

fn dump_size_condition_args(size_condition: &SizeCondition) {
    print!(
        " {} {}",
        match size_condition.operator {
            SizeOperator::GreaterEqual => ">=",
            SizeOperator::Greater => ">",
            SizeOperator::LessEqual => "<=",
            SizeOperator::Less => "<",
            SizeOperator::Equal => "==",
            SizeOperator::NotEqual => "!=",
        },
        size_condition.value
    );
}

fn dump_content_condition(content_condition_node: &ContentConditionNode) {
    match &content_condition_node.condition {
        ContentCondition::Match(regex_condition) => {
            print!("        match");
            dump_regex_condition_args(&regex_condition);
        }
        ContentCondition::NoMatch(regex_condition) => {
            print!("        nomatch");
            dump_regex_condition_args(&regex_condition);
        }
        ContentCondition::Size(size_condition) => {
            print!("        size");
            dump_size_condition_args(&size_condition);
        }
        ContentCondition::Lines(size_condition) => {
            print!("        lines");
            dump_size_condition_args(&size_condition);
        }
    }
    println!();
}

fn dump_path_condition(path_condition: &GlobCondition) {
    let directive = match path_condition.logic {
        ConditionLogic::Positive => "files",
        ConditionLogic::Negative => "nofiles",
    };
    print!("    {}", directive);
    path_condition.patterns.iter().for_each(|glob| {
        print!(" ");
        dump_glob(glob);
    });
    path_condition.excludes.iter().for_each(|glob| {
        print!(" !");
        dump_glob(glob);
    });
    println!();
    path_condition
        .content_conditions
        .iter()
        .for_each(dump_content_condition);
}

fn dump_tags(rule: &Rule) {
    let mut tags: Vec<_> = rule.tags.iter().map(|tag| tag.as_str()).collect();
    tags.sort();
    println!("{}", tags.join(","));
}

fn dump_rule(rule: &Rule) {
    println!("[{}]", rule.title.replace(']', "]]"));
    if !rule.tags.is_empty() {
        print!("    tags ");
        dump_tags(rule);
    }
    rule.path_conditions.iter().for_each(dump_path_condition);
}

impl Config {
    fn dump_directives(&self, had_before_content: &mut bool) {
        for root in &self.roots {
            println!("root {}", root.display());
            *had_before_content = true;
        }
    }

    fn dump_rules(&self, had_before_content: &mut bool) {
        for rule in &self.ruleset.rules {
            if *had_before_content {
                println!();
            }
            dump_rule(rule);
            *had_before_content = true;
        }
    }

    pub fn dump(&self) {
        let mut had_before_content = false;
        self.dump_directives(&mut had_before_content);
        self.dump_rules(&mut had_before_content);
    }
}
