//! Import: BPMN → MDX conversion
//!
//! Converts a parsed BPMN Definitions (the IR) into MDX file contents.
//! This is pure logic with no filesystem interaction.

use regex::Regex;
use serde::Serialize;

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::Definitions;

/// A single MDX file output from the import process
#[derive(Debug, Clone)]
pub struct MdxOutput {
    /// Filename (e.g., "start_1.mdx")
    pub filename: String,
    /// Full MDX content including frontmatter
    pub content: String,
}

/// Import error types
#[derive(Debug, Clone, PartialEq)]
pub enum ImportError {
    /// No process found in the BPMN definitions
    NoProcess,
    /// YAML serialization failed
    SerializationError(String),
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::NoProcess => write!(f, "No process found in BPMN definitions"),
            ImportError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for ImportError {}

pub fn import_to_mdx(definitions: &Definitions) -> Result<Vec<MdxOutput>, ImportError> {
    let process = definitions.process.as_ref().ok_or(ImportError::NoProcess)?;

    let mut outputs = Vec::new();

    for event in &process.start_events {
        outputs.push(to_mdx_output(&event.id, "bpmn:startEvent", event)?);
    }

    for event in &process.end_events {
        outputs.push(to_mdx_output(&event.id, "bpmn:endEvent", event)?);
    }

    for task in &process.tasks {
        outputs.push(to_mdx_output(&task.id, "bpmn:task", task)?);
    }

    for task in &process.service_tasks {
        outputs.push(to_mdx_output(&task.id, "bpmn:serviceTask", task)?);
    }

    for task in &process.script_tasks {
        outputs.push(to_mdx_output(&task.id, "bpmn:scriptTask", task)?);
    }

    for gateway in &process.exclusive_gateways {
        outputs.push(to_mdx_output(&gateway.id, "bpmn:exclusiveGateway", gateway)?);
    }

    for gateway in &process.parallel_gateways {
        outputs.push(to_mdx_output(&gateway.id, "bpmn:parallelGateway", gateway)?);
    }

    for flow in &process.sequence_flows {
        outputs.push(to_mdx_output(&flow.id, "bpmn:sequenceFlow", flow)?);
    }

    Ok(outputs)
}

fn to_mdx_output<T: Serialize>(id: &str, bpmn_type: &str, data: &T) -> Result<MdxOutput, ImportError> {
    let yaml =
        serde_yaml::to_string(data).map_err(|e| ImportError::SerializationError(e.to_string()))?;

    let clean_yaml = clean_yaml_for_mdx(&yaml);
    let content = format!("---\ntype: {}\n{}---\n", bpmn_type, clean_yaml);

    Ok(MdxOutput {
        filename: format!("{}.mdx", id),
        content,
    })
}

fn clean_yaml_for_mdx(yaml: &str) -> String {
    let re_single_at = Regex::new(r"'@([a-zA-Z_][a-zA-Z0-9_]*)':").unwrap();
    let re_double_at = Regex::new(r#""@([a-zA-Z_][a-zA-Z0-9_]*)":"#).unwrap();

    let re_single_dollar = Regex::new(r"'\$([a-zA-Z_][a-zA-Z0-9_]*)':").unwrap();
    let re_double_dollar = Regex::new(r#""\$([a-zA-Z_][a-zA-Z0-9_]*)":"#).unwrap();
    let re_unquoted_dollar = Regex::new(r"(\s)\$([a-zA-Z_][a-zA-Z0-9_]*):").unwrap();

    let result = re_single_at.replace_all(yaml, "$1:");
    let result = re_double_at.replace_all(&result, "$1:");
    let result = re_single_dollar.replace_all(&result, "$1:");
    let result = re_double_dollar.replace_all(&result, "$1:");
    let result = re_unquoted_dollar.replace_all(&result, "$1$2:");

    result.to_string()
}
