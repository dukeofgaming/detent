//! Compatibility shim for historical `graph_validation` paths.
//!
//! The `#4` graph-validation feature now owns its domain model, adapters,
//! use cases, infrastructure, and tests under [`crate::features::graph_validation`].
//! This module re-exports the public graph-validation domain surface only.

pub use crate::features::graph_validation::domain::bpmn;
pub use crate::features::graph_validation::domain::graph::Graph;
