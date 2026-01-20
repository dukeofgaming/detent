//! BPMN Process type

use serde::{Deserialize, Serialize};

use super::{
    Documentation, EndEvent, ExclusiveGateway, FlowElements, ParallelGateway, ScriptTask,
    SequenceFlow, ServiceTask, StartEvent, Task,
};

/// BPMN Process - container for flow elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "process")]
pub struct Process {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(rename = "@name", alias = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@isExecutable", alias = "isExecutable", skip_serializing_if = "Option::is_none")]
    pub is_executable: Option<bool>,

    #[serde(rename = "@processType", alias = "processType", skip_serializing_if = "Option::is_none")]
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
