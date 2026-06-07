use super::{
    EndEvent, ExclusiveGateway, ParallelGateway, ScriptTask, SequenceFlow, ServiceTask, StartEvent,
    Task,
};

#[derive(Debug, Clone, PartialEq)]
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
