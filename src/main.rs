mod applier;
mod location;
mod parser;
mod reporter;
mod ruleset;

use crate::applier::apply_ruleset_to_root;
use crate::parser::Config;
use crate::reporter::json::JsonReporter;
use crate::reporter::stdout::{ReporterOptions, StdoutReporter};
use crate::reporter::Reporter;
use clap::Parser;
use std::path::PathBuf;
use xdg;

const CONFIG_FILE_NAME: &str = "omnilinter.conf";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path(s) to configuration file(s)
    #[arg(short = 'c', long = "config", value_name = "CONFIG_PATH")]
    config_paths: Vec<PathBuf>,

    /// Report full paths of matched files
    #[arg(short = 'f', long = "full_paths")]
    report_full_paths: bool,

    /// Output matches in JSON format
    #[arg(long = "json")]
    json_output: bool,

    /// Paths to directories to operate on
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
        args.config_paths
            .iter()
            .for_each(|path| config.append_from_file(&path));
    } else if let Some(path) = default_config_path {
        config.append_from_file(&path);
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

    for root in roots {
        apply_ruleset_to_root(&config.ruleset, &root, reporter.as_mut());
    }

    reporter.flush();
}
