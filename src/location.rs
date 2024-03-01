// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

pub struct MatchLocationFile<'a> {
    pub path: &'a Path,
    pub line: Option<usize>,
}

pub struct MatchLocation<'a> {
    pub root: &'a Path,
    pub file: Option<MatchLocationFile<'a>>,
}

impl<'a> MatchLocation<'a> {
    pub fn for_root(root: &'a Path) -> MatchLocation<'a> {
        MatchLocation { root, file: None }
    }

    pub fn for_file(root: &'a Path, path: &'a Path) -> MatchLocation<'a> {
        MatchLocation {
            root,
            file: Some(MatchLocationFile { path, line: None }),
        }
    }

    pub fn for_line(root: &'a Path, path: &'a Path, line: usize) -> MatchLocation<'a> {
        MatchLocation {
            root,
            file: Some(MatchLocationFile {
                path,
                line: Some(line),
            }),
        }
    }
}
