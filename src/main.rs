// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod applier;
mod config;
mod formatters;
mod r#match;
mod ruleset;

use crate::applier::apply_ruleset;
use crate::config::Config;
use crate::format_text::Palette;
use crate::formatters::json as format_json;
use crate::formatters::text as format_text;
use crate::r#match::MatchResult;
use anyhow::{bail, Error};
use clap::{Parser, ValueEnum};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::ExitCode;

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
    #[arg(
        short = 't',
        long = "tags",
        value_name = "TAG[,...]",
        value_delimiter = ','
    )]
    required_tags: Vec<String>,

    /// Ignore rules tagged with these values
    #[arg(long = "skip-tags", value_name = "TAG[,...]", value_delimiter = ',')]
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
    error_exitcode: Option<u8>,

    /// Number of target directories to process simultaneously
    #[arg(short = 'j', long = "jobs", value_name = "JOBS")]
    num_threads: Option<usize>,

    /// Parse and dump specified config, do nothing else
    #[arg(long = "dump-config", value_name = "CONFIG_PATH")]
    config_to_dump: Option<PathBuf>,

    /// Directories to operate on
    #[arg(value_name = "TARGET_DIR")]
    roots: Vec<PathBuf>,
}

fn get_default_config_path() -> Option<PathBuf> {
    match directories::ProjectDirs::from("", "", "omnilinter") {
        Some(directories) => {
            let path = directories.config_dir().join(CONFIG_FILE_NAME);
            if path.exists() {
                return Some(path);
            }
        }
        None => {
            eprintln!(
                "Warning: cannot set up project directories, default config will not be available"
            );
        }
    }
    None
}

fn read_config(args: &Args) -> Result<Config, Error> {
    if !args.config_paths.is_empty() {
        let mut config = Config::new();
        for path in &args.config_paths {
            config.merge_from(Config::from_file(path)?);
        }
        Ok(config)
    } else if let Some(path) = get_default_config_path() {
        Ok(Config::from_file(&path)?)
    } else {
        bail!("config file is neither specified on the command line, nor present in the application config directory");
    }
}

fn main() -> Result<ExitCode, Error> {
    let args = Args::parse();

    if let Some(config_to_dump) = args.config_to_dump {
        Config::from_file(&config_to_dump).unwrap().dump();
        return Ok(ExitCode::SUCCESS);
    }

    match args.color_mode {
        ColorMode::Always => colored::control::set_override(true),
        ColorMode::Never => colored::control::set_override(false),
        _ => {}
    }

    let mut config = read_config(&args)?;

    if config.ruleset.rules.is_empty() {
        bail!("ruleset is empty");
    }

    let roots: Vec<_> = if args.roots.is_empty() {
        config.roots
    } else {
        args.roots
    }
    .into_iter()
    .filter(|path| {
        if path.is_dir() {
            true
        } else {
            eprintln!("Skipping non-directory root {}", path.display());
            false
        }
    })
    .collect();

    config.ruleset.filter_by_tags(
        &HashSet::from_iter(args.required_tags.iter().map(|tag| tag.to_lowercase())),
        &HashSet::from_iter(args.ignored_tags.iter().map(|tag| tag.to_lowercase())),
    );

    let ruleset = config.ruleset.compile();

    let result = {
        #[cfg(feature = "multithreading")]
        {
            use rayon::prelude::*;
            use std::sync::{Arc, Mutex};

            if let Some(num_threads) = args.num_threads {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(num_threads)
                    .build_global()
                    .unwrap();
            }

            let shared_result = Arc::new(Mutex::new(MatchResult::new()));

            roots.par_iter().for_each(|root| {
                let partial_result = apply_ruleset(&ruleset, root);
                shared_result.lock().unwrap().append(partial_result);
            });

            Arc::into_inner(shared_result)
                .unwrap()
                .into_inner()
                .unwrap()
        }
        #[cfg(not(feature = "multithreading"))]
        {
            let mut result = MatchResult::new();
            roots
                .iter()
                .for_each(|root| result.append(apply_ruleset(&ruleset, root)));

            result
        }
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
            return Ok(error_exitcode.into());
        }
    }
    Ok(ExitCode::SUCCESS)
}
