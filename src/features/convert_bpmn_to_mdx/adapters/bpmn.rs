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
        Regex::new(r"(?s)<(?:bpmn:)?(userTask|manualTask|laneSet|lane|collaboration|participant)\b[^>]*>.*?</(?:bpmn:)?(?:userTask|manualTask|laneSet|lane|collaboration|participant)\s*>").unwrap()
    });
    RE.replace_all(xml, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_removes_user_task() {
        let xml = r#"<bpmn:process id="p">
    <bpmn:serviceTask id="st1"/>
    <bpmn:userTask id="ut1"><bpmn:incoming>f1</bpmn:incoming></bpmn:userTask>
    <bpmn:serviceTask id="st2"/>
</bpmn:process>"#;

        let stripped = strip_unsupported(xml);
        assert!(!stripped.contains("userTask"), "userTask should be removed: {}", stripped);
        assert!(stripped.contains("st1"), "serviceTask st1 should remain");
        assert!(stripped.contains("st2"), "serviceTask st2 should remain");
    }
}
