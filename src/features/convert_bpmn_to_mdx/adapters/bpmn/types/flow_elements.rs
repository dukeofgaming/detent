//! Flow elements container type

use serde::{Deserialize, Serialize};

use super::{
    EndEvent, ExclusiveGateway, FlowNode, ManualTask, ParallelGateway, ScriptTask, SequenceFlow,
    ServiceTask, StartEvent, Task, UserTask,
};

/// Container for all flow element types
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FlowElements {
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

impl FlowElements {
    /// Iterate over all flow nodes
    pub fn nodes(&self) -> impl Iterator<Item = FlowNode> + '_ {
        self.start_events
            .iter()
            .cloned()
            .map(FlowNode::StartEvent)
            .chain(self.end_events.iter().cloned().map(FlowNode::EndEvent))
            .chain(self.tasks.iter().cloned().map(FlowNode::Task))
            .chain(self.manual_tasks.iter().cloned().map(FlowNode::ManualTask))
            .chain(self.user_tasks.iter().cloned().map(FlowNode::UserTask))
            .chain(
                self.service_tasks
                    .iter()
                    .cloned()
                    .map(FlowNode::ServiceTask),
            )
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
