// src/main.rs
/*
 * Main executable for DeFiAiAutomationSolutionsUltra
 */

use clap::Parser;
use defiaiautomationsolutionsultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "DeFiAiAutomationSolutionsUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
