//! Compile: MDX → BPMN conversion
//!
//! Converts a collection of MDX file contents into a BPMN Definitions (the IR).
//! This is pure logic with no filesystem interaction and no graph-semantic checks.

use serde::Deserialize;

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{
    BPMNDiagram, BPMNEdge, BPMNLabel, BPMNPlane, BPMNShape, Bounds, Definitions, Process, Waypoint,
};
use crate::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;

#[derive(Debug, Deserialize)]
struct DiagramBlock {
    #[serde(default)]
    bounds: Option<BoundsData>,
    #[serde(default)]
    label: Option<LabelData>,
    #[serde(default)]
    waypoints: Option<Vec<WaypointData>>,
}

#[derive(Debug, Deserialize)]
struct BoundsData {
    x: String,
    y: String,
    width: String,
    height: String,
}

#[derive(Debug, Deserialize)]
struct LabelData {
    bounds: BoundsData,
}

#[derive(Debug, Deserialize)]
struct WaypointData {
    x: String,
    y: String,
}

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

fn parse_diagram(frontmatter: &str) -> Option<DiagramBlock> {
    let value: serde_yaml::Value = serde_yaml::from_str(frontmatter).ok()?;
    let diagram = value.get("diagram")?;
    serde_yaml::from_value(diagram.clone()).ok()
}

fn to_shape(element_id: &str, d: &DiagramBlock) -> BPMNShape {
    let bounds = d.bounds.as_ref().unwrap();
    BPMNShape {
        id: format!("{}_di", element_id),
        bpmn_element: element_id.to_string(),
        bounds: Bounds {
            x: bounds.x.clone(),
            y: bounds.y.clone(),
            width: bounds.width.clone(),
            height: bounds.height.clone(),
        },
        label: d.label.as_ref().map(|l| BPMNLabel {
            bounds: Bounds {
                x: l.bounds.x.clone(),
                y: l.bounds.y.clone(),
                width: l.bounds.width.clone(),
                height: l.bounds.height.clone(),
            },
        }),
    }
}

fn to_edge(element_id: &str, d: &DiagramBlock) -> BPMNEdge {
    BPMNEdge {
        id: format!("{}_di", element_id),
        bpmn_element: element_id.to_string(),
        waypoints: d
            .waypoints
            .as_ref()
            .map(|wps| {
                wps.iter()
                    .map(|w| Waypoint {
                        x: w.x.clone(),
                        y: w.y.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        label: d.label.as_ref().map(|l| BPMNLabel {
            bounds: Bounds {
                x: l.bounds.x.clone(),
                y: l.bounds.y.clone(),
                width: l.bounds.width.clone(),
                height: l.bounds.height.clone(),
            },
        }),
    }
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
        manual_tasks: vec![],
        user_tasks: vec![],
        exclusive_gateways: vec![],
        parallel_gateways: vec![],
        sequence_flows: vec![],
        ..Default::default()
    };

    let mut diagram_shapes: Vec<BPMNShape> = Vec::new();
    let mut diagram_edges: Vec<BPMNEdge> = Vec::new();

    for input in inputs {
        let mdx = MdxFile::parse(&input.content).map_err(|e| CompileError::ParseError {
            filename: input.filename.clone(),
            message: e.to_string(),
        })?;

        let bpmn_type =
            extract_type(&mdx.frontmatter).ok_or_else(|| CompileError::MissingType {
                filename: input.filename.clone(),
            })?;

        let element_id = match bpmn_type.as_str() {
            "bpmn:startEvent" => {
                let event =
                    mdx.parse_start_event()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                let id = event.id.clone();
                process.start_events.push(event);
                id
            }
            "bpmn:endEvent" => {
                let event =
                    mdx.parse_end_event()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                let id = event.id.clone();
                process.end_events.push(event);
                id
            }
            "bpmn:task" => {
                let task = mdx
                    .parse_task()
                    .map_err(|e| CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    })?;
                let id = task.id.clone();
                process.tasks.push(task);
                id
            }
            "bpmn:manualTask" => {
                let task = mdx
                    .parse_manual_task()
                    .map_err(|e| CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    })?;
                let id = task.id.clone();
                process.manual_tasks.push(task);
                id
            }
            "bpmn:userTask" => {
                let task = mdx
                    .parse_user_task()
                    .map_err(|e| CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    })?;
                let id = task.id.clone();
                process.user_tasks.push(task);
                id
            }
            "bpmn:serviceTask" => {
                let task =
                    mdx.parse_service_task()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                let id = task.id.clone();
                process.service_tasks.push(task);
                id
            }
            "bpmn:scriptTask" => {
                let task =
                    mdx.parse_script_task()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                let id = task.id.clone();
                process.script_tasks.push(task);
                id
            }
            "bpmn:exclusiveGateway" => {
                let gateway = mdx.parse_exclusive_gateway().map_err(|e| {
                    CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    }
                })?;
                let id = gateway.id.clone();
                process.exclusive_gateways.push(gateway);
                id
            }
            "bpmn:parallelGateway" => {
                let gateway = mdx.parse_parallel_gateway().map_err(|e| {
                    CompileError::DeserializationError {
                        filename: input.filename.clone(),
                        message: e.to_string(),
                    }
                })?;
                let id = gateway.id.clone();
                process.parallel_gateways.push(gateway);
                id
            }
            "bpmn:sequenceFlow" => {
                let flow =
                    mdx.parse_sequence_flow()
                        .map_err(|e| CompileError::DeserializationError {
                            filename: input.filename.clone(),
                            message: e.to_string(),
                        })?;
                let id = flow.id.clone();
                process.sequence_flows.push(flow);
                id
            }
            _ => {
                return Err(CompileError::UnknownType {
                    filename: input.filename.clone(),
                    bpmn_type,
                });
            }
        };

        if let Some(diagram) = parse_diagram(&mdx.frontmatter) {
            if diagram.bounds.is_some() {
                diagram_shapes.push(to_shape(&element_id, &diagram));
            } else if diagram.waypoints.is_some() {
                diagram_edges.push(to_edge(&element_id, &diagram));
            }
        }
    }

    let bpmn_diagram = if diagram_shapes.is_empty() && diagram_edges.is_empty() {
        None
    } else {
        Some(BPMNDiagram {
            id: "BPMNDiagram_1".to_string(),
            plane: BPMNPlane {
                id: "BPMNPlane_1".to_string(),
                bpmn_element: process.id.clone(),
                shapes: diagram_shapes,
                edges: diagram_edges,
            },
        })
    };

    let definitions = Definitions {
        id: "definitions_1".to_string(),
        name: None,
        target_namespace: None,
        exporter: Some("detent".to_string()),
        exporter_version: None,
        process: Some(process),
        bpmn_diagram,
    };

    Ok(definitions)
}
