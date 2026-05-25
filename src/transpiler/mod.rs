//! Bidirectional transpiler: MDX ↔ BPMN
//!
//! This module implements the shared intermediate representation (IR) architecture
//! described in ADR-2. Both directions pass through the same BPMN types (the IR).
//!
//! - `import`: BPMN XML → IR → MDX files
//! - `compile`: MDX files → IR → BPMN XML

pub mod bpmn;
pub mod commands;
pub mod compile;
pub mod import;
pub mod mdx;

#[cfg(test)]
mod tests;
