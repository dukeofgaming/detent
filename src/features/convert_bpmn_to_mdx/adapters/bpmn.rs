//! BPMN 2.0 parsing and serialization
//!
//! This module provides types and utilities for working with BPMN 2.0 documents.

pub mod types;

pub use types::*;

use quick_xml::de::from_str;
use quick_xml::se::to_string;
use regex::Regex;
use std::sync::LazyLock;

/// Parse a BPMN XML string into Definitions
pub fn parse_bpmn(xml: &str) -> Result<Definitions, quick_xml::DeError> {
    let stripped = strip_unsupported(xml);
    from_str(&stripped)
}

/// Serialize Definitions to BPMN XML string
pub fn serialize_bpmn(definitions: &Definitions) -> Result<String, quick_xml::SeError> {
    to_string(definitions)
}

fn strip_unsupported(xml: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?s)<(?:bpmn:)?(laneSet|lane|collaboration|participant)\b[^>]*>.*?</(?:bpmn:)?(?:laneSet|lane|collaboration|participant)\s*>").unwrap()
    });
    RE.replace_all(xml, "").to_string()
}

