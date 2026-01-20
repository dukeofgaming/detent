//! Validate command implementation

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use detent::bpmn::parse_bpmn;
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

    let defs = parse_bpmn(&content)
        .map_err(|e| format!("Invalid BPMN: {}", e))?;

    // Basic validation checks
    if defs.id.is_empty() {
        return Err("BPMN definitions must have an id".to_string());
    }

    if let Some(process) = &defs.process {
        validate_process(process)?;
    }

    Ok(())
}

/// Validate a BPMN process
fn validate_process(process: &detent::bpmn::Process) -> Result<(), String> {
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

    // Try to parse as node frontmatter first
    if let Ok(node) = mdx.parse_node_frontmatter() {
        validate_node_frontmatter(&node)?;
        return Ok(());
    }

    // Try to parse as flow frontmatter
    if let Ok(flow) = mdx.parse_flow_frontmatter() {
        validate_flow_frontmatter(&flow)?;
        return Ok(());
    }

    // Try to parse as process frontmatter
    if let Ok(process) = mdx.parse_process_frontmatter() {
        validate_process_frontmatter(&process)?;
        return Ok(());
    }

    Err("Could not parse frontmatter as node, flow, or process".to_string())
}

/// Validate node frontmatter
fn validate_node_frontmatter(node: &detent::mdx::NodeFrontmatter) -> Result<(), String> {
    if node.id.is_empty() {
        return Err("Node must have an id".to_string());
    }

    if node.node_type.is_empty() {
        return Err("Node must have a type".to_string());
    }

    // Validate type prefix
    if !node.node_type.starts_with("bpmn:") {
        return Err(format!("Invalid node type: {} (must start with 'bpmn:')", node.node_type));
    }

    Ok(())
}

/// Validate flow frontmatter
fn validate_flow_frontmatter(flow: &detent::mdx::FlowFrontmatter) -> Result<(), String> {
    if flow.id.is_empty() {
        return Err("Flow must have an id".to_string());
    }

    if flow.source_ref.is_empty() {
        return Err("Flow must have a sourceRef".to_string());
    }

    if flow.target_ref.is_empty() {
        return Err("Flow must have a targetRef".to_string());
    }

    Ok(())
}

/// Validate process frontmatter
fn validate_process_frontmatter(process: &detent::mdx::ProcessFrontmatter) -> Result<(), String> {
    if process.id.is_empty() {
        return Err("Process must have an id".to_string());
    }

    Ok(())
}
