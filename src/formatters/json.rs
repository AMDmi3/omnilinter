// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::r#match::MatchResult;

#[derive(serde::Serialize)]
struct Match<'a> {
    message: &'a String,
    root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u64>,
}

pub fn format_matches(match_result: &MatchResult) {
    println!("[");
    for (n, m) in match_result.matches.iter().enumerate() {
        if n > 0 {
            println!(",");
        }
        print!(
            "{}",
            serde_json::to_string_pretty(&Match {
                message: &m.rule.title,
                root: m.root.display().to_string(),
                file: m.file.as_ref().map(|file| file.path.display().to_string()),
                line: m
                    .file
                    .as_ref()
                    .and_then(|file| file.line.map(|line| line + 1)),
            })
            .unwrap()
        );
    }
    println!("]");
}
