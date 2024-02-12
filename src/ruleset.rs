// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

pub use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

enum GlobScope {
    Filenames,
    Paths,
}

pub struct Glob {
    pattern: glob::Pattern,
    scope: GlobScope,
}

impl Glob {
    pub fn new(pattern: &str) -> Result<Self, glob::PatternError> {
        Ok(Self {
            pattern: glob::Pattern::new(pattern.trim_start_matches(std::path::MAIN_SEPARATOR))?,
            scope: if pattern.contains(std::path::MAIN_SEPARATOR_STR) {
                GlobScope::Paths
            } else {
                GlobScope::Filenames
            },
        })
    }

    pub fn matches_path_with(&self, path: &Path, options: glob::MatchOptions) -> bool {
        self.pattern.matches_path_with(
            match self.scope {
                GlobScope::Paths => path,
                GlobScope::Filenames => Path::new(
                    path.file_name()
                        .expect("valid path is expected when matching"),
                ),
            },
            options,
        )
    }
}

pub struct Rule {
    pub title: String,
    pub tags: HashSet<String>,
    pub globs: Option<Vec<Glob>>,
    pub antiglobs: Option<Vec<Glob>>,
    pub regex: Option<Regex>,
    pub antiregex: Option<Regex>,
}

pub struct Ruleset {
    pub rules: Vec<Rule>,
}
