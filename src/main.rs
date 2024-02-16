// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod applier;
mod config;
mod location;
mod parser;
mod reporter;
mod ruleset;

use crate::applier::{Applier, ApplierOptions};
use crate::config::Config;
use crate::parser::ParsedConfig;
use crate::reporter::json::JsonReporter;
use crate::reporter::stdout::{ReporterOptions, StdoutReporter};
use crate::reporter::Reporter;
use clap::Parser;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "omnilinter.conf";

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

    /// Report full paths of matched files
    #[arg(short = 'f', long = "full-paths")]
    report_full_paths: bool,

    /// Output matches in JSON format
    #[arg(long = "json")]
    json_output: bool,

    /// If any matches are found, exit with given code
    #[arg(long, value_name = "EXITCODE")]
    error_exitcode: Option<i32>,

    /// Directories to operate on
    #[arg(value_name = "TARGET_DIR")]
    roots: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

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
            ParsedConfig::from_yaml_file(&path)
                .unwrap()
                .append_into_config(&mut config)
        });
    } else if let Some(path) = default_config_path {
        ParsedConfig::from_yaml_file(&path)
            .unwrap()
            .append_into_config(&mut config);
    } else {
        eprintln!("Error: config file is neither specified on the command line, nor present in the application config directory");
    }

    if config.ruleset.rules.is_empty() {
        eprintln!("Warning: ruleset is empty");
    }

    let mut reporter: Box<dyn Reporter> = if args.json_output {
        Box::new(JsonReporter::new())
    } else {
        Box::new(StdoutReporter::new(ReporterOptions {
            full_paths: args.report_full_paths,
        }))
    };

    let roots = if args.roots.is_empty() {
        config.roots
    } else {
        args.roots
    };

    let mut applier = Applier::new(
        &config.ruleset,
        reporter.as_mut(),
        ApplierOptions {
            required_tags: args.required_tags.into_iter().collect(),
            ignored_tags: args.ignored_tags.into_iter().collect(),
        },
    );

    for root in roots {
        applier.apply_to_root(&root);
    }

    reporter.flush();

    if let Some(error_exitcode) = args.error_exitcode {
        if reporter.has_matches() {
            std::process::exit(error_exitcode);
        }
    }
}
