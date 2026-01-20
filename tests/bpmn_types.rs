//! Unit tests for BPMN types

use detent::bpmn::{parse_bpmn, FlowNode};
use std::fs;

const HELLO_WORLD_BPMN: &str = "tests/assets/processes/hello-world/hello-world.bpmn2";

#[test]
fn test_flow_node_id() {
    // Arrange
    let xml = fs::read_to_string(HELLO_WORLD_BPMN).expect("Failed to read BPMN");
    let defs = parse_bpmn(&xml).expect("Failed to parse BPMN");
    let process = defs.process.expect("Expected process");

    // Act & Assert
    let start = FlowNode::StartEvent(process.start_events[0].clone());
    assert_eq!(start.id(), "_1E892844-423C-464F-ADC4-22F1EC73851B");
    assert_eq!(start.type_name(), "bpmn:startEvent");

    let task = FlowNode::Task(process.tasks[0].clone());
    assert_eq!(task.id(), "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(task.type_name(), "bpmn:task");

    let end = FlowNode::EndEvent(process.end_events[0].clone());
    assert_eq!(end.id(), "_D3F6E97D-7783-492C-98CE-57EC815D304C");
    assert_eq!(end.type_name(), "bpmn:endEvent");
}

#[test]
fn test_flow_node_from_sequence_flow() {
    // Arrange
    let xml = fs::read_to_string(HELLO_WORLD_BPMN).expect("Failed to read BPMN");
    let defs = parse_bpmn(&xml).expect("Failed to parse BPMN");
    let process = defs.process.expect("Expected process");

    // Act
    let flow = &process.sequence_flows[0];

    // Assert
    assert!(!flow.id.is_empty());
    assert!(!flow.source_ref.is_empty());
    assert!(!flow.target_ref.is_empty());
}
