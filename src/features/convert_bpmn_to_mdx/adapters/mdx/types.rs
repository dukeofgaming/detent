//! MDX types for BPMN node representation
//!
//! MDX frontmatter uses BPMN types directly to ensure parity with the XML schema.
//! The YAML frontmatter field names match the BPMN XML attribute/element names exactly.

mod mdx_file;

pub use mdx_file::{MdxFile, MdxParseError};

// Re-export BPMN types for frontmatter use
pub use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{
    EndEvent, ExclusiveGateway, ManualTask, ParallelGateway, Process, ScriptTask, SequenceFlow,
    ServiceTask, StartEvent, Task, UserTask,
};
