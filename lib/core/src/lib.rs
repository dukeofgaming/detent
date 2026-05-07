//! Core domain layer for workflow process graphs.
//!
//! Zero-dependency types and algorithmns reusable across workflow engines
//! (BPMN, SWS, or any process standard). See `bpmn/` for the concrete types
//! and `process_graph` for the generic graph traversal/validation.

pub mod bpmn;
pub mod process_graph;
