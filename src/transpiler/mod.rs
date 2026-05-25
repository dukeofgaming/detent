//! Compatibility shim for the historical `transpiler` module path.
//!
//! The real feature ownership now lives under [`crate::features`].
//! In this branch, the standalone behavior comes from the
//! `convert_bpmn_to_mdx` slice (`#3`).
//!
//! This module remains only to preserve existing paths while the refactor settles.

pub mod bpmn;
pub mod commands;
pub mod compile;
pub mod import;
pub mod mdx;
