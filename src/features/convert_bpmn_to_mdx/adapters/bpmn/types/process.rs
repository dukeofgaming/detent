//! BPMN Process type

use serde::de;
use serde::{Deserialize, Serialize};

use super::{
    Documentation, EndEvent, ExclusiveGateway, FlowElements, ManualTask, ParallelGateway,
    ScriptTask, SequenceFlow, ServiceTask, StartEvent, Task, UserTask,
};
use crate::features::convert_bpmn_to_mdx::adapters::bpmn::Validate;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum ProcessElement {
    StartEvent(StartEvent),
    EndEvent(EndEvent),
    Task(Task),
    ManualTask(ManualTask),
    UserTask(UserTask),
    ServiceTask(ServiceTask),
    ScriptTask(ScriptTask),
    ExclusiveGateway(ExclusiveGateway),
    ParallelGateway(ParallelGateway),
    SequenceFlow(SequenceFlow),
}

/// BPMN Process - container for flow elements
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename = "process")]
pub struct Process {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(
        rename = "@name",
        alias = "name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(
        rename = "@isExecutable",
        alias = "isExecutable",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_executable: Option<bool>,

    #[serde(
        rename = "@isClosed",
        alias = "isClosed",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_closed: Option<bool>,

    #[serde(
        rename = "@processType",
        alias = "processType",
        skip_serializing_if = "Option::is_none"
    )]
    pub process_type: Option<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,

    #[serde(rename = "startEvent", default)]
    pub start_events: Vec<StartEvent>,

    #[serde(rename = "endEvent", default)]
    pub end_events: Vec<EndEvent>,

    #[serde(rename = "task", default)]
    pub tasks: Vec<Task>,

    #[serde(rename = "manualTask", default)]
    pub manual_tasks: Vec<ManualTask>,

    #[serde(rename = "userTask", default)]
    pub user_tasks: Vec<UserTask>,

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

impl<'de> Deserialize<'de> for Process {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "process")]
        struct ProcessHelper {
            #[serde(rename = "@id", alias = "id")]
            id: String,
            #[serde(rename = "@name", alias = "name")]
            name: Option<String>,
            #[serde(rename = "@isExecutable", alias = "isExecutable")]
            is_executable: Option<bool>,
            #[serde(rename = "@isClosed", alias = "isClosed")]
            is_closed: Option<bool>,
            #[serde(rename = "@processType", alias = "processType")]
            process_type: Option<String>,
            #[serde(rename = "documentation")]
            documentation: Option<Documentation>,
            #[serde(rename = "$value", default)]
            elements: Vec<ProcessElement>,
        }

        let helper = ProcessHelper::deserialize(deserializer)?;

        let mut process = Process {
            id: helper.id,
            name: helper.name,
            is_executable: helper.is_executable,
            is_closed: helper.is_closed,
            process_type: helper.process_type,
            documentation: helper.documentation,
            ..Default::default()
        };

        for elem in helper.elements {
            match elem {
                ProcessElement::StartEvent(e) => process.start_events.push(e),
                ProcessElement::EndEvent(e) => process.end_events.push(e),
                ProcessElement::Task(t) => process.tasks.push(t),
                ProcessElement::ManualTask(t) => process.manual_tasks.push(t),
                ProcessElement::UserTask(t) => process.user_tasks.push(t),
                ProcessElement::ServiceTask(t) => process.service_tasks.push(t),
                ProcessElement::ScriptTask(t) => process.script_tasks.push(t),
                ProcessElement::ExclusiveGateway(g) => process.exclusive_gateways.push(g),
                ProcessElement::ParallelGateway(g) => process.parallel_gateways.push(g),
                ProcessElement::SequenceFlow(f) => process.sequence_flows.push(f),
            }
        }

        Ok(process)
    }
}

impl Process {
    /// Get all flow elements as a FlowElements struct (for convenience)
    pub fn flow_elements(&self) -> FlowElements {
        FlowElements {
            start_events: self.start_events.clone(),
            end_events: self.end_events.clone(),
            tasks: self.tasks.clone(),
            manual_tasks: self.manual_tasks.clone(),
            user_tasks: self.user_tasks.clone(),
            service_tasks: self.service_tasks.clone(),
            script_tasks: self.script_tasks.clone(),
            exclusive_gateways: self.exclusive_gateways.clone(),
            parallel_gateways: self.parallel_gateways.clone(),
            sequence_flows: self.sequence_flows.clone(),
        }
    }

    pub fn validate_for_bpmn(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Process must have an id".to_string());
        }

        if self.start_events.is_empty() {
            return Err("Process must have at least one start event".to_string());
        }

        if self.end_events.is_empty() {
            return Err("Process must have at least one end event".to_string());
        }

        Ok(())
    }
}

impl Validate for Process {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Process must have an id".to_string());
        }
        Ok(())
    }
}
