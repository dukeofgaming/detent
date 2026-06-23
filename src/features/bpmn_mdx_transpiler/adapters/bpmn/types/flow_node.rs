//! Unified enum for all flow node types

use super::{
    EndEvent, ExclusiveGateway, ManualTask, ParallelGateway, ScriptTask, ServiceTask, StartEvent,
    Task, UserTask,
};

/// Unified enum for all flow node types (for graph operations)
#[derive(Debug, Clone, PartialEq)]
pub enum FlowNode {
    StartEvent(StartEvent),
    EndEvent(EndEvent),
    Task(Task),
    ManualTask(ManualTask),
    UserTask(UserTask),
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
            FlowNode::ManualTask(t) => &t.id,
            FlowNode::UserTask(t) => &t.id,
            FlowNode::ServiceTask(t) => &t.id,
            FlowNode::ScriptTask(t) => &t.id,
            FlowNode::ExclusiveGateway(g) => &g.id,
            FlowNode::ParallelGateway(g) => &g.id,
        }
    }

    /// Get the BPMN type name (e.g., "bpmn:startEvent")
    pub fn type_name(&self) -> &'static str {
        match self {
            FlowNode::StartEvent(_) => "bpmn:startEvent",
            FlowNode::EndEvent(_) => "bpmn:endEvent",
            FlowNode::Task(_) => "bpmn:task",
            FlowNode::ManualTask(_) => "bpmn:manualTask",
            FlowNode::UserTask(_) => "bpmn:userTask",
            FlowNode::ServiceTask(_) => "bpmn:serviceTask",
            FlowNode::ScriptTask(_) => "bpmn:scriptTask",
            FlowNode::ExclusiveGateway(_) => "bpmn:exclusiveGateway",
            FlowNode::ParallelGateway(_) => "bpmn:parallelGateway",
        }
    }
}
