//! Validate command implementation

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[cfg(feature = "xsd-validation")]
use crate::transpiler::bpmn::validate_bpmn_xsd;
use crate::transpiler::bpmn::{parse_bpmn, Validate};
use crate::transpiler::mdx::MdxFile;

/// Run the validate command
pub fn run(files: Vec<PathBuf>) -> ExitCode {
    let mut has_errors = false;

    for path in files {
        match validate_file(&path) {
            Ok(()) => {
                println!("✓ {}", path.display());
            }
            Err(e) => {
                eprintln!("✗ {}: {}", path.display(), e);
                has_errors = true;
            }
        }
    }

    if has_errors {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// Validate a single file based on its extension
fn validate_file(path: &PathBuf) -> Result<(), String> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match extension {
        "bpmn" | "bpmn2" => validate_bpmn(path),
        "mdx" => validate_mdx(path),
        _ => Err(format!("Unknown file type: .{}", extension)),
    }
}

/// Validate a BPMN file
fn validate_bpmn(path: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Step 1: XSD Schema validation (when feature is enabled)
    #[cfg(feature = "xsd-validation")]
    {
        validate_bpmn_xsd(&content).map_err(|e| format!("XSD validation failed: {}", e))?;
    }

    // Step 2: Parse into Rust types
    let defs = parse_bpmn(&content).map_err(|e| format!("Invalid BPMN: {}", e))?;

    // Step 3: Semantic validation checks
    defs.validate_for_bpmn()?;

    Ok(())
}

/// Validate an MDX file
fn validate_mdx(path: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mdx = MdxFile::parse(&content).map_err(|e| format!("Invalid MDX: {}", e))?;

    // Try to parse frontmatter as various BPMN types
    // Start with process (for _process.mdx files)
    if let Ok(process) = mdx.parse_process() {
        return process.validate();
    }

    // Try as start event
    if let Ok(event) = mdx.parse_start_event() {
        return event.validate();
    }

    // Try as end event
    if let Ok(event) = mdx.parse_end_event() {
        return event.validate();
    }

    // Try as task
    if let Ok(task) = mdx.parse_task() {
        return task.validate();
    }

    // Try as sequence flow
    if let Ok(flow) = mdx.parse_sequence_flow() {
        return flow.validate();
    }

    Err("Could not parse frontmatter as a valid BPMN type".to_string())
}
