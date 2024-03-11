// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::Config;
use crate::ruleset::{ConditionLogic, Glob, GlobCondition, Regex, RegexCondition, Rule};

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

fn dump_content_condition(content_condition: &RegexCondition) {
    let directive = match content_condition.logic {
        ConditionLogic::Positive => "match",
        ConditionLogic::Negative => "nomatch",
    };
    print!("        {}", directive);
    content_condition.patterns.iter().for_each(|pattern| {
        print!(" ");
        dump_regex(pattern);
    });
    content_condition.excludes.iter().for_each(|pattern| {
        print!(" !");
        dump_regex(pattern);
    });
    println!();
}

fn dump_path_condition(path_condition: &GlobCondition) {
    let directive = match path_condition.logic {
        ConditionLogic::Positive => "files",
        ConditionLogic::Negative => "nofiles",
    };
    print!("    {}", directive);
    path_condition.patterns.iter().for_each(|pattern| {
        print!(" ");
        dump_glob(pattern);
    });
    path_condition.excludes.iter().for_each(|pattern| {
        print!(" !");
        dump_glob(pattern);
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
    fn dump_directives(&self) -> bool {
        for root in &self.roots {
            println!("root {}", root.display());
        }

        !self.roots.is_empty()
    }

    fn dump_rules(&self) -> bool {
        for rule in &self.ruleset.rules {
            dump_rule(rule);
            println!();
        }

        !self.ruleset.rules.is_empty()
    }

    pub fn dump(&self) {
        if self.dump_directives() {
            println!();
        }

        self.dump_rules();
    }
}
