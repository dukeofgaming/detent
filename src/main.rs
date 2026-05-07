//! Detent CLI - BPMN execution engine with MDX round-trip

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use detent::compiler::commands;

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
    /// Import a BPMN file and generate MDX files
    Import {
        /// BPMN file to import
        #[arg(required = true)]
        bpmn_file: PathBuf,

        /// Output directory for generated MDX files
        #[arg(short, long, default_value = ".")]
        output_directory: PathBuf,
    },
    /// Compile MDX files into BPMN XML
    Compile {
        /// Directory containing MDX files
        #[arg(required = true)]
        directory: PathBuf,

        /// Output BPMN file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { files } => commands::validate::run(files),
        Commands::Import {
            bpmn_file,
            output_directory,
        } => commands::import::run(bpmn_file, output_directory),
        Commands::Compile { directory, output } => commands::compile::run(directory, output),
    }
}
