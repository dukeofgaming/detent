//! Compile command implementation - MDX to BPMN conversion
//!
//! Reads a directory of MDX files, compiles them into a BPMN Definitions (IR),
//! and serializes the result to BPMN XML. This is a thin CLI wrapper around
//! compiler::compile.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use detent::bpmn::serialize_bpmn;
use detent::compiler::compile::{compile_to_definitions, MdxInput};

/// Run the compile command
pub fn run(directory: PathBuf, output: Option<PathBuf>) -> ExitCode {
    // Read all .mdx files from the directory
    let entries = match fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", directory.display(), e);
            return ExitCode::FAILURE;
        }
    };

    let mut inputs: Vec<MdxInput> = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read directory entry: {}", e);
                return ExitCode::FAILURE;
            }
        };

        let path = entry.path();

        // Only process .mdx files
        if path.extension().and_then(|e| e.to_str()) != Some("mdx") {
            continue;
        }

        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read {}: {}", path.display(), e);
                return ExitCode::FAILURE;
            }
        };

        inputs.push(MdxInput { filename, content });
    }

    if inputs.is_empty() {
        eprintln!("No .mdx files found in {}", directory.display());
        return ExitCode::FAILURE;
    }

    // Compile MDX inputs into Definitions (IR)
    let definitions = match compile_to_definitions(&inputs) {
        Ok(defs) => defs,
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Serialize to BPMN XML
    let xml = match serialize_bpmn(&definitions) {
        Ok(xml) => xml,
        Err(e) => {
            eprintln!("Failed to serialize BPMN: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Write output
    match output {
        Some(path) => {
            if let Err(e) = fs::write(&path, &xml) {
                eprintln!("Failed to write {}: {}", path.display(), e);
                return ExitCode::FAILURE;
            }
            println!("Compiled BPMN written to {}", path.display());
        }
        None => {
            print!("{}", xml);
        }
    }

    ExitCode::SUCCESS
}
