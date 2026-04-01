//! Tests for compiler::import - BPMN → MDX conversion (pure logic, no filesystem)

use detent::bpmn::{Definitions, EndEvent, Process, SequenceFlow, StartEvent, Task};
use detent::compiler::import::import_to_mdx;

/// Helper: build a minimal Definitions with a simple process
fn simple_definitions() -> Definitions {
    Definitions {
        id: "def_1".to_string(),
        name: Some("Test Process".to_string()),
        target_namespace: None,
        exporter: None,
        exporter_version: None,
        process: Some(Process {
            id: "process_1".to_string(),
            name: Some("Simple Process".to_string()),
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
                incoming: vec!["flow_2".to_string()],
                documentation: None,
            }],
            tasks: vec![Task {
                id: "task_1".to_string(),
                name: Some("Do Something".to_string()),
                incoming: vec!["flow_1".to_string()],
                outgoing: vec!["flow_2".to_string()],
                documentation: None,
            }],
            service_tasks: vec![],
            script_tasks: vec![],
            exclusive_gateways: vec![],
            parallel_gateways: vec![],
            sequence_flows: vec![
                SequenceFlow {
                    id: "flow_1".to_string(),
                    name: None,
                    source_ref: "start_1".to_string(),
                    target_ref: "task_1".to_string(),
                    condition_expression: None,
                    documentation: None,
                },
                SequenceFlow {
                    id: "flow_2".to_string(),
                    name: None,
                    source_ref: "task_1".to_string(),
                    target_ref: "end_1".to_string(),
                    condition_expression: None,
                    documentation: None,
                },
            ],
        }),
        bpmn_diagram: None,
    }
}

#[test]
fn test_import_returns_one_mdx_per_flow_element() {
    let defs = simple_definitions();
    let outputs = import_to_mdx(&defs).unwrap();

    // 1 start + 1 end + 1 task + 2 sequence flows = 5
    assert_eq!(outputs.len(), 5);
}

#[test]
fn test_import_mdx_output_has_correct_filenames() {
    let defs = simple_definitions();
    let outputs = import_to_mdx(&defs).unwrap();

    let filenames: Vec<&str> = outputs.iter().map(|o| o.filename.as_str()).collect();
    assert!(filenames.contains(&"start_1.mdx"));
    assert!(filenames.contains(&"end_1.mdx"));
    assert!(filenames.contains(&"task_1.mdx"));
    assert!(filenames.contains(&"flow_1.mdx"));
    assert!(filenames.contains(&"flow_2.mdx"));
}

#[test]
fn test_import_mdx_output_contains_frontmatter_delimiters() {
    let defs = simple_definitions();
    let outputs = import_to_mdx(&defs).unwrap();

    for output in &outputs {
        assert!(
            output.content.starts_with("---\n"),
            "MDX for {} should start with ---",
            output.filename
        );
        assert!(
            output.content.contains("\n---\n"),
            "MDX for {} should have closing ---",
            output.filename
        );
    }
}

#[test]
fn test_import_mdx_output_contains_bpmn_type() {
    let defs = simple_definitions();
    let outputs = import_to_mdx(&defs).unwrap();

    let start = outputs
        .iter()
        .find(|o| o.filename == "start_1.mdx")
        .unwrap();
    assert!(start.content.contains("type: bpmn:startEvent"));

    let end = outputs.iter().find(|o| o.filename == "end_1.mdx").unwrap();
    assert!(end.content.contains("type: bpmn:endEvent"));

    let task = outputs.iter().find(|o| o.filename == "task_1.mdx").unwrap();
    assert!(task.content.contains("type: bpmn:task"));

    let flow = outputs.iter().find(|o| o.filename == "flow_1.mdx").unwrap();
    assert!(flow.content.contains("type: bpmn:sequenceFlow"));
}

#[test]
fn test_import_errors_on_no_process() {
    let defs = Definitions {
        id: "def_1".to_string(),
        name: None,
        target_namespace: None,
        exporter: None,
        exporter_version: None,
        process: None,
        bpmn_diagram: None,
    };

    let result = import_to_mdx(&defs);
    assert!(result.is_err());
}

#[test]
fn test_import_frontmatter_has_no_at_or_dollar_prefixes() {
    let defs = simple_definitions();
    let outputs = import_to_mdx(&defs).unwrap();

    for output in &outputs {
        // Frontmatter should not contain @key or $key patterns from quick-xml
        assert!(
            !output.content.contains("'@"),
            "MDX for {} should not contain '@' prefixed keys",
            output.filename
        );
        assert!(
            !output.content.contains("\"@"),
            "MDX for {} should not contain \"@\" prefixed keys",
            output.filename
        );
        assert!(
            !output.content.contains("$text"),
            "MDX for {} should not contain $text keys",
            output.filename
        );
    }
}
