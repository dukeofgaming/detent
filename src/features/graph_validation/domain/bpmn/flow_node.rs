use super::{EndEvent, ExclusiveGateway, ParallelGateway, ScriptTask, ServiceTask, StartEvent, Task};

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
    pub fn id(&self) -> &str {
        match self {
            FlowNode::StartEvent(x) => &x.id,
            FlowNode::EndEvent(x) => &x.id,
            FlowNode::Task(x) => &x.id,
            FlowNode::ServiceTask(x) => &x.id,
            FlowNode::ScriptTask(x) => &x.id,
            FlowNode::ExclusiveGateway(x) => &x.id,
            FlowNode::ParallelGateway(x) => &x.id,
        }
    }
}
