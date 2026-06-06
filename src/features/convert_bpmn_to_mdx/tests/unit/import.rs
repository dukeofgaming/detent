use detent::features::convert_bpmn_to_mdx::adapters::bpmn::{
    Definitions, EndEvent, Process, SequenceFlow, StartEvent, Task,
};
use detent::features::convert_bpmn_to_mdx::use_cases::import::import_to_mdx;

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
            start_events: vec![StartEvent { id: "start_1".to_string(), name: None, outgoing: vec!["flow_1".to_string()], documentation: None }],
            end_events: vec![EndEvent { id: "end_1".to_string(), name: None, incoming: vec!["flow_2".to_string()], documentation: None }],
            tasks: vec![Task { id: "task_1".to_string(), name: Some("Do Something".to_string()), incoming: vec!["flow_1".to_string()], outgoing: vec!["flow_2".to_string()], documentation: None }],
            service_tasks: vec![],
            script_tasks: vec![],
            exclusive_gateways: vec![],
            parallel_gateways: vec![],
            sequence_flows: vec![
                SequenceFlow { id: "flow_1".to_string(), name: None, source_ref: "start_1".to_string(), target_ref: "task_1".to_string(), condition_expression: None, documentation: None },
                SequenceFlow { id: "flow_2".to_string(), name: None, source_ref: "task_1".to_string(), target_ref: "end_1".to_string(), condition_expression: None, documentation: None },
            ],
        }),
        bpmn_diagram: None,
    }
}

#[test]
fn test_import_mdx_output_has_correct_filenames() {
    let outputs = import_to_mdx(&simple_definitions()).unwrap();
    let names: Vec<&str> = outputs.iter().map(|o| o.filename.as_str()).collect();
    assert!(names.contains(&"start_1.mdx"));
    assert!(names.contains(&"end_1.mdx"));
    assert!(names.contains(&"task_1.mdx"));
    assert!(names.contains(&"flow_1.mdx"));
    assert!(names.contains(&"flow_2.mdx"));
}

#[test]
fn test_import_frontmatter_has_no_at_or_dollar_prefixes() {
    for output in &import_to_mdx(&simple_definitions()).unwrap() {
        assert!(
            !output.content.contains("'@"),
            "MDX for {} should not contain '@' keys",
            output.filename
        );
        assert!(
            !output.content.contains("\"@"),
            "MDX for {} should not contain \"@\" keys",
            output.filename
        );
        assert!(
            !output.content.contains("$text"),
            "MDX for {} should not contain $text keys",
            output.filename
        );
    }
}
