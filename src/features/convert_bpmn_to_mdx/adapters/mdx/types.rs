//! MDX types for BPMN node representation
//!
//! MDX frontmatter uses BPMN types directly to ensure parity with the XML schema.
//! The YAML frontmatter field names match the BPMN XML attribute/element names exactly.

mod backoff_config;
mod io_spec;
mod mdx_file;
mod retry_config;

pub use backoff_config::BackoffConfig;
pub use io_spec::IoSpec;
pub use mdx_file::{MdxFile, MdxParseError};
pub use retry_config::RetryConfig;

// Re-export BPMN types for frontmatter use
pub use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{
    EndEvent, ExclusiveGateway, ParallelGateway, Process, ScriptTask, SequenceFlow, ServiceTask,
    StartEvent, Task,
};
