use super::flow_node::FlowNode;
use super::sequence_flow::SequenceFlow;
use super::{EndEvent, ExclusiveGateway, ParallelGateway, ScriptTask, ServiceTask, StartEvent, Task};

#[derive(Clone)]
pub struct FlowElements {
    pub start_events: Vec<StartEvent>,
    pub end_events: Vec<EndEvent>,
    pub tasks: Vec<Task>,
    pub service_tasks: Vec<ServiceTask>,
    pub script_tasks: Vec<ScriptTask>,
    pub exclusive_gateways: Vec<ExclusiveGateway>,
    pub parallel_gateways: Vec<ParallelGateway>,
    pub sequence_flows: Vec<SequenceFlow>,
}

impl FlowElements {
    pub fn nodes(&self) -> impl Iterator<Item = FlowNode> + '_ {
        self.start_events
            .iter()
            .cloned()
            .map(FlowNode::StartEvent)
            .chain(self.end_events.iter().cloned().map(FlowNode::EndEvent))
            .chain(self.tasks.iter().cloned().map(FlowNode::Task))
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
