pub use regex::Regex;
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
            pattern: glob::Pattern::new(pattern.trim_start_matches('/'))?,
            scope: if pattern.contains("/") {
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
    pub globs: Option<Vec<Glob>>,
    pub antiglobs: Option<Vec<Glob>>,
    pub regex: Option<Regex>,
    pub antiregex: Option<Regex>,
}

pub struct Ruleset {
    pub rules: Vec<Rule>,
}
