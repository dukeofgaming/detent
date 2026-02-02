//! BPMN 2.0 parsing and serialization
//!
//! This module provides types and utilities for working with BPMN 2.0 documents.

pub mod types;

#[cfg(feature = "xsd-validation")]
pub mod xsd_validator;

pub use types::*;

#[cfg(feature = "xsd-validation")]
pub use xsd_validator::{validate_bpmn_xsd, validate_bpmn_file_xsd, XsdValidationError};

use quick_xml::de::from_str;
use quick_xml::se::to_string;

/// Parse a BPMN XML string into Definitions
pub fn parse_bpmn(xml: &str) -> Result<Definitions, quick_xml::DeError> {
    from_str(xml)
}

/// Serialize Definitions to BPMN XML string
pub fn serialize_bpmn(definitions: &Definitions) -> Result<String, quick_xml::SeError> {
    to_string(definitions)
}

