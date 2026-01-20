//! BPMN 2.0 types for the subset defined in spec.md
//!
//! These types are handcrafted to match the OMG BPMN 2.0 XSD structure
//! while being tailored for our MDX round-trip use case.
//!
//! Supported elements:
//! - definitions (root)
//! - process
//! - startEvent, endEvent
//! - task, serviceTask, scriptTask
//! - exclusiveGateway, parallelGateway
//! - sequenceFlow
//! - documentation
//! - extensionElements (passthrough)

use serde::{Deserialize, Serialize};

/// BPMN 2.0 namespace
pub const BPMN_NS: &str = "http://www.omg.org/spec/BPMN/20100524/MODEL";
/// BPMN DI namespace
pub const BPMNDI_NS: &str = "http://www.omg.org/spec/BPMN/20100524/DI";

/// Root element of a BPMN document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "definitions")]
pub struct Definitions {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@targetNamespace", skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,

    #[serde(rename = "@exporter", skip_serializing_if = "Option::is_none")]
    pub exporter: Option<String>,

    #[serde(rename = "@exporterVersion", skip_serializing_if = "Option::is_none")]
    pub exporter_version: Option<String>,

    /// Single process (most common case) - we may need to extend for multiple
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,

    // We skip diagram info for now - it's preserved as raw XML
    #[serde(skip)]
    pub bpmn_diagram: Option<String>,
}

/// BPMN Process - container for flow elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "process")]
pub struct Process {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@isExecutable", skip_serializing_if = "Option::is_none")]
    pub is_executable: Option<bool>,

    #[serde(rename = "@processType", skip_serializing_if = "Option::is_none")]
    pub process_type: Option<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,

    #[serde(rename = "startEvent", default)]
    pub start_events: Vec<StartEvent>,

    #[serde(rename = "endEvent", default)]
    pub end_events: Vec<EndEvent>,

    #[serde(rename = "task", default)]
    pub tasks: Vec<Task>,

    #[serde(rename = "serviceTask", default)]
    pub service_tasks: Vec<ServiceTask>,

    #[serde(rename = "scriptTask", default)]
    pub script_tasks: Vec<ScriptTask>,

    #[serde(rename = "exclusiveGateway", default)]
    pub exclusive_gateways: Vec<ExclusiveGateway>,

    #[serde(rename = "parallelGateway", default)]
    pub parallel_gateways: Vec<ParallelGateway>,

    #[serde(rename = "sequenceFlow", default)]
    pub sequence_flows: Vec<SequenceFlow>,
}

impl Process {
    /// Get all flow elements as a FlowElements struct (for convenience)
    pub fn flow_elements(&self) -> FlowElements {
        FlowElements {
            start_events: self.start_events.clone(),
            end_events: self.end_events.clone(),
            tasks: self.tasks.clone(),
            service_tasks: self.service_tasks.clone(),
            script_tasks: self.script_tasks.clone(),
            exclusive_gateways: self.exclusive_gateways.clone(),
            parallel_gateways: self.parallel_gateways.clone(),
            sequence_flows: self.sequence_flows.clone(),
        }
    }
}

/// Container for all flow element types
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FlowElements {
    #[serde(rename = "startEvent", default)]
    pub start_events: Vec<StartEvent>,

    #[serde(rename = "endEvent", default)]
    pub end_events: Vec<EndEvent>,

    #[serde(rename = "task", default)]
    pub tasks: Vec<Task>,

    #[serde(rename = "serviceTask", default)]
    pub service_tasks: Vec<ServiceTask>,

    #[serde(rename = "scriptTask", default)]
    pub script_tasks: Vec<ScriptTask>,

    #[serde(rename = "exclusiveGateway", default)]
    pub exclusive_gateways: Vec<ExclusiveGateway>,

    #[serde(rename = "parallelGateway", default)]
    pub parallel_gateways: Vec<ParallelGateway>,

    #[serde(rename = "sequenceFlow", default)]
    pub sequence_flows: Vec<SequenceFlow>,
}

/// Documentation element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Documentation {
    #[serde(rename = "$text", default)]
    pub text: String,
}

/// BPMN Start Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "startEvent")]
pub struct StartEvent {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN End Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "endEvent")]
pub struct EndEvent {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// Generic BPMN Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "task")]
pub struct Task {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN Service Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "serviceTask")]
pub struct ServiceTask {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@implementation", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN Script Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "scriptTask")]
pub struct ScriptTask {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@scriptFormat", skip_serializing_if = "Option::is_none")]
    pub script_format: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN Exclusive Gateway (XOR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "exclusiveGateway")]
pub struct ExclusiveGateway {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@gatewayDirection", skip_serializing_if = "Option::is_none")]
    pub gateway_direction: Option<String>,

    #[serde(rename = "@default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN Parallel Gateway (AND)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "parallelGateway")]
pub struct ParallelGateway {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@gatewayDirection", skip_serializing_if = "Option::is_none")]
    pub gateway_direction: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN Sequence Flow (edge between nodes)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sequenceFlow")]
pub struct SequenceFlow {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@sourceRef")]
    pub source_ref: String,

    #[serde(rename = "@targetRef")]
    pub target_ref: String,

    #[serde(rename = "conditionExpression", skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// Condition expression for gateways
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionExpression {
    #[serde(rename = "@xsi:type", skip_serializing_if = "Option::is_none")]
    pub xsi_type: Option<String>,

    #[serde(rename = "$text", default)]
    pub expression: String,
}

/// Unified enum for all flow node types (for graph operations)
#[derive(Debug, Clone, PartialEq)]
pub enum FlowNode {
    StartEvent(StartEvent),
    EndEvent(EndEvent),
    Task(Task),
    ServiceTask(ServiceTask),
    ScriptTask(ScriptTask),
    ExclusiveGateway(ExclusiveGateway),
    ParallelGateway(ParallelGateway),
}

impl FlowNode {
    /// Get the ID of this flow node
    pub fn id(&self) -> &str {
        match self {
            FlowNode::StartEvent(e) => &e.id,
            FlowNode::EndEvent(e) => &e.id,
            FlowNode::Task(t) => &t.id,
            FlowNode::ServiceTask(t) => &t.id,
            FlowNode::ScriptTask(t) => &t.id,
            FlowNode::ExclusiveGateway(g) => &g.id,
            FlowNode::ParallelGateway(g) => &g.id,
        }
    }

    /// Get the name of this flow node
    pub fn name(&self) -> Option<&str> {
        match self {
            FlowNode::StartEvent(e) => e.name.as_deref(),
            FlowNode::EndEvent(e) => e.name.as_deref(),
            FlowNode::Task(t) => t.name.as_deref(),
            FlowNode::ServiceTask(t) => t.name.as_deref(),
            FlowNode::ScriptTask(t) => t.name.as_deref(),
            FlowNode::ExclusiveGateway(g) => g.name.as_deref(),
            FlowNode::ParallelGateway(g) => g.name.as_deref(),
        }
    }

    /// Get the BPMN type name (e.g., "bpmn:startEvent")
    pub fn type_name(&self) -> &'static str {
        match self {
            FlowNode::StartEvent(_) => "bpmn:startEvent",
            FlowNode::EndEvent(_) => "bpmn:endEvent",
            FlowNode::Task(_) => "bpmn:task",
            FlowNode::ServiceTask(_) => "bpmn:serviceTask",
            FlowNode::ScriptTask(_) => "bpmn:scriptTask",
            FlowNode::ExclusiveGateway(_) => "bpmn:exclusiveGateway",
            FlowNode::ParallelGateway(_) => "bpmn:parallelGateway",
        }
    }

    /// Get incoming flow references
    pub fn incoming(&self) -> &[String] {
        match self {
            FlowNode::StartEvent(_) => &[],
            FlowNode::EndEvent(e) => &e.incoming,
            FlowNode::Task(t) => &t.incoming,
            FlowNode::ServiceTask(t) => &t.incoming,
            FlowNode::ScriptTask(t) => &t.incoming,
            FlowNode::ExclusiveGateway(g) => &g.incoming,
            FlowNode::ParallelGateway(g) => &g.incoming,
        }
    }

    /// Get outgoing flow references
    pub fn outgoing(&self) -> &[String] {
        match self {
            FlowNode::StartEvent(e) => &e.outgoing,
            FlowNode::EndEvent(_) => &[],
            FlowNode::Task(t) => &t.outgoing,
            FlowNode::ServiceTask(t) => &t.outgoing,
            FlowNode::ScriptTask(t) => &t.outgoing,
            FlowNode::ExclusiveGateway(g) => &g.outgoing,
            FlowNode::ParallelGateway(g) => &g.outgoing,
        }
    }
}

impl FlowElements {
    /// Iterate over all flow nodes
    pub fn nodes(&self) -> impl Iterator<Item = FlowNode> + '_ {
        self.start_events
            .iter()
            .cloned()
            .map(FlowNode::StartEvent)
            .chain(self.end_events.iter().cloned().map(FlowNode::EndEvent))
            .chain(self.tasks.iter().cloned().map(FlowNode::Task))
            .chain(self.service_tasks.iter().cloned().map(FlowNode::ServiceTask))
            .chain(self.script_tasks.iter().cloned().map(FlowNode::ScriptTask))
            .chain(
                self.exclusive_gateways
                    .iter()
                    .cloned()
                    .map(FlowNode::ExclusiveGateway),
            )
            .chain(
                self.parallel_gateways
                    .iter()
                    .cloned()
                    .map(FlowNode::ParallelGateway),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_node_id() {
        let start = FlowNode::StartEvent(StartEvent {
            id: "start_1".to_string(),
            name: Some("Start".to_string()),
            outgoing: vec!["flow_1".to_string()],
            documentation: None,
        });
        assert_eq!(start.id(), "start_1");
        assert_eq!(start.type_name(), "bpmn:startEvent");
    }
}
