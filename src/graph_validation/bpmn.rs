//! Minimal BPMN types for graph validation (self-contained slice).
//! No XML/serde dependencies — pure Rust structs for the graph IR.

pub use sequence_flow::SequenceFlow;
pub use types::*;

mod types {
    use super::SequenceFlow;

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

    #[derive(Clone)]
    pub struct StartEvent {
        pub id: String,
        pub name: Option<String>,
        pub outgoing: Vec<String>,
        pub documentation: Option<String>,
    }

    #[derive(Clone)]
    pub struct EndEvent {
        pub id: String,
        pub name: Option<String>,
        pub incoming: Vec<String>,
        pub documentation: Option<String>,
    }

    #[derive(Clone)]
    pub struct Task {
        pub id: String,
        pub name: Option<String>,
        pub incoming: Vec<String>,
        pub outgoing: Vec<String>,
        pub documentation: Option<String>,
    }

    #[derive(Clone)]
    pub struct ServiceTask {
        pub id: String,
    }

    #[derive(Clone)]
    pub struct ScriptTask {
        pub id: String,
    }

    #[derive(Clone)]
    pub struct ExclusiveGateway {
        pub id: String,
    }

    #[derive(Clone)]
    pub struct ParallelGateway {
        pub id: String,
    }
}

mod sequence_flow {
    #[derive(Clone)]
    pub struct SequenceFlow {
        pub id: String,
        pub name: Option<String>,
        pub source_ref: String,
        pub target_ref: String,
        pub condition_expression: Option<String>,
        pub documentation: Option<String>,
    }
}

pub use types::EndEvent;
pub use types::StartEvent;
pub use types::Task;
pub use types::ServiceTask;
pub use types::ScriptTask;
pub use types::ExclusiveGateway;
pub use types::ParallelGateway;
pub use types::FlowNode;
pub use types::FlowElements;
