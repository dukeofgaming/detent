use std::fs;

use super::hello_world_asset_path;

#[test]
fn test_flow_node_id() {
    let xml = fs::read_to_string(hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read BPMN");
    let defs = detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(&xml)
        .expect("Failed to parse BPMN");
    let process = defs.process.expect("Expected process");
    let start = detent::features::convert_bpmn_to_mdx::adapters::bpmn::FlowNode::StartEvent(
        process.start_events[0].clone(),
    );
    assert_eq!(start.id(), "_1E892844-423C-464F-ADC4-22F1EC73851B");
    assert_eq!(start.type_name(), "bpmn:startEvent");
    let task =
        detent::features::convert_bpmn_to_mdx::adapters::bpmn::FlowNode::Task(process.tasks[0].clone());
    assert_eq!(task.id(), "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(task.type_name(), "bpmn:task");
    let end = detent::features::convert_bpmn_to_mdx::adapters::bpmn::FlowNode::EndEvent(
        process.end_events[0].clone(),
    );
    assert_eq!(end.id(), "_D3F6E97D-7783-492C-98CE-57EC815D304C");
    assert_eq!(end.type_name(), "bpmn:endEvent");
}

#[test]
fn test_flow_node_from_sequence_flow() {
    let xml = fs::read_to_string(hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read BPMN");
    let defs = detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(&xml)
        .expect("Failed to parse BPMN");
    let process = defs.process.expect("Expected process");
    let flow = &process.sequence_flows[0];
    assert!(!flow.id.is_empty());
    assert!(!flow.source_ref.is_empty());
    assert!(!flow.target_ref.is_empty());
}
