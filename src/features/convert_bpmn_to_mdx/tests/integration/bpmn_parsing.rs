use std::fs;

use super::hello_world_asset_path;

#[test]
fn test_parse_hello_world_bpmn() {
    let xml = fs::read_to_string(hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read hello-world.bpmn2");
    let defs = detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(&xml)
        .expect("Failed to parse hello-world.bpmn2");
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
    assert_eq!(
        process.start_events[0].id,
        "_1E892844-423C-464F-ADC4-22F1EC73851B"
    );
    assert_eq!(process.tasks.len(), 1);
    assert_eq!(process.tasks[0].id, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    assert_eq!(process.end_events.len(), 1);
    assert_eq!(
        process.end_events[0].id,
        "_D3F6E97D-7783-492C-98CE-57EC815D304C"
    );
    assert_eq!(process.sequence_flows.len(), 2);
}
