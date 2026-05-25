//! Compile: MDX → BPMN conversion
//!
//! Converts a collection of MDX file contents into a BPMN Definitions (the IR).
//! This is pure logic with no filesystem interaction and no graph-semantic checks.

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{Definitions, Process};
use crate::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;

/// A single MDX file input for compilation
#[derive(Debug, Clone)]
pub struct MdxInput {
    /// Filename (e.g., "start_1.mdx")
    pub filename: String,
    /// Full MDX content including frontmatter
    pub content: String,
}

/// Compile error types
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// No MDX inputs provided
    EmptyInputs,
    /// Failed to parse MDX file
    ParseError { filename: String, message: String },
    /// Missing `type` field in frontmatter
    MissingType { filename: String },
    /// Unknown BPMN type in frontmatter
    UnknownType { filename: String, bpmn_type: String },
    /// Failed to deserialize frontmatter into BPMN type
    DeserializationError { filename: String, message: String },
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::EmptyInputs => write!(f, "No MDX inputs provided"),
            CompileError::ParseError { filename, message } => {
                write!(f, "Failed to parse {}: {}", filename, message)
            }
            CompileError::MissingType { filename } => {
                write!(f, "Missing 'type' field in frontmatter of {}", filename)
            }
            CompileError::UnknownType {
                filename,
                bpmn_type,
            } => {
                write!(f, "Unknown BPMN type '{}' in {}", bpmn_type, filename)
            }
            CompileError::DeserializationError { filename, message } => {
                write!(
                    f,
                    "Failed to deserialize {} frontmatter: {}",
                    filename, message
                )
            }
        }
    }
}

impl std::error::Error for CompileError {}

fn extract_type(frontmatter: &str) -> Option<String> {
    for line in frontmatter.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("type:") {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

pub fn compile_to_definitions(inputs: &[MdxInput]) -> Result<Definitions, CompileError> {
    if inputs.is_empty() {
        return Err(CompileError::EmptyInputs);
    }

    let mut process = Process {
        id: "process_1".to_string(),
        name: None,
        is_executable: Some(true),
        process_type: None,
        documentation: None,
        start_events: vec![],
        end_events: vec![],
        tasks: vec![],
        service_tasks: vec![],
        script_tasks: vec![],
        exclusive_gateways: vec![],
        parallel_gateways: vec![],
        sequence_flows: vec![],
    };

    for input in inputs {
        let mdx = MdxFile::parse(&input.content).map_err(|e| CompileError::ParseError {
            filename: input.filename.clone(),
            message: e.to_string(),
        })?;

        let bpmn_type =
            extract_type(&mdx.frontmatter).ok_or_else(|| CompileError::MissingType {
                filename: input.filename.clone(),
            })?;

        match bpmn_type.as_str() {
            "bpmn:startEvent" => {
                let event =
                    mdx.parse_start_event()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                process.start_events.push(event);
            }
            "bpmn:endEvent" => {
                let event =
                    mdx.parse_end_event()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                process.end_events.push(event);
            }
            "bpmn:task" => {
                let task = mdx
                    .parse_task()
                    .map_err(|e| CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    })?;
                process.tasks.push(task);
            }
            "bpmn:serviceTask" => {
                let task =
                    mdx.parse_service_task()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                process.service_tasks.push(task);
            }
            "bpmn:scriptTask" => {
                let task =
                    mdx.parse_script_task()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                process.script_tasks.push(task);
            }
            "bpmn:exclusiveGateway" => {
                let gateway = mdx.parse_exclusive_gateway().map_err(|e| {
                    CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    }
                })?;
                process.exclusive_gateways.push(gateway);
            }
            "bpmn:parallelGateway" => {
                let gateway = mdx.parse_parallel_gateway().map_err(|e| {
                    CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    }
                })?;
                process.parallel_gateways.push(gateway);
            }
            "bpmn:sequenceFlow" => {
                let flow =
                    mdx.parse_sequence_flow()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                process.sequence_flows.push(flow);
            }
            _ => {
                return Err(CompileError::UnknownType {
                    filename: input.filename.clone(),
                    bpmn_type,
                });
            }
        }
    }

    let definitions = Definitions {
        id: "definitions_1".to_string(),
        name: None,
        target_namespace: None,
        exporter: Some("detent".to_string()),
        exporter_version: None,
        process: Some(process),
        bpmn_diagram: None,
    };

    Ok(definitions)
}
