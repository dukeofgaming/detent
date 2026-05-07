//! Graph validation — Domain-layer semantic checks on Process graphs
//!
//! Provides graph-level validation that is standard-neutral (BPMN, SWS, etc.).
//! Graph invariants belong in the Domain layer per ADR-7.

pub mod bpmn;
pub mod graph;
