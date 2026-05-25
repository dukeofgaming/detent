use super::{EndEvent, ExclusiveGateway, ParallelGateway, ScriptTask, ServiceTask, StartEvent, Task};

#[derive(Clone)]
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
            FlowNode::StartEvent(e) => &e.id,
            FlowNode::EndEvent(e) => &e.id,
            FlowNode::Task(t) => &t.id,
            FlowNode::ServiceTask(t) => &t.id,
            FlowNode::ScriptTask(t) => &t.id,
            FlowNode::ExclusiveGateway(g) => &g.id,
            FlowNode::ParallelGateway(g) => &g.id,
        }
    }
}
