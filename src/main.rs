mod applier;
mod parser;
mod ruleset;

use crate::applier::apply_ruleset_to_target;
use crate::parser::parse_config_from_file;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short = 'c', long, value_name = "CONFIG_PATH")]
    config: PathBuf,

    /// Paths to directories to operate on
    #[arg(value_name = "TARGET_DIR")]
    targets: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let config = parse_config_from_file(&args.config);

    for target in args.targets {
        apply_ruleset_to_target(&config, &target);
    }
}
