use cucumber::{given, then};
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::{
    ConditionExpression, Definitions, EndEvent, Process, SequenceFlow, StartEvent,
};

use super::ConvertWorld;

#[given("in-memory definitions whose sequence flow carries a condition expression")]
fn given_conditional_flow(world: &mut ConvertWorld) {
    world.parsed_defs = Some(Definitions {
        id: "def_1".to_string(),
        name: None,
        target_namespace: None,
        exporter: None,
        exporter_version: None,
        process: Some(Process {
            id: "process_1".to_string(),
            name: None,
            is_executable: Some(true),
            process_type: None,
            documentation: None,
            start_events: vec![StartEvent {
                id: "start_1".to_string(),
                name: None,
                outgoing: vec!["flow_1".to_string()],
                documentation: None,
            }],
            end_events: vec![EndEvent {
                id: "end_1".to_string(),
                name: None,
                incoming: vec!["flow_1".to_string()],
                documentation: None,
            }],
            tasks: vec![],
            service_tasks: vec![],
            script_tasks: vec![],
            exclusive_gateways: vec![],
            parallel_gateways: vec![],
            sequence_flows: vec![SequenceFlow {
                id: "flow_1".to_string(),
                name: None,
                source_ref: "start_1".to_string(),
                target_ref: "end_1".to_string(),
                condition_expression: Some(ConditionExpression {
                    xsi_type: Some("tFormalExpression".to_string()),
                    expression: "amount > 100".to_string(),
                }),
                documentation: None,
            }],
        }),
        bpmn_diagram: None,
    });
}

#[then("the flow_1 MDX output contains the condition \"amount > 100\"")]
fn then_condition_present(world: &mut ConvertWorld) {
    let outputs = world.import_outputs.as_ref().expect("expected import outputs");
    let flow = outputs
        .iter()
        .find(|o| o.filename == "flow_1.mdx")
        .expect("expected flow_1.mdx output");
    assert!(
        flow.content.contains("amount > 100"),
        "flow_1.mdx should contain the condition expression; got:\n{}",
        flow.content
    );
}
