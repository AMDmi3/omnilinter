// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ruleset::enumerator::Enumerator;
use std::path::Path;

#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(not(feature = "coverage"), derive(Debug))]
enum GlobScope {
    Filenames,
    Paths,
}

#[derive(Clone)]
pub struct Glob {
    pattern: glob::Pattern,
    scope: GlobScope,
    unique_id: usize,
}

#[cfg(not(feature = "coverage"))]
impl std::fmt::Debug for Glob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Glob")
            .field("pattern", &self.pattern.as_str())
            .field("scope", &self.scope)
            .field("unique_id", &self.unique_id)
            .finish()
    }
}

impl Glob {
    pub fn new(pattern: &str) -> Result<Self, glob::PatternError> {
        Ok(Self {
            pattern: glob::Pattern::new(pattern.trim_start_matches(std::path::is_separator))?,
            scope: if pattern.chars().any(std::path::is_separator) {
                GlobScope::Paths
            } else {
                GlobScope::Filenames
            },
            unique_id: usize::MAX,
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

    pub fn as_str(&self) -> &str {
        self.pattern.as_str()
    }

    pub fn enumerate_with(&mut self, enumerator: &mut Enumerator) {
        self.unique_id = enumerator.get_id(
            &(self.pattern.as_str().to_owned()
                + match self.scope {
                    GlobScope::Filenames => "f",
                    GlobScope::Paths => "p",
                }),
        );
    }

    #[cfg_attr(not(feature = "matching-cache"), allow(dead_code))]
    pub fn get_unique_id(&self) -> usize {
        debug_assert!(self.unique_id != usize::MAX, "Glob is not enumerated");
        self.unique_id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashSet;

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn not_enumerated() {
        let g = Glob::new("README.md").unwrap();
        g.get_unique_id();
    }

    fn calc_unique_globs(patterns: &[&str]) -> usize {
        let mut e = Enumerator::new();
        patterns
            .iter()
            .map(|pattern| {
                let mut g = Glob::new(pattern).unwrap();
                g.enumerate_with(&mut e);
                g.get_unique_id()
            })
            .collect::<HashSet<_>>()
            .len()
    }

    #[test]
    fn enumeration_same() {
        assert_eq!(calc_unique_globs(&["README.md", "README.md"]), 1);
    }

    #[test]
    fn enumeration_different() {
        assert_eq!(calc_unique_globs(&["README.md", "/README.md"]), 2);
    }
}
