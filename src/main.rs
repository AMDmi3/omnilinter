mod applier;
mod location;
mod parser;
mod reporter;
mod ruleset;

use crate::applier::apply_ruleset_to_root;
use crate::parser::Config;
use crate::reporter::{Reporter, ReporterOptions};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short = 'c', long, value_name = "CONFIG_PATH")]
    config: PathBuf,

    /// Report full paths of matched files
    #[arg(short = 'f', long)]
    full_paths: bool,

    /// Paths to directories to operate on
    #[arg(value_name = "TARGET_DIR")]
    roots: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut config = Config::new();

    config.append_from_file(&args.config);

    let mut reporter = Reporter::new(ReporterOptions {
        full_paths: args.full_paths,
    });

    let mut roots = if args.roots.is_empty() {
        config.roots
    } else {
        args.roots
    };

    for root in roots {
        apply_ruleset_to_root(&config.ruleset, &root, &mut reporter);
    }
}
