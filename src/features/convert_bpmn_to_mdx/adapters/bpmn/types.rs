//! BPMN 2.0 types for the subset defined in spec.md
//!
//! These types are handcrafted to match the OMG BPMN 2.0 XSD structure
//! while being tailored for our MDX round-trip use case.

mod condition_expression;
mod definitions;
mod diagram;
mod documentation;
mod events;
mod flow_elements;
mod flow_node;
mod gateways;
mod process;
mod sequence_flow;
mod tasks;

pub use condition_expression::ConditionExpression;
pub use definitions::Definitions;
pub use diagram::{BPMNDiagram, BPMNEdge, BPMNLabel, BPMNPlane, BPMNShape, Bounds, Waypoint};
pub use documentation::Documentation;
pub use events::{EndEvent, StartEvent};
pub use flow_elements::FlowElements;
pub use flow_node::FlowNode;
pub use gateways::{ExclusiveGateway, ParallelGateway};
pub use process::Process;
pub use sequence_flow::SequenceFlow;
pub use tasks::{ManualTask, ScriptTask, ServiceTask, Task, UserTask};

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

/// BPMN 2.0 namespace
pub const BPMN_NS: &str = "http://www.omg.org/spec/BPMN/20100524/MODEL";
/// BPMN DI namespace
pub const BPMNDI_NS: &str = "http://www.omg.org/spec/BPMN/20100524/DI";
