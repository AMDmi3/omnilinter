// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::r#match::{Match, MatchResult};
use colored::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Format {
    ByRule,
    ByRootGrouped,
    ByRootFullPaths,
    ByPath,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, clap::ValueEnum)]
pub enum Palette {
    /// No specific rule coloring
    None,

    /// Use one of 4 neutral colors (green, blue, magenta, cyan) + their bright variants for each
    /// rule
    Simple,

    /// Use color based on severity guessed from tags (error/fatal/critical; warning; minor)
    Severity,

    /// Use wider true color palette
    TrueColor,
}

fn order_by_roots(lhs: &&Match, rhs: &&Match) -> std::cmp::Ordering {
    lhs.root
        .cmp(rhs.root)
        .then_with(|| lhs.file.cmp(&rhs.file)) // note: sorts by both path and line
        .then_with(|| lhs.rule.number.cmp(&rhs.rule.number))
}

fn order_by_rules(lhs: &&Match, rhs: &&Match) -> std::cmp::Ordering {
    lhs.rule
        .number
        .cmp(&rhs.rule.number)
        .then_with(|| lhs.root.cmp(&rhs.root))
        .then_with(|| lhs.file.cmp(&rhs.file)) // note: sorts by both path and line
        .then_with(|| lhs.rule.number.cmp(&rhs.rule.number))
}

fn sort_matches(matches: &mut Vec<&Match>, format: Format) {
    match format {
        Format::ByRule => matches.sort_unstable_by(order_by_rules),
        Format::ByRootGrouped | Format::ByRootFullPaths | Format::ByPath => {
            matches.sort_unstable_by(order_by_roots)
        }
    }
}

fn get_path_rel(m: &Match) -> Option<String> {
    if let Some(file) = &m.file {
        Some(file.path.display().to_string())
    } else {
        None
    }
}

fn get_path_abs(m: &Match) -> String {
    if let Some(file) = &m.file {
        m.root.join(file.path.as_ref()).display().to_string()
    } else {
        m.root.display().to_string()
    }
}

fn get_title(m: &Match, palette: Palette) -> String {
    let title = m.rule.title.clone();

    match palette {
        Palette::None => title,
        Palette::Simple => match fxhash::hash32(&m.rule.title) >> 4 & 0b111 {
            0 => title.green(),
            1 => title.blue(),
            2 => title.magenta(),
            3 => title.cyan(),
            4 => title.bright_green(),
            5 => title.bright_blue(),
            6 => title.bright_magenta(),
            7 => title.bright_cyan(),
            _ => unreachable!(),
        }
        .to_string(),
        Palette::Severity => {
            let tags = &m.rule.tags;
            if tags.contains("error") || tags.contains("fatal") || tags.contains("critical") {
                title.red().to_string()
            } else if tags.contains("warning") {
                title.yellow().to_string()
            } else if tags.contains("minor") {
                title
            } else {
                title.green().to_string()
            }
        }
        Palette::TrueColor => {
            let hash = fxhash::hash32(&m.rule.title);
            let r = hash >> 4 & 0b1111;
            let g = hash >> 8 & 0b1111;
            let b = hash >> 12 & 0b1111;
            let r: u8 = (128 + r * 8).try_into().unwrap();
            let g: u8 = (128 + g * 8).try_into().unwrap();
            let b: u8 = (160 + b * 6).try_into().unwrap();
            title.truecolor(r, g, b).to_string()
        }
    }
}

fn get_group(m: &Match, format: Format, palette: Palette) -> Option<String> {
    match format {
        Format::ByRule => Some(get_title(m, palette)),
        Format::ByRootGrouped => Some(format!("{}", m.root.display().to_string().yellow().bold())),
        Format::ByRootFullPaths => None,
        Format::ByPath => Some(format!("{}", get_path_abs(m).yellow().bold())),
    }
}

fn get_line(m: &Match) -> Option<usize> {
    if let Some(file) = &m.file {
        file.line
    } else {
        None
    }
}

fn get_message(m: &Match, format: Format, palette: Palette) -> Option<String> {
    match format {
        Format::ByRule => None,
        _ => Some(get_title(m, palette)),
    }
}

fn get_path(m: &Match, format: Format) -> Option<String> {
    match format {
        Format::ByRule => Some(get_path_abs(m)),
        Format::ByRootGrouped => get_path_rel(m),
        Format::ByRootFullPaths => Some(get_path_abs(m)),
        Format::ByPath => None,
    }
}

fn get_location(m: &Match, format: Format) -> Option<String> {
    match (get_path(m, format), get_line(m)) {
        (Some(path), Some(line)) => Some(format!("{}{}{}", path.bold(), ":".cyan(), line + 1)),
        (Some(path), None) => Some(format!("{}", path.bold())),
        (None, Some(line)) => Some(format!("line {}", line + 1)),
        (None, None) => None, //Some("general".into()),
                              //(None, None) => Some("???".into()),
    }
}

fn get_group_prefix(_format: Format) -> &'static str {
    ""
}

fn get_match_prefix(format: Format) -> &'static str {
    match format {
        Format::ByRootFullPaths => "",
        _ => "  ",
    }
}

pub fn format_matches(match_result: &MatchResult, format: Format, palette: Palette) {
    let mut matches: Vec<&Match> = match_result.matches.iter().collect();

    sort_matches(&mut matches, format);

    let mut prev_group: Option<String> = None;

    for m in matches {
        let current_group = get_group(&m, format, palette);
        if current_group != prev_group {
            if let Some(group) = &current_group {
                println!("{}{}", get_group_prefix(format), group);
            }
            prev_group = current_group;
        }

        match (get_location(m, format), get_message(m, format, palette)) {
            (Some(l), Some(m)) => println!("{}{}{} {}", get_match_prefix(format), l, ":".cyan(), m),
            (Some(l), None) => println!("{}{}", get_match_prefix(format), l),
            (None, Some(m)) => println!("{}{}", get_match_prefix(format), m),
            (None, None) => {}
        }
    }
}
