use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[cfg(feature = "xsd-validation")]
use crate::features::convert_bpmn_to_mdx::infrastructure::xsd_validator::LibxmlSchemaValidator;
use crate::features::convert_bpmn_to_mdx::adapters::bpmn::Validate;
use crate::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;
use crate::features::graph_validation::use_cases::schema_validator::NoopSchemaValidator;
use crate::features::graph_validation::use_cases::validate_workflow;

pub fn run(files: Vec<PathBuf>) -> ExitCode {
    let mut has_errors = false;

    for path in files {
        match validate_file(&path) {
            Ok(()) => println!("✓ {}", path.display()),
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

fn validate_file(path: &PathBuf) -> Result<(), String> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match extension {
        "bpmn" | "bpmn2" => validate_bpmn(path),
        "mdx" => validate_mdx(path),
        _ => Err(format!("Unknown file type: .{}", extension)),
    }
}

fn validate_bpmn(path: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    #[cfg(feature = "xsd-validation")]
    {
        let validator = LibxmlSchemaValidator;
        return validate_bpmn_content(&content, &validator);
    }

    let validator = NoopSchemaValidator;
    validate_bpmn_content(&content, &validator)
}

fn validate_bpmn_content(content: &str, validator: &dyn crate::features::graph_validation::use_cases::schema_validator::SchemaValidator) -> Result<(), String> {
    validate_workflow::validate_bpmn_string(content, validator)
}

fn validate_mdx(path: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let mdx = MdxFile::parse(&content).map_err(|e| format!("Invalid MDX: {}", e))?;

    if let Ok(process) = mdx.parse_process() {
        return process.validate();
    }
    if let Ok(event) = mdx.parse_start_event() {
        return event.validate();
    }
    if let Ok(event) = mdx.parse_end_event() {
        return event.validate();
    }
    if let Ok(task) = mdx.parse_task() {
        return task.validate();
    }
    if let Ok(flow) = mdx.parse_sequence_flow() {
        return flow.validate();
    }

    Err("Could not parse frontmatter as a valid BPMN type".to_string())
}
