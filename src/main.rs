//! Detent CLI - BPMN execution engine with MDX round-trip

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use detent::features::convert_bpmn_to_mdx;

#[derive(Parser)]
#[command(name = "detent")]
#[command(about = "BPMN execution engine with MDX round-trip")]
#[command(
    long_about = "A stepper workflow engine that compiles MDX files to BPMN XML and back.\n\
    \n\
    Workflows are authored as a folder of MDX files (one per flow element),\n\
    compiled deterministically to BPMN XML, and can be imported back to MDX.\n\
    Both formats run through a 3-stage validation pipeline:\n\
    parse → schema → structural."
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate BPMN or MDX files
    #[command(
        long_about = "Validate one or more BPMN (.bpmn, .bpmn2) or MDX (.mdx) files.\n\
        \n\
        Each file goes through a staged validation pipeline:\n\
        1. Parse — XML for BPMN, YAML frontmatter for MDX\n\
        2. XSD schema validation — BPMN only (requires xsd-validation feature)\n\
        3. Structural validation — element IDs, required fields\n\
        \n\
        All files must pass for the command to succeed."
    )]
    Validate {
        /// One or more paths to .bpmn, .bpmn2, or .mdx files
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Import a BPMN file and generate MDX files
    #[command(
        long_about = "Import a BPMN XML file into individual MDX files.\n\
        \n\
        Each BPMN flow element (start event, end event, task, gateway,\n\
        sequence flow) becomes one .mdx file with its frontmatter extracted.\n\
        \n\
        If an MDX file already exists at the target path, the frontmatter is\n\
        updated while the Markdown body is preserved (merge behavior)."
    )]
    Import {
        /// Path to the BPMN XML file to import
        #[arg(required = true)]
        bpmn_file: PathBuf,

        /// Output directory for the generated MDX files (default: current directory)
        #[arg(short, long, default_value = ".")]
        output_directory: PathBuf,
    },
    /// Compile MDX files into BPMN XML
    #[command(
        long_about = "Compile a directory of MDX files into a BPMN XML document.\n\
        \n\
        Reads all .mdx files from the given directory, parses their YAML\n\
        frontmatter by type field (e.g. bpmn:startEvent, bpmn:task,\n\
        bpmn:sequenceFlow), and assembles a BPMN 2.0 Definitions document.\n\
        \n\
        Emits BPMN XML after structural frontmatter parsing and assembly."
    )]
    Compile {
        /// Directory containing .mdx files to compile
        #[arg(required = true)]
        directory: PathBuf,

        /// Output BPMN XML file (default: prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { files } => convert_bpmn_to_mdx::infrastructure::cli::validate::run(files),
        Commands::Import {
            bpmn_file,
            output_directory,
        } => convert_bpmn_to_mdx::infrastructure::cli::import::run(bpmn_file, output_directory),
        Commands::Compile { directory, output } => convert_bpmn_to_mdx::infrastructure::cli::compile::run(directory, output),
    }
}
