//! Graph validation — Domain-layer semantic checks on Process graphs
//!
//! Provides graph-level validation that is standard-neutral (BPMN, SWS, etc.).
//! Graph invariants belong in the Domain layer per ADR-7.
//!
//! This module re-exports from the graph_validation feature slice.
//!
//! # Architecture
//!
//! Adapter→domain conversions (e.g., `to_domain_process`) belong in the
//! **adapter layer**, not here. This module is a thin re-export shim.
//! See [`crate::transpiler::bpmn::to_domain`] for BPMN→domain conversion.

pub use crate::features::graph_validation::domain::bpmn;
pub use crate::features::graph_validation::domain::graph::Graph;
