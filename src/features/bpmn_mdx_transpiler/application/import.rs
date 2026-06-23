//! Import: BPMN → MDX conversion
//!
//! Converts a parsed BPMN Definitions (the IR) into MDX file contents.
//! This is pure logic with no filesystem interaction.

use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::features::bpmn_mdx_transpiler::adapters::bpmn::{
    BPMNEdge, BPMNShape, ConditionExpression, Definitions, Documentation, Process,
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
    id: String,
    #[serde(rename = "isMarkerVisible", skip_serializing_if = "Option::is_none")]
    is_marker_visible: Option<bool>,
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

#[derive(Serialize)]
struct ProcessMetadata<'a> {
    id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(rename = "isExecutable", skip_serializing_if = "Option::is_none")]
    is_executable: Option<bool>,
    #[serde(rename = "isClosed", skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(rename = "processType", skip_serializing_if = "Option::is_none")]
    process_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation: Option<&'a Documentation>,
    definitions: DefinitionsMetadata<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagram: Option<ProcessDiagramMetadata<'a>>,
}

#[derive(Serialize)]
struct DefinitionsMetadata<'a> {
    id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(rename = "targetNamespace", skip_serializing_if = "Option::is_none")]
    target_namespace: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exporter: Option<&'a str>,
    #[serde(rename = "exporterVersion", skip_serializing_if = "Option::is_none")]
    exporter_version: Option<&'a str>,
}

#[derive(Serialize)]
struct ProcessDiagramMetadata<'a> {
    id: &'a str,
    plane: ProcessPlaneMetadata<'a>,
}

#[derive(Serialize)]
struct ProcessPlaneMetadata<'a> {
    id: &'a str,
    #[serde(rename = "bpmnElement")]
    bpmn_element: &'a str,
}

fn shape_diagram(shape: &BPMNShape) -> DiagramEntry {
    DiagramEntry {
        id: shape.id.clone(),
        is_marker_visible: shape.is_marker_visible,
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
        id: edge.id.clone(),
        is_marker_visible: None,
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

    outputs.push(to_process_mdx(definitions, process)?);

    for event in &process.start_events {
        outputs.push(to_diagram_mdx(
            &event.id,
            "bpmn:startEvent",
            event,
            diagram.get(event.id.as_str()),
        )?);
    }
    for event in &process.end_events {
        outputs.push(to_diagram_mdx(
            &event.id,
            "bpmn:endEvent",
            event,
            diagram.get(event.id.as_str()),
        )?);
    }
    for task in &process.tasks {
        outputs.push(to_diagram_mdx(
            &task.id,
            "bpmn:task",
            task,
            diagram.get(task.id.as_str()),
        )?);
    }
    for task in &process.manual_tasks {
        outputs.push(to_diagram_mdx(
            &task.id,
            "bpmn:manualTask",
            task,
            diagram.get(task.id.as_str()),
        )?);
    }
    for task in &process.user_tasks {
        outputs.push(to_diagram_mdx(
            &task.id,
            "bpmn:userTask",
            task,
            diagram.get(task.id.as_str()),
        )?);
    }
    for task in &process.service_tasks {
        outputs.push(to_diagram_mdx(
            &task.id,
            "bpmn:serviceTask",
            task,
            diagram.get(task.id.as_str()),
        )?);
    }
    for task in &process.script_tasks {
        outputs.push(to_diagram_mdx(
            &task.id,
            "bpmn:scriptTask",
            task,
            diagram.get(task.id.as_str()),
        )?);
    }
    for gateway in &process.exclusive_gateways {
        outputs.push(to_diagram_mdx(
            &gateway.id,
            "bpmn:exclusiveGateway",
            gateway,
            diagram.get(gateway.id.as_str()),
        )?);
    }
    for gateway in &process.parallel_gateways {
        outputs.push(to_diagram_mdx(
            &gateway.id,
            "bpmn:parallelGateway",
            gateway,
            diagram.get(gateway.id.as_str()),
        )?);
    }
    for flow in &process.sequence_flows {
        let cleaned_flow = if let Some(ref ce) = flow.condition_expression {
            if ce.xsi_type.is_some() {
                flow.clone()
            } else {
                let mut f = flow.clone();
                f.condition_expression = Some(ConditionExpression {
                    xsi_type: Some("bpmn:tFormalExpression".to_string()),
                    expression: ce.expression.clone(),
                });
                f
            }
        } else {
            flow.clone()
        };
        outputs.push(to_diagram_mdx(
            &cleaned_flow.id,
            "bpmn:sequenceFlow",
            &cleaned_flow,
            diagram.get(cleaned_flow.id.as_str()),
        )?);
    }

    Ok(outputs)
}

fn to_process_mdx(definitions: &Definitions, process: &Process) -> Result<MdxOutput, ImportError> {
    let diagram = definitions.bpmn_diagram.as_ref().map(|d| ProcessDiagramMetadata {
        id: d.id.as_str(),
        plane: ProcessPlaneMetadata {
            id: d.plane.id.as_str(),
            bpmn_element: d.plane.bpmn_element.as_str(),
        },
    });
    let metadata = ProcessMetadata {
        id: process.id.as_str(),
        name: process.name.as_deref(),
        is_executable: process.is_executable,
        is_closed: process.is_closed,
        process_type: process.process_type.as_deref(),
        documentation: process.documentation.as_ref(),
        definitions: DefinitionsMetadata {
            id: definitions.id.as_str(),
            name: definitions.name.as_deref(),
            target_namespace: definitions.target_namespace.as_deref(),
            exporter: definitions.exporter.as_deref(),
            exporter_version: definitions.exporter_version.as_deref(),
        },
        diagram,
    };
    let yaml = serde_yaml::to_string(&metadata)
        .map_err(|e| ImportError::SerializationError(e.to_string()))?;
    let clean_yaml = clean_yaml_for_mdx(&yaml);

    Ok(MdxOutput {
        filename: format!("{}.mdx", process.id),
        content: format!("---\ntype: bpmn:process\n{}---\n", clean_yaml),
    })
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
        format!(
            "---\ntype: {}\n{}diagram:\n{}---\n",
            bpmn_type,
            clean_yaml,
            indent_lines(&clean_diag)
        )
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
    static RE_SINGLE_AT_QUALIFIED: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"'@([a-zA-Z_][a-zA-Z0-9_]*):([a-zA-Z_][a-zA-Z0-9_]*)':").unwrap()
    });
    static RE_DOUBLE_AT_QUALIFIED: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#""@([a-zA-Z_][a-zA-Z0-9_]*):([a-zA-Z_][a-zA-Z0-9_]*)":"#)
            .unwrap()
    });
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

    let result = RE_SINGLE_AT_QUALIFIED.replace_all(yaml, "$1_$2:");
    let result = RE_DOUBLE_AT_QUALIFIED.replace_all(&result, "$1_$2:");
    let result = RE_SINGLE_AT.replace_all(&result, "$1:");
    let result = RE_DOUBLE_AT.replace_all(&result, "$1:");
    let result = RE_SINGLE_DOLLAR.replace_all(&result, "$1:");
    let result = RE_DOUBLE_DOLLAR.replace_all(&result, "$1:");
    let result = RE_UNQUOTED_DOLLAR.replace_all(&result, "$1$2:");

    result.to_string()
}
