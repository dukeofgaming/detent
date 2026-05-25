use super::{FlowNode, Process};

pub struct FlowElements<'a> {
    process: &'a Process,
}

impl<'a> FlowElements<'a> {
    pub fn nodes(&self) -> impl Iterator<Item = FlowNode> + '_ {
        self.process
            .start_events
            .iter()
            .cloned()
            .map(FlowNode::StartEvent)
            .chain(self.process.end_events.iter().cloned().map(FlowNode::EndEvent))
            .chain(self.process.tasks.iter().cloned().map(FlowNode::Task))
            .chain(
                self.process
                    .service_tasks
                    .iter()
                    .cloned()
                    .map(FlowNode::ServiceTask),
            )
            .chain(
                self.process
                    .script_tasks
                    .iter()
                    .cloned()
                    .map(FlowNode::ScriptTask),
            )
            .chain(
                self.process
                    .exclusive_gateways
                    .iter()
                    .cloned()
                    .map(FlowNode::ExclusiveGateway),
            )
            .chain(
                self.process
                    .parallel_gateways
                    .iter()
                    .cloned()
                    .map(FlowNode::ParallelGateway),
            )
    }
}

impl Process {
    pub fn flow_elements(&self) -> FlowElements<'_> {
        FlowElements { process: self }
    }
}
