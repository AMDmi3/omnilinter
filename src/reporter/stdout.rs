// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::location::MatchLocation;
use crate::reporter::Reporter;
use std::path::PathBuf;

pub struct ReporterOptions {
    pub full_paths: bool,
}

pub struct StdoutReporter {
    options: ReporterOptions,
    prev_root: PathBuf,
    has_matches: bool,
}

impl StdoutReporter {
    pub fn new(options: ReporterOptions) -> Self {
        Self {
            options,
            prev_root: Default::default(),
            has_matches: false,
        }
    }
}

impl Reporter for StdoutReporter {
    fn report(&mut self, location: &MatchLocation, message: &str) {
        if location.root != self.prev_root {
            println!("in {}", location.root.display());
            self.prev_root = location.root.to_path_buf();
        }

        if let Some(file) = &location.file {
            let path_display = if self.options.full_paths {
                location.root.join(file.path).display().to_string()
            } else {
                file.path.display().to_string()
            };
            if let Some(line) = file.line {
                println!("- {}:{}: {}", path_display, line, message);
            } else {
                println!("- {}: {}", path_display, message);
            }
        } else {
            println!("- {}", message);
        }

        self.has_matches = true;
    }

    fn flush(&self) {}

    fn has_matches(&self) -> bool {
        self.has_matches
    }
}
