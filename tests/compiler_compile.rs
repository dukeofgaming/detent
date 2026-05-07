//! Tests for compiler::compile - MDX → BPMN conversion (pure logic, no filesystem)

use detent::compiler::compile::{compile_to_definitions, MdxInput};

/// Helper: build a set of MDX inputs representing a simple process
fn simple_mdx_inputs() -> Vec<MdxInput> {
    vec![
        MdxInput {
            filename: "start_1.mdx".to_string(),
            content: "\
---
type: bpmn:startEvent
id: start_1
outgoing:
- flow_1
---

# Start Event
"
            .to_string(),
        },
        MdxInput {
            filename: "end_1.mdx".to_string(),
            content: "\
---
type: bpmn:endEvent
id: end_1
incoming:
- flow_2
---

# End Event
"
            .to_string(),
        },
        MdxInput {
            filename: "task_1.mdx".to_string(),
            content: "\
---
type: bpmn:task
id: task_1
name: Do Something
incoming:
- flow_1
outgoing:
- flow_2
---

# Do Something
"
            .to_string(),
        },
        MdxInput {
            filename: "flow_1.mdx".to_string(),
            content: "\
---
type: bpmn:sequenceFlow
id: flow_1
sourceRef: start_1
targetRef: task_1
---

# Flow 1
"
            .to_string(),
        },
        MdxInput {
            filename: "flow_2.mdx".to_string(),
            content: "\
---
type: bpmn:sequenceFlow
id: flow_2
sourceRef: task_1
targetRef: end_1
---

# Flow 2
"
            .to_string(),
        },
    ]
}

#[test]
fn test_compile_produces_definitions_with_process() {
    let inputs = simple_mdx_inputs();
    let defs = compile_to_definitions(&inputs).unwrap();

    assert!(defs.process.is_some(), "Should produce a process");
}

#[test]
fn test_compile_process_has_correct_start_events() {
    let inputs = simple_mdx_inputs();
    let defs = compile_to_definitions(&inputs).unwrap();
    let process = defs.process.as_ref().unwrap();

    assert_eq!(process.start_events.len(), 1);
    assert_eq!(process.start_events[0].id, "start_1");
    assert_eq!(process.start_events[0].outgoing, vec!["flow_1"]);
}

#[test]
fn test_compile_process_has_correct_end_events() {
    let inputs = simple_mdx_inputs();
    let defs = compile_to_definitions(&inputs).unwrap();
    let process = defs.process.as_ref().unwrap();

    assert_eq!(process.end_events.len(), 1);
    assert_eq!(process.end_events[0].id, "end_1");
    assert_eq!(process.end_events[0].incoming, vec!["flow_2"]);
}

#[test]
fn test_compile_process_has_correct_tasks() {
    let inputs = simple_mdx_inputs();
    let defs = compile_to_definitions(&inputs).unwrap();
    let process = defs.process.as_ref().unwrap();

    assert_eq!(process.tasks.len(), 1);
    assert_eq!(process.tasks[0].id, "task_1");
    assert_eq!(process.tasks[0].name, Some("Do Something".to_string()));
    assert_eq!(process.tasks[0].incoming, vec!["flow_1"]);
    assert_eq!(process.tasks[0].outgoing, vec!["flow_2"]);
}

#[test]
fn test_compile_process_has_correct_sequence_flows() {
    let inputs = simple_mdx_inputs();
    let defs = compile_to_definitions(&inputs).unwrap();
    let process = defs.process.as_ref().unwrap();

    assert_eq!(process.sequence_flows.len(), 2);

    let flow1 = process
        .sequence_flows
        .iter()
        .find(|f| f.id == "flow_1")
        .expect("flow_1 should exist");
    assert_eq!(flow1.source_ref, "start_1");
    assert_eq!(flow1.target_ref, "task_1");

    let flow2 = process
        .sequence_flows
        .iter()
        .find(|f| f.id == "flow_2")
        .expect("flow_2 should exist");
    assert_eq!(flow2.source_ref, "task_1");
    assert_eq!(flow2.target_ref, "end_1");
}

#[test]
fn test_compile_errors_on_empty_inputs() {
    let inputs: Vec<MdxInput> = vec![];
    let result = compile_to_definitions(&inputs);

    assert!(result.is_err(), "Should error on empty inputs");
}

#[test]
fn test_compile_errors_on_missing_frontmatter() {
    let inputs = vec![MdxInput {
        filename: "bad.mdx".to_string(),
        content: "# No frontmatter here".to_string(),
    }];

    let result = compile_to_definitions(&inputs);
    assert!(result.is_err());
}

#[test]
fn test_compile_errors_on_missing_type_field() {
    let inputs = vec![MdxInput {
        filename: "no_type.mdx".to_string(),
        content: "\
---
id: start_1
outgoing:
- flow_1
---
"
        .to_string(),
    }];

    let result = compile_to_definitions(&inputs);
    assert!(result.is_err());
}

#[test]
fn test_compile_errors_on_unknown_type() {
    let inputs = vec![MdxInput {
        filename: "unknown.mdx".to_string(),
        content: "\
---
type: bpmn:unknownElement
id: x_1
---
"
        .to_string(),
    }];

    let result = compile_to_definitions(&inputs);
    assert!(result.is_err());
}

#[test]
fn test_compile_roundtrip_with_import() {
    // Compile MDX → Definitions, then import back to MDX
    // The structural data (ids, refs) should survive the roundtrip
    use detent::compiler::import::import_to_mdx;

    let inputs = simple_mdx_inputs();
    let defs = compile_to_definitions(&inputs).unwrap();
    let outputs = import_to_mdx(&defs).unwrap();

    // Same number of elements
    assert_eq!(outputs.len(), inputs.len());

    // All original IDs should be present in the output filenames
    for input in &inputs {
        assert!(
            outputs.iter().any(|o| o.filename == input.filename),
            "Output should contain file {}",
            input.filename
        );
    }
}

#[test]
fn test_compile_with_hello_world_reference_files() {
    // Use the actual reference MDX files from tests/assets
    let inputs = vec![
        MdxInput {
            filename: "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx".to_string(),
            content: std::fs::read_to_string(
                "tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx",
            )
            .unwrap(),
        },
        MdxInput {
            filename: "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx".to_string(),
            content: std::fs::read_to_string(
                "tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx",
            )
            .unwrap(),
        },
        MdxInput {
            filename: "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx".to_string(),
            content: std::fs::read_to_string(
                "tests/assets/processes/hello-world/_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx",
            )
            .unwrap(),
        },
        MdxInput {
            filename: "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx".to_string(),
            content: std::fs::read_to_string(
                "tests/assets/processes/hello-world/_4083739B-66F0-4B92-A348-A37DF3B29083.mdx",
            )
            .unwrap(),
        },
        MdxInput {
            filename: "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx".to_string(),
            content: std::fs::read_to_string(
                "tests/assets/processes/hello-world/_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx",
            )
            .unwrap(),
        },
    ];

    let defs = compile_to_definitions(&inputs).unwrap();
    let process = defs.process.as_ref().unwrap();

    // Verify structural integrity
    assert_eq!(process.start_events.len(), 1);
    assert_eq!(process.end_events.len(), 1);
    assert_eq!(process.tasks.len(), 1);
    assert_eq!(process.sequence_flows.len(), 2);

    // Verify task name survived
    assert_eq!(process.tasks[0].name, Some("Hello World".to_string()));

    // Verify the graph is valid
    use detent::graph_validation::graph::Graph;
    let graph = Graph::new(process);
    assert!(
        graph.validate().is_ok(),
        "Compiled process should form a valid graph"
    );
}
