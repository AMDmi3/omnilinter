// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ruleset::{Glob, GlobCondition, Regex, RegexCondition};
use std::path::Path;

pub struct GlobMatchingCache<'a> {
    path: &'a Path,
    match_options: glob::MatchOptions,
    #[cfg_attr(not(feature = "matching-cache"), allow(dead_code))]
    cached_matches: Vec<Option<bool>>,
}

impl<'a> GlobMatchingCache<'a> {
    pub fn new(path: &'a Path, match_options: glob::MatchOptions, size: usize) -> Self {
        GlobMatchingCache {
            path,
            match_options,
            cached_matches: vec![None; size],
        }
    }

    pub fn check_pattern_match(&mut self, glob: &Glob) -> bool {
        #[cfg(feature = "matching-cache")]
        {
            let cached = &mut self.cached_matches[glob.get_unique_id()];
            if let Some(cached) = &cached {
                *cached
            } else {
                let computed = glob.matches_path_with(self.path, self.match_options);
                *cached = Some(computed);
                computed
            }
        }
        #[cfg(not(feature = "matching-cache"))]
        {
            glob.matches_path_with(self.path, self.match_options)
        }
    }

    pub fn check_condition_match(&mut self, condition: &GlobCondition) -> bool {
        condition
            .patterns
            .iter()
            .any(|glob| self.check_pattern_match(glob))
            && !condition
                .excludes
                .iter()
                .any(|glob| self.check_pattern_match(glob))
    }
}

pub struct RegexMatchingCache<'a> {
    line: &'a str,
    #[cfg_attr(not(feature = "matching-cache"), allow(dead_code))]
    cached_matches: Vec<Option<bool>>,
}

const IGNORE_MARKER: &str = "omnilinter: ignore";

impl<'a> RegexMatchingCache<'a> {
    pub fn new(line: &'a str, size: usize) -> Self {
        RegexMatchingCache {
            line,
            cached_matches: vec![None; size + 1],
        }
    }

    pub fn check_pattern_match(&mut self, regex: &Regex) -> bool {
        #[cfg(feature = "matching-cache")]
        {
            let cached = &mut self.cached_matches[regex.get_unique_id() + 1];
            if let Some(cached) = &cached {
                *cached
            } else {
                let computed = regex.is_match(self.line);
                *cached = Some(computed);
                computed
            }
        }
        #[cfg(not(feature = "matching-cache"))]
        {
            regex.is_match(self.line)
        }
    }

    pub fn check_ignore_marker_match(&mut self) -> bool {
        #[cfg(feature = "matching-cache")]
        {
            let cached = &mut self.cached_matches[0];
            if let Some(cached) = &cached {
                *cached
            } else {
                let computed = self.line.contains(IGNORE_MARKER);
                *cached = Some(computed);
                computed
            }
        }
        #[cfg(not(feature = "matching-cache"))]
        {
            self.line.contains(IGNORE_MARKER)
        }
    }

    pub fn check_condition_match(&mut self, condition: &RegexCondition) -> bool {
        condition
            .patterns
            .iter()
            .any(|regex| self.check_pattern_match(regex))
            && !condition
                .excludes
                .iter()
                .any(|regex| self.check_pattern_match(regex))
            && !self.check_ignore_marker_match()
    }
}
