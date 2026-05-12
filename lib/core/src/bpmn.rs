//! BPMN domain types — pure structural data, no serialization.
//!
//! These types represent the BPMN process model independent of any
//! XML/serde/YAML format. They are the shared IR for all workflow
//! adapters (BPMN XML, MDX, SWS, etc.).

mod events;
mod flow_elements;
mod flow_node;
mod gateways;
mod process;
mod sequence_flow;
mod tasks;

pub use events::{EndEvent, StartEvent};
pub use flow_elements::FlowElements;
pub use flow_node::FlowNode;
pub use gateways::{ExclusiveGateway, ParallelGateway};
pub use process::Process;
pub use sequence_flow::SequenceFlow;
pub use tasks::{ScriptTask, ServiceTask, Task};
