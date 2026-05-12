//! Import command implementation - BPMN to MDX transpilation
//!
//! Reads a BPMN XML file, converts it to MDX files via transpiler::import,
//! and writes them to the output directory. This is a thin CLI wrapper
//! around transpiler::import.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::transpiler::bpmn::parse_bpmn;
use crate::transpiler::import::import_to_mdx;

/// Run the import command
pub fn run(bpmn_file: PathBuf, output_directory: PathBuf) -> ExitCode {
    // Read and parse BPMN file
    let xml = match fs::read_to_string(&bpmn_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {}: {}", bpmn_file.display(), e);
            return ExitCode::FAILURE;
        }
    };

    let definitions = match parse_bpmn(&xml) {
        Ok(defs) => defs,
        Err(e) => {
            eprintln!("Failed to parse BPMN: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all(&output_directory) {
        eprintln!(
            "Failed to create output directory {}: {}",
            output_directory.display(),
            e
        );
        return ExitCode::FAILURE;
    }

    // Delegate to transpiler::import (pure logic)
    let outputs = match import_to_mdx(&definitions) {
        Ok(outputs) => outputs,
        Err(e) => {
            eprintln!("Import failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Write MDX files to disk
    for output in &outputs {
        let file_path = output_directory.join(&output.filename);
        if let Err(e) = fs::write(&file_path, &output.content) {
            eprintln!("Failed to write {}: {}", file_path.display(), e);
            return ExitCode::FAILURE;
        }
    }

    println!(
        "Generated {} MDX files in {}",
        outputs.len(),
        output_directory.display()
    );
    ExitCode::SUCCESS
}
