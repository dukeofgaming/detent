use detent::features::convert_bpmn_to_mdx::use_cases::compile::{compile_to_definitions, MdxInput};

use super::hello_world_asset_path;

fn simple_mdx_inputs() -> Vec<MdxInput> {
    vec![
        MdxInput { filename: "start_1.mdx".to_string(), content: "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n\n# Start Event\n".to_string() },
        MdxInput { filename: "end_1.mdx".to_string(), content: "---\ntype: bpmn:endEvent\nid: end_1\nincoming:\n- flow_2\n---\n\n# End Event\n".to_string() },
        MdxInput { filename: "task_1.mdx".to_string(), content: "---\ntype: bpmn:task\nid: task_1\nname: Do Something\nincoming:\n- flow_1\noutgoing:\n- flow_2\n---\n\n# Do Something\n".to_string() },
        MdxInput { filename: "flow_1.mdx".to_string(), content: "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: task_1\n---\n\n# Flow 1\n".to_string() },
        MdxInput { filename: "flow_2.mdx".to_string(), content: "---\ntype: bpmn:sequenceFlow\nid: flow_2\nsourceRef: task_1\ntargetRef: end_1\n---\n\n# Flow 2\n".to_string() },
    ]
}

#[test]
fn test_compile_produces_definitions_with_process() {
    let defs = compile_to_definitions(&simple_mdx_inputs()).unwrap();
    assert!(defs.process.is_some());
}

#[test]
fn test_compile_process_has_correct_elements() {
    let defs = compile_to_definitions(&simple_mdx_inputs()).unwrap();
    let process = defs.process.as_ref().unwrap();
    assert_eq!(process.start_events.len(), 1);
    assert_eq!(process.start_events[0].id, "start_1");
    assert_eq!(process.end_events.len(), 1);
    assert_eq!(process.end_events[0].id, "end_1");
    assert_eq!(process.tasks.len(), 1);
    assert_eq!(process.tasks[0].id, "task_1");
    assert_eq!(process.sequence_flows.len(), 2);
}

#[test]
fn test_compile_errors_on_empty_inputs() {
    assert!(compile_to_definitions(&[]).is_err());
}

#[test]
fn test_compile_errors_on_missing_frontmatter() {
    assert!(compile_to_definitions(&[MdxInput { filename: "bad.mdx".to_string(), content: "# No frontmatter".to_string() }]).is_err());
}

#[test]
fn test_compile_errors_on_missing_type_field() {
    assert!(compile_to_definitions(&[MdxInput { filename: "no_type.mdx".to_string(), content: "---\nid: start_1\n---\n".to_string() }]).is_err());
}

#[test]
fn test_compile_errors_on_unknown_type() {
    assert!(compile_to_definitions(&[MdxInput { filename: "unknown.mdx".to_string(), content: "---\ntype: bpmn:unknownElement\nid: x_1\n---\n".to_string() }]).is_err());
}

#[test]
fn test_compile_allows_invalid_graphs_in_feature_3_scope() {
    let inputs = vec![
        MdxInput { filename: "start_1.mdx".to_string(), content: "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n".to_string() },
        MdxInput { filename: "flow_1.mdx".to_string(), content: "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: ghost_task\n---\n".to_string() },
    ];

    let defs = compile_to_definitions(&inputs)
        .expect("feature #3 transpilation should not enforce graph semantics");
    let process = defs.process.as_ref().unwrap();
    assert_eq!(process.start_events.len(), 1);
    assert_eq!(process.sequence_flows.len(), 1);
    assert_eq!(process.sequence_flows[0].target_ref, "ghost_task");
}

#[test]
fn test_compile_roundtrip_with_import() {
    let inputs = simple_mdx_inputs();
    let defs = compile_to_definitions(&inputs).unwrap();
    let outputs = detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx(&defs)
        .unwrap();
    assert_eq!(outputs.len(), inputs.len());
    for input in &inputs {
        assert!(
            outputs.iter().any(|o| o.filename == input.filename),
            "Missing file {}",
            input.filename
        );
    }
}

#[test]
fn test_compile_with_hello_world_reference_files() {
    let inputs = vec![
        MdxInput { filename: "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")).unwrap() },
        MdxInput { filename: "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")).unwrap() },
        MdxInput { filename: "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx")).unwrap() },
        MdxInput { filename: "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_4083739B-66F0-4B92-A348-A37DF3B29083.mdx")).unwrap() },
        MdxInput { filename: "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx".to_string(), content: std::fs::read_to_string(hello_world_asset_path("_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx")).unwrap() },
    ];
    let defs = compile_to_definitions(&inputs).unwrap();
    let process = defs.process.as_ref().unwrap();
    assert_eq!(process.tasks[0].name, Some("Hello World".to_string()));
}
