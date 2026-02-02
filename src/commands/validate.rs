//! Validate command implementation

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use detent::bpmn::{parse_bpmn, StartEvent, EndEvent, Task, SequenceFlow, Process};
#[cfg(feature = "xsd-validation")]
use detent::bpmn::validate_bpmn_xsd;
use detent::mdx::MdxFile;

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
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match extension {
        "bpmn" | "bpmn2" => validate_bpmn(path),
        "mdx" => validate_mdx(path),
        _ => Err(format!("Unknown file type: .{}", extension)),
    }
}

/// Validate a BPMN file
fn validate_bpmn(path: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Step 1: XSD Schema validation (when feature is enabled)
    #[cfg(feature = "xsd-validation")]
    {
        validate_bpmn_xsd(&content)
            .map_err(|e| format!("XSD validation failed: {}", e))?;
    }

    // Step 2: Parse into Rust types
    let defs = parse_bpmn(&content)
        .map_err(|e| format!("Invalid BPMN: {}", e))?;

    // Step 3: Semantic validation checks
    if defs.id.is_empty() {
        return Err("BPMN definitions must have an id".to_string());
    }

    if let Some(process) = &defs.process {
        validate_process(process)?;
    }

    Ok(())
}

/// Validate a BPMN process
fn validate_process(process: &Process) -> Result<(), String> {
    if process.id.is_empty() {
        return Err("Process must have an id".to_string());
    }

    // Check for at least one start event
    if process.start_events.is_empty() {
        return Err("Process must have at least one start event".to_string());
    }

    // Check for at least one end event
    if process.end_events.is_empty() {
        return Err("Process must have at least one end event".to_string());
    }

    Ok(())
}

/// Validate an MDX file
fn validate_mdx(path: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mdx = MdxFile::parse(&content)
        .map_err(|e| format!("Invalid MDX: {}", e))?;

    // Try to parse frontmatter as various BPMN types
    // Start with process (for _process.mdx files)
    if let Ok(process) = mdx.parse_process() {
        return validate_process_frontmatter(&process);
    }

    // Try as start event
    if let Ok(event) = mdx.parse_start_event() {
        return validate_start_event(&event);
    }

    // Try as end event
    if let Ok(event) = mdx.parse_end_event() {
        return validate_end_event(&event);
    }

    // Try as task
    if let Ok(task) = mdx.parse_task() {
        return validate_task(&task);
    }

    // Try as sequence flow
    if let Ok(flow) = mdx.parse_sequence_flow() {
        return validate_sequence_flow(&flow);
    }

    Err("Could not parse frontmatter as a valid BPMN type".to_string())
}

/// Validate a start event
fn validate_start_event(event: &StartEvent) -> Result<(), String> {
    if event.id.is_empty() {
        return Err("StartEvent must have an id".to_string());
    }
    Ok(())
}

/// Validate an end event
fn validate_end_event(event: &EndEvent) -> Result<(), String> {
    if event.id.is_empty() {
        return Err("EndEvent must have an id".to_string());
    }
    Ok(())
}

/// Validate a task
fn validate_task(task: &Task) -> Result<(), String> {
    if task.id.is_empty() {
        return Err("Task must have an id".to_string());
    }
    Ok(())
}

/// Validate a sequence flow
fn validate_sequence_flow(flow: &SequenceFlow) -> Result<(), String> {
    if flow.id.is_empty() {
        return Err("SequenceFlow must have an id".to_string());
    }
    if flow.source_ref.is_empty() {
        return Err("SequenceFlow must have a sourceRef".to_string());
    }
    if flow.target_ref.is_empty() {
        return Err("SequenceFlow must have a targetRef".to_string());
    }
    Ok(())
}

/// Validate process frontmatter
fn validate_process_frontmatter(process: &Process) -> Result<(), String> {
    if process.id.is_empty() {
        return Err("Process must have an id".to_string());
    }
    Ok(())
}
