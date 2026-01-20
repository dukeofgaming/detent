//! Detent CLI - BPMN execution engine with MDX round-trip

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

mod commands;

#[derive(Parser)]
#[command(name = "detent")]
#[command(about = "BPMN execution engine with MDX round-trip", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate BPMN or MDX files
    Validate {
        /// Files to validate (BPMN or MDX)
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { files } => commands::validate::run(files),
    }
}
