// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::parser::ParsedConfig;
use std::fs;
use std::path::Path;

impl ParsedConfig {
    pub fn from_yaml_str(s: &str) -> Result<ParsedConfig, serde_yaml::Error> {
        serde_yaml::from_str(&s)
    }

    pub fn from_yaml_file(path: &Path) -> Result<ParsedConfig, serde_yaml::Error> {
        Self::from_yaml_str(&fs::read_to_string(path).unwrap())
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

        let config = ParsedConfig::from_yaml_str(text).unwrap().into_config();

        assert_eq!(config.ruleset.rules.len(), 1);
    }
}
