use std::fs;

use cucumber::{given, then, when};

use super::super::ConvertWorld;

#[given("the hello-world BPMN XML for flow node tests")]
fn given_bpmn(world: &mut ConvertWorld) {
    let xml = fs::read_to_string(super::super::hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read BPMN");
    world.bpmn_xml = Some(xml);
}

#[when("I inspect flow node ids from the parsed process")]
fn when_flow_node_ids(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    let defs = detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(xml)
        .expect("Failed to parse BPMN");
    let process = defs.process.expect("Expected process");
    let start = detent::features::convert_bpmn_to_mdx::adapters::bpmn::FlowNode::StartEvent(
        process.start_events[0].clone(),
    );
    let task =
        detent::features::convert_bpmn_to_mdx::adapters::bpmn::FlowNode::Task(process.tasks[0].clone());
    let end = detent::features::convert_bpmn_to_mdx::adapters::bpmn::FlowNode::EndEvent(
        process.end_events[0].clone(),
    );
    world.e2e_file_content = Some(format!(
        "{}:{}|{}:{}|{}:{}",
        start.id(),
        start.type_name(),
        task.id(),
        task.type_name(),
        end.id(),
        end.type_name()
    ));
}

#[then("flow node ids and types match hello-world")]
fn then_flow_node_ids(world: &mut ConvertWorld) {
    assert_eq!(
        world.e2e_file_content.as_deref(),
        Some("_1E892844-423C-464F-ADC4-22F1EC73851B:bpmn:startEvent|_808AA40C-EAA1-40C4-A2DC-27000FBF1866:bpmn:task|_D3F6E97D-7783-492C-98CE-57EC815D304C:bpmn:endEvent")
    );
}

#[when("I inspect the first sequence flow")]
fn when_sequence_flow(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    // Act
    let defs = detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(xml)
        .expect("Failed to parse BPMN");
    let process = defs.process.expect("Expected process");
    let flow = &process.sequence_flows[0];
    // Assert
    assert!(!flow.id.is_empty());
    assert!(!flow.source_ref.is_empty());
    assert!(!flow.target_ref.is_empty());
    world.e2e_last_success = true;
}

#[then("the sequence flow has non-empty id and refs")]
fn then_sequence_flow(_world: &mut ConvertWorld) {}
