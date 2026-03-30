//! Integration tests for BPMN parsing
//!
//! These tests validate that we can parse real BPMN files from the test fixtures.

use detent::bpmn::parse_bpmn;
use std::fs;

const HELLO_WORLD_BPMN: &str = "tests/assets/processes/hello-world/hello-world.bpmn2";

/// Test parsing the hello-world.bpmn2 file
#[test]
fn test_parse_hello_world_bpmn() {
    // Arrange
    let xml = fs::read_to_string(HELLO_WORLD_BPMN).expect("Failed to read hello-world.bpmn2");

    // Act
    let defs = parse_bpmn(&xml).expect("Failed to parse hello-world.bpmn2");

    // Assert
    assert_eq!(defs.id, "_sogPkOUBED6gTICEHR0M4w");

    let process = defs.process.expect("Expected a process");
    assert_eq!(process.id, "hello_world");
    assert_eq!(process.name, Some("hello-world".to_string()));
    assert_eq!(process.is_executable, Some(true));
    assert_eq!(process.process_type, Some("Public".to_string()));

    assert!(process.documentation.is_some());
    assert_eq!(
        process.documentation.as_ref().unwrap().text,
        "This is a hello world activity"
    );

    assert_eq!(process.start_events.len(), 1);
    let start = &process.start_events[0];
    assert_eq!(start.id, "_1E892844-423C-464F-ADC4-22F1EC73851B");
    assert_eq!(start.outgoing, vec!["_4083739B-66F0-4B92-A348-A37DF3B29083"]);

    assert_eq!(process.tasks.len(), 1);
    let task = &process.tasks[0];
    assert_eq!(task.id, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(task.name, Some("Hello World".to_string()));
    assert_eq!(task.incoming, vec!["_4083739B-66F0-4B92-A348-A37DF3B29083"]);
    assert_eq!(task.outgoing, vec!["_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB"]);
    assert!(task.documentation.is_some());
    assert_eq!(task.documentation.as_ref().unwrap().text, "T");

    assert_eq!(process.end_events.len(), 1);
    let end = &process.end_events[0];
    assert_eq!(end.id, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
    assert_eq!(end.incoming, vec!["_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB"]);

    assert_eq!(process.sequence_flows.len(), 2);

    let flow_start_to_task = process
        .sequence_flows
        .iter()
        .find(|f| f.id == "_4083739B-66F0-4B92-A348-A37DF3B29083")
        .expect("Missing flow from start to task");
    assert_eq!(flow_start_to_task.source_ref, "_1E892844-423C-464F-ADC4-22F1EC73851B");
    assert_eq!(flow_start_to_task.target_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");

    let flow_task_to_end = process
        .sequence_flows
        .iter()
        .find(|f| f.id == "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB")
        .expect("Missing flow from task to end");
    assert_eq!(flow_task_to_end.source_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(flow_task_to_end.target_ref, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
}
