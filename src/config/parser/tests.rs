// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::Config;

macro_rules! lines {
    ($($s:expr),+) => {{
        concat!(
            $($s,'\n',)+
        )
    }};
}

mod parse_rule_title {
    use crate::config::Config;

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

mod parse_regexp {
    use crate::config::Config;

    #[test]
    fn whitespace() {
        let text = lines!["[]", "files *", "match /f o\to/"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].content_conditions[0].patterns[0].as_str(),
            "f o\to"
        );
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
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].content_conditions[0].patterns[0].as_str(),
            "foo"
        );
        assert_eq!(
            config.ruleset.rules[1].path_conditions[0].content_conditions[0].patterns[0].as_str(),
            "foo"
        );
    }

    #[test]
    fn character_classes_support() {
        let text = lines!["[]", "files *", r"match /\s+/"];
        let config = Config::from_str(text).unwrap();
        assert!(
            config.ruleset.rules[0].path_conditions[0].content_conditions[0].patterns[0]
                .is_match(" \t")
        );
    }

    #[test]
    fn escaping() {
        // XXX: this is incompre this out; consider disallowing escaping in regexps to avoid confusion
        let text = lines!["[]", "files *", r"match /a\\c/"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].content_conditions[0].patterns[0].as_str(),
            r"a\\c"
        );
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
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].content_conditions[0].patterns[0].as_str(),
            "foo"
        );
    }

    #[test]
    fn unicode_framing() {
        let text = lines!["[]", "files *", "match 🚧foo🚧"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].content_conditions[0].patterns[0].as_str(),
            "foo"
        );
    }

    #[test]
    fn exclude() {
        let text = lines!["[]", "files *", r"match /\s+/ !/abc/"];
        let config = Config::from_str(text).unwrap();
        assert_eq!(
            config.ruleset.rules[0].path_conditions[0].content_conditions[0].excludes[0].as_str(),
            "abc"
        );
    }
}

#[test]
fn multiple_files() {
    let text = lines!["[]", "files *", "files *"];
    let config = Config::from_str(text).unwrap();
    assert_eq!(config.ruleset.rules[0].path_conditions.len(), 2);
}

#[test]
fn multiple_nofiles() {
    let text = lines!["[]", "nofiles *", "nofiles *"];
    let config = Config::from_str(text).unwrap();
    assert_eq!(config.ruleset.rules[0].path_conditions.len(), 2);
}

#[test]
#[should_panic]
fn duplicate_conditions() {
    let text = lines!["[]", "files *", "match //", "match //"];
    Config::from_str(text).unwrap();
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
        "root /nonexistent",
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

#[test]
fn conditionless_rules() {
    let text = lines!["[]", "[]"];
    let config = Config::from_str(text).unwrap();
    assert_eq!(config.ruleset.rules.len(), 2);
}