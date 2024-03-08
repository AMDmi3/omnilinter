// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod dumper;
mod parser;

use crate::ruleset::Ruleset;
use std::path::PathBuf;

#[derive(Default)]
pub struct Config {
    pub ruleset: Ruleset,
    pub roots: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }

    pub fn merge_from(&mut self, mut other: Config) {
        self.ruleset.rules.append(&mut other.ruleset.rules);
        self.roots.append(&mut other.roots);
    }
}
