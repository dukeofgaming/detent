//! Detent - BPMN execution engine with MDX round-trip
//!
//! This crate provides:
//! - BPMN 2.0 parsing and serialization
//! - MDX-based workflow definitions
//! - Bidirectional MDX ↔ BPMN transpilation

pub mod transpiler;
pub mod graph_validation;
pub mod features;
