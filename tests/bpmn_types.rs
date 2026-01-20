//! Unit tests for BPMN types

use detent::bpmn::{parse_bpmn, FlowNode, StartEvent};

#[test]
fn test_flow_node_id() {
    let start = FlowNode::StartEvent(StartEvent {
        id: "start_1".to_string(),
        name: Some("Start".to_string()),
        outgoing: vec!["flow_1".to_string()],
        documentation: None,
    });
    assert_eq!(start.id(), "start_1");
    assert_eq!(start.type_name(), "bpmn:startEvent");
}

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
