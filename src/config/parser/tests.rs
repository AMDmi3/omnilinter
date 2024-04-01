// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::Config;
use crate::ruleset::{ContentCondition, Regex, SizeCondition, SizeOperator};
use testutils::lines;

fn get_first_regex_pattern(config: &Config) -> &Regex {
    match &config.ruleset.rules[0].path_conditions[0].content_conditions[0].condition {
        ContentCondition::Match(regex_condition) => &regex_condition.patterns[0],
        ContentCondition::NoMatch(regex_condition) => &regex_condition.patterns[0],
        _ => panic!(),
    }
}

fn get_first_regex_exclude(config: &Config) -> &Regex {
    match &config.ruleset.rules[0].path_conditions[0].content_conditions[0].condition {
        ContentCondition::Match(regex_condition) => &regex_condition.excludes[0],
        ContentCondition::NoMatch(regex_condition) => &regex_condition.excludes[0],
        _ => panic!(),
    }
}

fn get_first_size_condition(config: &Config) -> &SizeCondition {
    match &config.ruleset.rules[0].path_conditions[0].content_conditions[0].condition {
        ContentCondition::Size(size_condition) => &size_condition,
        _ => panic!(),
    }
}

mod parse_rule_title {
    use super::*;

    #[test]
    fn simple() {
        let text = lines!["[test rule] # ]"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(config.ruleset.rules[0].title, "test rule");
    }

    #[test]
    #[should_panic]
    fn unclosed() {
        let text = lines!["[test"];
        let _config = Config::from_str(text).unwrap();
    }

    #[test]
    #[should_panic]
    fn incorrect_escaping() {
        let text = lines!["[test[ ]rule]"];
        let _config = Config::from_str(text).unwrap();
    }

    #[test]
    #[should_panic]
    fn incorrect_escaping_at_end() {
        let text = lines!["[test rule]]"];
        let _config = Config::from_str(text).unwrap();
    }

    #[test]
    fn correct_escaping() {
        let text = lines![r"[test[ ]]rule] # ]"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(config.ruleset.rules[0].title, "test[ ]rule");
    }
}

mod parse_tags {
    use super::*;

    #[test]
    fn tags() {
        let text = lines!["[]", "tags A B,C"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(config.ruleset.rules[0].tags.len(), 3);
        assert!(config.ruleset.rules[0].tags.contains("a"));
        assert!(config.ruleset.rules[0].tags.contains("b"));
        assert!(config.ruleset.rules[0].tags.contains("c"));
    }
}

mod parse_regexp {
    use super::*;

    #[test]
    fn whitespace() {
        let text = lines!["[]", "files *", "match /f o\to/"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(get_first_regex_pattern(&config).as_str(), "f o\to");
    }

    #[test]
    fn arbitrary_delimiters() {
        let text = lines![
            "[]",
            "files *",
            "match /foo/",
            "[]",
            "files *",
            "match ,foo,"
        ];
        let config = Config::from_str(text).unwrap();
        assert_eq!(get_first_regex_pattern(&config).as_str(), "foo");
        assert_eq!(get_first_regex_pattern(&config).as_str(), "foo");
    }

    #[test]
    fn character_classes_support() {
        let text = lines!["[]", "files *", r"match /\s+/"];
        let config = Config::from_str(text).unwrap();
        assert!(get_first_regex_pattern(&config).is_match(" \t"));
    }

    #[test]
    fn escaping() {
        // XXX: this is incompre this out; consider disallowing escaping in regexps to avoid confusion
        let text = lines!["[]", "files *", r"match /a\\c/"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(get_first_regex_pattern(&config).as_str(), r"a\\c");
    }

    #[test]
    #[should_panic]
    fn missing_delimiter() {
        let text = lines!["[]", "files *", "match /foo"];
        Config::from_str(text).unwrap();
    }

    #[test]
    #[should_panic]
    fn escape_at_eol() {
        let text = lines!["[]", "files *", r"match /foo/bar/"];
        Config::from_str(text).unwrap();
    }

    #[test]
    #[ignore] // TODO: not implemented yet
    fn paired_framing_characters() {
        let text = lines!["[]", "files *", "match (foo)"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(get_first_regex_pattern(&config).as_str(), "foo");
    }

    #[test]
    fn unicode_framing() {
        let text = lines!["[]", "files *", "match 🚧foo🚧"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(get_first_regex_pattern(&config).as_str(), "foo");
    }

    #[test]
    fn exclude() {
        let text = lines!["[]", "files *", r"match /\s+/ !/abc/"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(get_first_regex_exclude(&config).as_str(), "abc");
    }
}

mod parse_size_condition {
    use super::*;

    #[test]
    fn with_space() {
        let text = lines!["[]", "files *", "size >= 123"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            get_first_size_condition(&config).operator,
            SizeOperator::GreaterEqual
        );
        assert_eq!(get_first_size_condition(&config).value, 123);
    }

    #[test]
    fn without_space() {
        let text = lines!["[]", "files *", "size>=123"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            get_first_size_condition(&config).operator,
            SizeOperator::GreaterEqual
        );
        assert_eq!(get_first_size_condition(&config).value, 123);
    }
}

mod parse_globs {
    use super::*;

    #[test]
    fn basic() {
        let text = lines!["[]", "files *"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].patterns[0].as_str(),
            "*"
        );
    }

    #[test]
    fn backslash_escape() {
        let text = lines!["[]", r"files \*\ \*"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].patterns[0].as_str(),
            "[*] [*]" // as per glob::Pattern::escape
        );
    }

    #[test]
    fn dquote() {
        let text = lines!["[]", r#"files "* *""#];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].patterns[0].as_str(),
            "[*] [*]" // as per glob::Pattern::escape
        );
    }

    #[test]
    fn squote() {
        let text = lines!["[]", "files '* *'"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].patterns[0].as_str(),
            "[*] [*]" // as per glob::Pattern::escape
        );
    }

    #[test]
    fn mixed_quoting() {
        let text = lines!["[]", r#"files *\*"*"'*'"#];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].patterns[0].as_str(),
            "*[*][*][*]" // as per glob::Pattern::escape
        );
    }

    #[test]
    fn quite_escaping() {
        let text = lines!["[]", r#"files \"\'"#];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].patterns[0].as_str(),
            r#""'"#
        );
    }

    #[test]
    #[should_panic]
    fn bad_escape() {
        let text = lines!["[]", r"files \"];
        Config::from_str(text).unwrap();
    }

    #[test]
    #[should_panic]
    fn unclosed_squote() {
        let text = lines!["[]", r"files '"];
        Config::from_str(text).unwrap();
    }

    #[test]
    #[should_panic]
    fn unclosed_dquote() {
        let text = lines!["[]", r#"files ""#];
        Config::from_str(text).unwrap();
    }
}

#[test]
#[should_panic]
fn match_without_files() {
    let text = lines!["[]", "match //"];
    Config::from_str(text).unwrap();
}

#[test]
fn empty_lines() {
    let text = lines![
        "",
        "root .",
        "",
        "[]",
        "",
        "tags foo",
        "",
        "nofiles *",
        "",
        "files *",
        "",
        "nomatch /foo/",
        "",
        "match /foo/",
        ""
    ];
    Config::from_str(text).unwrap();
}

mod parse_root {
    use super::*;

    #[test]
    fn glob_expansion() {
        let text = lines!["root *"];
        let config = Config::from_str(text).unwrap();
        // does not matter what it's expanded into, we just check that it is expanded
        assert_ne!(config.roots[0].display().to_string(), "*");
    }

    #[test]
    #[cfg_attr(target_family = "windows", ignore)]
    fn tilde_expansion() {
        let text = lines!["root ~"];
        let config = Config::from_str(text).unwrap();
        assert_ne!(config.roots[0].display().to_string(), "~");
    }
}

#[test]
fn conditionless_rules() {
    let text = lines!["[]", "[]"];
    let config = Config::from_str(text).unwrap();
    assert_eq!(config.ruleset.rules.len(), 2);
}
