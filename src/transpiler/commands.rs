//! Compatibility shim for historical `transpiler::commands` paths.
//!
//! Real command ownership in this branch lives under
//! [`crate::features::convert_bpmn_to_mdx::infrastructure::cli`].

pub mod compile;
pub mod import;
pub mod validate;
