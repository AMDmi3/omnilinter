mod parser;
mod ruleset;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short = 'c', long, value_name = "CONFIG_PATH")]
    config: String,

    /// Paths to directories to operate on
    #[arg(value_name = "TARGET_DIR")]
    targets: Vec<String>,
}

fn main() {
    let args = Args::parse();
}
