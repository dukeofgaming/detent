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

    /// Get the name of this flow node
    pub fn name(&self) -> Option<&str> {
        match self {
            FlowNode::StartEvent(e) => e.name.as_deref(),
            FlowNode::EndEvent(e) => e.name.as_deref(),
            FlowNode::Task(t) => t.name.as_deref(),
            FlowNode::ManualTask(t) => t.name.as_deref(),
            FlowNode::UserTask(t) => t.name.as_deref(),
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
            FlowNode::ManualTask(_) => "bpmn:manualTask",
            FlowNode::UserTask(_) => "bpmn:userTask",
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
            FlowNode::ManualTask(t) => &t.incoming,
            FlowNode::UserTask(t) => &t.incoming,
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
            FlowNode::ManualTask(t) => &t.outgoing,
            FlowNode::UserTask(t) => &t.outgoing,
            FlowNode::ServiceTask(t) => &t.outgoing,
            FlowNode::ScriptTask(t) => &t.outgoing,
            FlowNode::ExclusiveGateway(g) => &g.outgoing,
            FlowNode::ParallelGateway(g) => &g.outgoing,
        }
    }
}
