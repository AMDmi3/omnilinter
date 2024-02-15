// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::location::MatchLocation;
use crate::reporter::Reporter;

#[derive(serde::Serialize)]
struct Match {
    message: String,
    root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
}

pub struct JsonReporter {
    reports: Vec<Match>,
}

impl JsonReporter {
    pub fn new() -> Self {
        Self {
            reports: Default::default(),
        }
    }
}

impl Reporter for JsonReporter {
    fn report(&mut self, loc: &MatchLocation, message: &str) {
        self.reports.push(Match {
            root: loc.root.display().to_string(),
            file: loc
                .file
                .as_ref()
                .map(|file| file.path.display().to_string()),
            line: loc
                .file
                .as_ref()
                .map(|file| file.line.map(|line| line + 1))
                .flatten(),
            message: String::from(message),
        });
    }

    fn flush(&self) {
        println!("{}", serde_json::to_string_pretty(&self.reports).unwrap());
    }

    fn has_matches(&self) -> bool {
        !self.reports.is_empty()
    }
}
