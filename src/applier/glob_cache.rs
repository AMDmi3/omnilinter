// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ruleset::{Glob, GlobCondition};
use std::collections::HashMap;
use std::path::Path;

pub struct GlobMatchingCache<'a> {
    path: &'a Path,
    match_options: glob::MatchOptions,
    glob_matches: HashMap<&'a Glob, bool>,
}

impl<'a> GlobMatchingCache<'a> {
    pub fn new(path: &'a Path, match_options: glob::MatchOptions) -> Self {
        GlobMatchingCache {
            path,
            match_options,
            glob_matches: Default::default(),
        }
    }

    pub fn check_glob_match(&mut self, glob: &'a Glob) -> bool {
        // XXX: benchmarks shows that the cache yields 2x performance
        // regression compared to straightforward glob matching in all
        // cases except "multiple rules with same pattern". This is
        // quite expected as we still have to match each pattern from
        // scratch plus we have cache overhead. In practice, howerver
        // rules are expected to have same patterns (such as a lot of
        // rules for *.py), so the cache still makes sence. Also, the
        // regression can be fixed by caching by pre-grouping globs
        // (during some kind of ruleset compilation phase) and assiging
        // unique incrementing ids to them, then indexing this cache
        // by these ids instead of computing hashes.
        *self
            .glob_matches
            .entry(glob)
            .or_insert_with(|| glob.matches_path_with(self.path, self.match_options))
    }

    pub fn check_condition_match(&mut self, condition: &'a GlobCondition) -> bool {
        condition
            .patterns
            .iter()
            .any(|glob| self.check_glob_match(glob))
            && !condition
                .excludes
                .iter()
                .any(|glob| self.check_glob_match(glob))
    }
}
