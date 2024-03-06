// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

#![feature(let_chains)]

mod applier;
mod config;
mod formatters;
mod r#match;
mod parser;
mod ruleset;

use crate::applier::apply_ruleset;
use crate::config::Config;
use crate::format_text::Palette;
use crate::formatters::json as format_json;
use crate::formatters::text as format_text;
use crate::r#match::MatchResult;
use clap::{Parser, ValueEnum};
use scoped_threadpool::Pool as ThreadPool;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const CONFIG_FILE_NAME: &str = "omnilinter.conf";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Plain text output, grouped by root
    ByRoot,

    /// Plain text output, full paths
    FullPaths,

    /// Plain text output, grouped by rule
    ByRule,

    /// Plain text output, grouped by path
    ByPath,

    /// JSON output
    Json,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path(s) to configuration file(s)
    #[arg(short = 'c', long = "config", value_name = "CONFIG_PATH")]
    config_paths: Vec<PathBuf>,

    /// Only process rules tagged with these values
    #[arg(short = 't', long = "tags", value_name = "TAGS")]
    required_tags: Vec<String>,

    /// Ignore rules tagged with these values
    #[arg(long = "skip-tags", value_name = "TAGS")]
    ignored_tags: Vec<String>,

    /// Output format
    #[arg(short = 'f', long = "format", value_name = "FORMAT", value_enum, default_value_t = OutputFormat::ByRoot)]
    output_format: OutputFormat,

    /// Coloring
    #[arg(long = "color", value_name = "MODE", value_enum, default_value_t = ColorMode::Auto)]
    color_mode: ColorMode,

    /// Palette to use for rule coloring
    #[arg(long = "palette", value_name = "PALETTE", value_enum, default_value_t = Palette::Simple)]
    palette: format_text::Palette,

    /// If any matches are found, exit with given code
    #[arg(long, value_name = "EXITCODE")]
    error_exitcode: Option<i32>,

    /// Number of target directories to process simultaneously
    #[arg(short = 'j', long = "jobs", value_name = "JOBS")]
    num_threads: Option<usize>,

    /// Directories to operate on
    #[arg(value_name = "TARGET_DIR")]
    roots: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    match args.color_mode {
        ColorMode::Always => colored::control::set_override(true),
        ColorMode::Never => colored::control::set_override(false),
        _ => {}
    }

    let mut config = Config::new();

    let default_config_path = match xdg::BaseDirectories::with_prefix("omnilinter") {
        Ok(xdg_dirs) => xdg_dirs.find_config_file(CONFIG_FILE_NAME),
        Err(err) => {
            eprintln!("Warning: cannot set up XDG directories: {}", err);
            None
        }
    };

    if !args.config_paths.is_empty() {
        args.config_paths.iter().for_each(|path| {
            config.merge_from(Config::from_file(path).unwrap());
        });
    } else if let Some(path) = default_config_path {
        config.merge_from(Config::from_file(&path).unwrap());
    } else {
        eprintln!("Error: config file is neither specified on the command line, nor present in the application config directory");
    }

    if config.ruleset.rules.is_empty() {
        eprintln!("Warning: ruleset is empty");
    }

    let roots = if args.roots.is_empty() {
        config.roots
    } else {
        args.roots
    };

    config.ruleset.filter_by_tags(
        &HashSet::from_iter(args.required_tags),
        &HashSet::from_iter(args.ignored_tags),
    );

    let ruleset = config.ruleset.compile();

    let result = {
        let result = Arc::new(Mutex::new(MatchResult::new()));

        let num_threads = args.num_threads.unwrap_or_else(num_cpus::get);
        let mut pool = ThreadPool::new(num_threads.try_into().unwrap_or(1));

        pool.scoped(|scope| {
            let ruleset = &ruleset;
            for root in &roots {
                let result = result.clone();
                scope.execute(move || {
                    let res = apply_ruleset(ruleset, root);
                    result.lock().unwrap().append(res);
                });
            }
        });

        Arc::into_inner(result).unwrap().into_inner().unwrap()
    };

    match args.output_format {
        OutputFormat::ByRoot => {
            format_text::format_matches(&result, format_text::Format::ByRootGrouped, args.palette)
        }
        OutputFormat::FullPaths => {
            format_text::format_matches(&result, format_text::Format::ByRootFullPaths, args.palette)
        }
        OutputFormat::ByRule => {
            format_text::format_matches(&result, format_text::Format::ByRule, args.palette)
        }
        OutputFormat::ByPath => {
            format_text::format_matches(&result, format_text::Format::ByPath, args.palette)
        }
        OutputFormat::Json => format_json::format_matches(&result),
    }

    if let Some(error_exitcode) = args.error_exitcode {
        if !result.is_empty() {
            std::process::exit(error_exitcode);
        }
    }
}
