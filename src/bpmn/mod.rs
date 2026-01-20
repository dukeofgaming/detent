//! BPMN 2.0 parsing and serialization
//!
//! This module provides types and utilities for working with BPMN 2.0 documents.

pub mod types;

pub use types::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_bpmn() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions id="def_1" xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL">
    <process id="process_1" isExecutable="true">
        <startEvent id="start_1">
            <outgoing>flow_1</outgoing>
        </startEvent>
        <endEvent id="end_1">
            <incoming>flow_1</incoming>
        </endEvent>
        <sequenceFlow id="flow_1" sourceRef="start_1" targetRef="end_1"/>
    </process>
</definitions>"#;

        let result = parse_bpmn(xml);
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        
        let defs = result.unwrap();
        assert_eq!(defs.id, "def_1");
        assert!(defs.process.is_some());
        
        let process = defs.process.as_ref().unwrap();
        assert_eq!(process.id, "process_1");
        assert_eq!(process.start_events.len(), 1);
        assert_eq!(process.end_events.len(), 1);
        assert_eq!(process.sequence_flows.len(), 1);
    }
}
