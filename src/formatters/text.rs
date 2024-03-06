// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::r#match::{Match, MatchResult};

#[derive(Clone, Copy)]
pub enum Format {
    ByRule,
    ByRootGrouped,
    ByRootFullPaths,
}

fn order_by_roots(lhs: &&Match, rhs: &&Match) -> std::cmp::Ordering {
    lhs.root
        .cmp(rhs.root)
        .then_with(|| lhs.file.cmp(&rhs.file))
        .then_with(|| lhs.rule.number.cmp(&rhs.rule.number))
}

fn order_by_rules(lhs: &&Match, rhs: &&Match) -> std::cmp::Ordering {
    lhs.rule
        .number
        .cmp(&rhs.rule.number)
        .then_with(|| lhs.root.cmp(&rhs.root))
        .then_with(|| lhs.file.cmp(&rhs.file))
        .then_with(|| lhs.rule.number.cmp(&rhs.rule.number))
}

fn sort_matches(matches: &mut Vec<&Match>, format: Format) {
    match format {
        Format::ByRule => matches.sort_unstable_by(order_by_rules),
        Format::ByRootGrouped | Format::ByRootFullPaths => matches.sort_unstable_by(order_by_roots),
    }
}

fn get_group(m: &Match, format: Format) -> Option<String> {
    match format {
        Format::ByRule => Some(m.rule.title.clone()),
        Format::ByRootGrouped => Some(m.root.display().to_string()),
        Format::ByRootFullPaths => None,
    }
}

fn get_message(m: &Match, format: Format) -> Option<String> {
    match format {
        Format::ByRule => None,
        _ => Some(m.rule.title.clone()),
    }
}

fn get_location(m: &Match, format: Format) -> Option<String> {
    let (full_path, always_path) = match format {
        Format::ByRule => (true, true),
        Format::ByRootGrouped => (false, false),
        Format::ByRootFullPaths => (true, true),
    };

    if let Some(file) = &m.file {
        let path_display = if full_path {
            m.root.join(file.path.as_ref()).display().to_string()
        } else {
            file.path.display().to_string()
        };
        if let Some(line) = file.line {
            Some(format!("{}:{}", path_display, line + 1))
        } else {
            Some(path_display)
        }
    } else {
        if always_path {
            Some(m.root.display().to_string())
        } else {
            None
        }
    }
}

fn get_prefix(format: Format) -> &'static str {
    match format {
        Format::ByRootFullPaths => "",
        _ => "  ",
    }
}

fn format_line(m: &Match, format: Format) {
    match (get_location(m, format), get_message(m, format)) {
        (Some(l), Some(m)) => println!("{}{}: {}", get_prefix(format), l, m),
        (Some(l), None) => println!("{}{}", get_prefix(format), l),
        (None, Some(m)) => println!("{}{}", get_prefix(format), m),
        (None, None) => {}
    }
}

pub fn format_matches(match_result: &MatchResult, format: Format) {
    let mut matches: Vec<&Match> = match_result.matches.iter().collect();

    sort_matches(&mut matches, format);

    let mut prev_group: Option<String> = None;

    for m in matches {
        let current_group = get_group(&m, format);
        if current_group != prev_group {
            if let Some(group) = &current_group {
                println!("{}:", group);
            }
            prev_group = current_group;
        }

        format_line(&m, format);
    }
}
