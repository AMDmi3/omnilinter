// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ruleset::Rule;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Default)]
pub struct MatchResult<'a> {
    pub matches: Vec<Match<'a>>,
}

impl<'a> MatchResult<'a> {
    pub fn new() -> Self {
        Self { matches: vec![] }
    }

    pub fn append(&mut self, mut other: Self) {
        self.matches.append(&mut other.matches);
    }

    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }
}

// Although MatchResult contains Rc<> in individual matches, it is safe
// to transfer across thread boundaries as a whole, because Rc's are only
// shared inside of it
unsafe impl Send for MatchResult<'_> {}

pub struct Match<'a> {
    pub rule: &'a Rule,
    pub root: &'a Path,
    pub file: Option<FileMatch>,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct FileMatch {
    pub path: Rc<PathBuf>,
    pub line: Option<u64>,
}

impl<'a> Match<'a> {
    pub fn for_root(rule: &'a Rule, root: &'a Path) -> Match<'a> {
        Match {
            rule,
            root,
            file: None,
        }
    }

    pub fn for_file(rule: &'a Rule, root: &'a Path, path: Rc<PathBuf>) -> Match<'a> {
        Match {
            rule,
            root,
            file: Some(FileMatch { path, line: None }),
        }
    }

    pub fn for_line(rule: &'a Rule, root: &'a Path, path: Rc<PathBuf>, line: u64) -> Match<'a> {
        Match {
            rule,
            root,
            file: Some(FileMatch {
                path,
                line: Some(line),
            }),
        }
    }
}
