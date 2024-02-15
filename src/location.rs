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
