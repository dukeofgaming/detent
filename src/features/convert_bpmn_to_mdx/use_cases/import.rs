//! Import: BPMN → MDX conversion
//!
//! Converts a parsed BPMN Definitions (the IR) into MDX file contents.
//! This is pure logic with no filesystem interaction.

use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{
    BPMNEdge, BPMNShape, Definitions,
};

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

#[derive(Serialize)]
struct DiagramEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<BoundsEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<DiagramLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    waypoints: Option<Vec<WaypointEntry>>,
}

#[derive(Serialize)]
struct BoundsEntry {
    x: String,
    y: String,
    width: String,
    height: String,
}

#[derive(Serialize)]
struct DiagramLabel {
    bounds: BoundsEntry,
}

#[derive(Serialize)]
struct WaypointEntry {
    x: String,
    y: String,
}

fn shape_diagram(shape: &BPMNShape) -> DiagramEntry {
    DiagramEntry {
        bounds: Some(BoundsEntry {
            x: shape.bounds.x.clone(),
            y: shape.bounds.y.clone(),
            width: shape.bounds.width.clone(),
            height: shape.bounds.height.clone(),
        }),
        label: shape.label.as_ref().map(|l| DiagramLabel {
            bounds: BoundsEntry {
                x: l.bounds.x.clone(),
                y: l.bounds.y.clone(),
                width: l.bounds.width.clone(),
                height: l.bounds.height.clone(),
            },
        }),
        waypoints: None,
    }
}

fn edge_diagram(edge: &BPMNEdge) -> DiagramEntry {
    DiagramEntry {
        bounds: None,
        label: edge.label.as_ref().map(|l| DiagramLabel {
            bounds: BoundsEntry {
                x: l.bounds.x.clone(),
                y: l.bounds.y.clone(),
                width: l.bounds.width.clone(),
                height: l.bounds.height.clone(),
            },
        }),
        waypoints: Some(
            edge.waypoints
                .iter()
                .map(|w| WaypointEntry {
                    x: w.x.clone(),
                    y: w.y.clone(),
                })
                .collect(),
        ),
    }
}

pub fn import_to_mdx(definitions: &Definitions) -> Result<Vec<MdxOutput>, ImportError> {
    let process = definitions.process.as_ref().ok_or(ImportError::NoProcess)?;

    let mut diagram: HashMap<&str, DiagramEntry> = HashMap::new();
    if let Some(bpmn_diagram) = &definitions.bpmn_diagram {
        for shape in &bpmn_diagram.plane.shapes {
            if !shape.bpmn_element.is_empty() {
                diagram.insert(shape.bpmn_element.as_str(), shape_diagram(shape));
            }
        }
        for edge in &bpmn_diagram.plane.edges {
            if !edge.bpmn_element.is_empty() {
                diagram.insert(edge.bpmn_element.as_str(), edge_diagram(edge));
            }
        }
    }

    let mut outputs = Vec::new();

    for event in &process.start_events {
        outputs.push(to_diagram_mdx(&event.id, "bpmn:startEvent", event, diagram.get(event.id.as_str()))?);
    }
    for event in &process.end_events {
        outputs.push(to_diagram_mdx(&event.id, "bpmn:endEvent", event, diagram.get(event.id.as_str()))?);
    }
    for task in &process.tasks {
        outputs.push(to_diagram_mdx(&task.id, "bpmn:task", task, diagram.get(task.id.as_str()))?);
    }
    for task in &process.manual_tasks {
        outputs.push(to_diagram_mdx(&task.id, "bpmn:manualTask", task, diagram.get(task.id.as_str()))?);
    }
    for task in &process.user_tasks {
        outputs.push(to_diagram_mdx(&task.id, "bpmn:userTask", task, diagram.get(task.id.as_str()))?);
    }
    for task in &process.service_tasks {
        outputs.push(to_diagram_mdx(&task.id, "bpmn:serviceTask", task, diagram.get(task.id.as_str()))?);
    }
    for task in &process.script_tasks {
        outputs.push(to_diagram_mdx(&task.id, "bpmn:scriptTask", task, diagram.get(task.id.as_str()))?);
    }
    for gateway in &process.exclusive_gateways {
        outputs.push(to_diagram_mdx(&gateway.id, "bpmn:exclusiveGateway", gateway, diagram.get(gateway.id.as_str()))?);
    }
    for gateway in &process.parallel_gateways {
        outputs.push(to_diagram_mdx(&gateway.id, "bpmn:parallelGateway", gateway, diagram.get(gateway.id.as_str()))?);
    }
    for flow in &process.sequence_flows {
        outputs.push(to_diagram_mdx(&flow.id, "bpmn:sequenceFlow", flow, diagram.get(flow.id.as_str()))?);
    }

    Ok(outputs)
}

fn to_diagram_mdx<T: Serialize>(
    id: &str,
    bpmn_type: &str,
    data: &T,
    diagram: Option<&DiagramEntry>,
) -> Result<MdxOutput, ImportError> {
    let yaml =
        serde_yaml::to_string(data).map_err(|e| ImportError::SerializationError(e.to_string()))?;
    let clean_yaml = clean_yaml_for_mdx(&yaml);

    let content = if let Some(d) = diagram {
        let diag_yaml = serde_yaml::to_string(d)
            .map_err(|e| ImportError::SerializationError(e.to_string()))?;
        let clean_diag = clean_yaml_for_mdx(&diag_yaml);
        format!("---\ntype: {}\n{}diagram:\n{}---\n", bpmn_type, clean_yaml, indent_lines(&clean_diag))
    } else {
        format!("---\ntype: {}\n{}---\n", bpmn_type, clean_yaml)
    };

    Ok(MdxOutput {
        filename: format!("{}.mdx", id),
        content,
    })
}

fn indent_lines(s: &str) -> String {
    s.lines()
        .map(|line| format!("  {}", line))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn clean_yaml_for_mdx(yaml: &str) -> String {
    static RE_SINGLE_AT: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"'@([a-zA-Z_][a-zA-Z0-9_]*)':").unwrap());
    static RE_DOUBLE_AT: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#""@([a-zA-Z_][a-zA-Z0-9_]*)":"#).unwrap());
    static RE_SINGLE_DOLLAR: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"'\$([a-zA-Z_][a-zA-Z0-9_]*)':").unwrap());
    static RE_DOUBLE_DOLLAR: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#""\$([a-zA-Z_][a-zA-Z0-9_]*)":"#).unwrap());
    static RE_UNQUOTED_DOLLAR: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(\s)\$([a-zA-Z_][a-zA-Z0-9_]*):").unwrap());

    let result = RE_SINGLE_AT.replace_all(yaml, "$1:");
    let result = RE_DOUBLE_AT.replace_all(&result, "$1:");
    let result = RE_SINGLE_DOLLAR.replace_all(&result, "$1:");
    let result = RE_DOUBLE_DOLLAR.replace_all(&result, "$1:");
    let result = RE_UNQUOTED_DOLLAR.replace_all(&result, "$1$2:");

    result.to_string()
}
