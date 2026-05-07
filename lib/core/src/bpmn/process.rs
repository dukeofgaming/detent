use super::flow_elements::FlowElements;
use super::{EndEvent, ExclusiveGateway, ParallelGateway, ScriptTask, SequenceFlow, ServiceTask, StartEvent, Task};

pub struct Process {
    pub id: String,
    pub name: Option<String>,
    pub is_executable: Option<bool>,
    pub process_type: Option<String>,
    pub documentation: Option<String>,
    pub start_events: Vec<StartEvent>,
    pub end_events: Vec<EndEvent>,
    pub tasks: Vec<Task>,
    pub service_tasks: Vec<ServiceTask>,
    pub script_tasks: Vec<ScriptTask>,
    pub exclusive_gateways: Vec<ExclusiveGateway>,
    pub parallel_gateways: Vec<ParallelGateway>,
    pub sequence_flows: Vec<SequenceFlow>,
}

impl Process {
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
