//! Graph validation — Domain-layer semantic checks on Process graphs
//!
//! Provides graph-level validation that is standard-neutral (BPMN, SWS, etc.).
//! Graph invariants belong in the Domain layer per ADR-7.
//!
//! This module re-exports from the `detent-core` crate, which holds the
//! framework-agnostic domain types and graph operations.
//!
//! # Architecture
//!
//! Adapter→domain conversions (e.g., `to_domain_process`) belong in the
//! **adapter layer**, not here. This module is a thin re-export shim.
//! See [`crate::transpiler::bpmn::to_domain`] for BPMN→domain conversion.

pub use detent_core::bpmn;
pub use detent_core::process_graph::Graph;

#[cfg(test)]
mod tests;
