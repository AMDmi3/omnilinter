// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::location::{MatchLocation, MatchLocationFile};
use std::path::Path;

pub struct RootMatchContext<'a> {
    pub root: &'a Path,
}

impl<'a> RootMatchContext<'a> {
    pub fn to_location(&self) -> MatchLocation {
        MatchLocation {
            root: self.root,
            file: None,
        }
    }
}

pub struct FileMatchContext<'a> {
    pub root: &'a Path,
    pub file: &'a Path,
}

impl<'a> FileMatchContext<'a> {
    pub fn from_root(root: &RootMatchContext<'a>, file: &'a Path) -> FileMatchContext<'a> {
        FileMatchContext {
            root: root.root,
            file,
        }
    }

    pub fn to_location(&self) -> MatchLocation {
        MatchLocation {
            root: self.root,
            file: Some(MatchLocationFile {
                path: self.file,
                line: None,
            }),
        }
    }

    pub fn to_location_with_line(&self, line: usize) -> MatchLocation {
        MatchLocation {
            root: self.root,
            file: Some(MatchLocationFile {
                path: self.file,
                line: Some(line),
            }),
        }
    }
}
