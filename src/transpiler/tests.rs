//! Tests for the transpiler slice — co-located with the code they test.
//!
//! These replace the former integration tests in `tests/compiler/`.
//! They are compiled as part of the crate (`#[cfg(test)]`), giving access to
//! private API surfaces while still exercising the public CLI via `assert_cmd`.

// ============================================================
// BPMN parsing
// ============================================================

mod bpmn_parsing {
    use crate::transpiler::bpmn::parse_bpmn;
    use std::fs;

    const HELLO_WORLD_BPMN: &str = "tests/assets/processes/hello-world/hello-world.bpmn2";

    #[test]
    fn test_parse_hello_world_bpmn() {
        let xml = fs::read_to_string(HELLO_WORLD_BPMN).expect("Failed to read hello-world.bpmn2");
        let defs = parse_bpmn(&xml).expect("Failed to parse hello-world.bpmn2");
        assert_eq!(defs.id, "_sogPkOUBED6gTICEHR0M4w");
        let process = defs.process.expect("Expected a process");
        assert_eq!(process.id, "hello_world");
        assert_eq!(process.name, Some("hello-world".to_string()));
        assert_eq!(process.is_executable, Some(true));
        assert_eq!(process.process_type, Some("Public".to_string()));
        assert!(process.documentation.is_some());
        assert_eq!(process.documentation.as_ref().unwrap().text, "This is a hello world activity");
        assert_eq!(process.start_events.len(), 1);
        assert_eq!(process.start_events[0].id, "_1E892844-423C-464F-ADC4-22F1EC73851B");
        assert_eq!(process.tasks.len(), 1);
        assert_eq!(process.tasks[0].id, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
        assert_eq!(process.end_events.len(), 1);
        assert_eq!(process.end_events[0].id, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
        assert_eq!(process.sequence_flows.len(), 2);
    }
}

// ============================================================
// BPMN types
// ============================================================

mod bpmn_types {
    use crate::transpiler::bpmn::{parse_bpmn, FlowNode};
    use std::fs;

    const HELLO_WORLD_BPMN: &str = "tests/assets/processes/hello-world/hello-world.bpmn2";

    #[test]
    fn test_flow_node_id() {
        let xml = fs::read_to_string(HELLO_WORLD_BPMN).expect("Failed to read BPMN");
        let defs = parse_bpmn(&xml).expect("Failed to parse BPMN");
        let process = defs.process.expect("Expected process");
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
        let xml = fs::read_to_string(HELLO_WORLD_BPMN).expect("Failed to read BPMN");
        let defs = parse_bpmn(&xml).expect("Failed to parse BPMN");
        let process = defs.process.expect("Expected process");
        let flow = &process.sequence_flows[0];
        assert!(!flow.id.is_empty());
        assert!(!flow.source_ref.is_empty());
        assert!(!flow.target_ref.is_empty());
    }
}

// ============================================================
// BPMN XSD validation (feature-gated)
// ============================================================

#[cfg(feature = "xsd-validation")]
mod bpmn_xsd_validation {
    use crate::transpiler::bpmn::validate_bpmn_xsd;

    const HELLO_WORLD_BPMN: &str = include_str!("../../tests/assets/processes/hello-world/hello-world.bpmn");
    const HELLO_WORLD_BPMN2: &str = include_str!("../../tests/assets/processes/hello-world/hello-world.bpmn2");

    fn hello_world_fixtures() -> [(&'static str, &'static str); 2] {
        [("hello-world.bpmn", HELLO_WORLD_BPMN), ("hello-world.bpmn2", HELLO_WORLD_BPMN2)]
    }

    #[test]
    fn test_valid_hello_world_files_pass_xsd_validation() {
        for (name, content) in hello_world_fixtures() {
            let result = validate_bpmn_xsd(content);
            assert!(result.is_ok(), "Expected {} to pass XSD validation: {:?}", name, result);
        }
    }

    #[test]
    fn test_invalid_element_fails_xsd_validation() {
        for (name, content) in hello_world_fixtures() {
            let invalid_bpmn = content
                .replace("<bpmn2:task", "<bpmn2:notARealTask")
                .replace("</bpmn2:task>", "</bpmn2:notARealTask>");
            let result = validate_bpmn_xsd(&invalid_bpmn);
            assert!(result.is_err(), "Expected {} with invalid element to fail", name);
        }
    }

    #[test]
    fn test_missing_required_attribute_fails_xsd_validation() {
        for (name, content) in hello_world_fixtures() {
            let invalid_bpmn = content.replacen("targetNamespace=", "removedTargetNamespace=", 1);
            let result = validate_bpmn_xsd(&invalid_bpmn);
            assert!(result.is_err(), "Expected {} without targetNamespace to fail", name);
        }
    }
}

// ============================================================
// CLI compile command
// ============================================================

mod cli_compile {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;

    #[allow(deprecated)]
    fn detent() -> Command {
        Command::cargo_bin("detent").expect("Failed to find detent binary")
    }

    #[test]
    fn test_compile_help() {
        detent().arg("compile").arg("--help").assert().success()
            .stdout(predicate::str::contains("MDX files"));
    }

    #[test]
    fn test_compile_requires_directory() {
        detent().arg("compile").assert().failure();
    }

    #[test]
    fn test_compile_missing_directory() {
        detent().arg("compile").arg("nonexistent-dir").assert().failure()
            .stderr(predicate::str::contains("Failed to read"));
    }

    #[test]
    fn test_compile_produces_bpmn_xml_file() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_file = temp_dir.path().join("output.bpmn");
        detent().arg("compile").arg("tests/assets/processes/hello-world").arg("--output").arg(&output_file)
            .assert().success();
        assert!(output_file.exists(), "Output BPMN file should exist");
        let content = fs::read_to_string(&output_file).expect("Failed to read output");
        assert!(content.contains("definitions"));
        assert!(content.contains("process"));
    }

    #[test]
    fn test_compile_output_contains_all_elements() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_file = temp_dir.path().join("output.bpmn");
        detent().arg("compile").arg("tests/assets/processes/hello-world").arg("--output").arg(&output_file)
            .assert().success();
        let content = fs::read_to_string(&output_file).expect("Failed to read output");
        assert!(content.contains("_1E892844-423C-464F-ADC4-22F1EC73851B"));
        assert!(content.contains("_808AA40C-EAA1-40C4-A2DC-27000FBF1866"));
        assert!(content.contains("_D3F6E97D-7783-492C-98CE-57EC815D304C"));
    }

    #[test]
    fn test_compile_writes_to_stdout_by_default() {
        detent().arg("compile").arg("tests/assets/processes/hello-world").assert().success()
            .stdout(predicate::str::contains("definitions"))
            .stdout(predicate::str::contains("process"));
    }

    #[test]
    fn test_compile_ignores_non_mdx_files() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_file = temp_dir.path().join("output.bpmn");
        detent().arg("compile").arg("tests/assets/processes/hello-world").arg("--output").arg(&output_file)
            .assert().success();
    }
}

// ============================================================
// CLI import command
// ============================================================

mod cli_import {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use std::path::Path;

    #[allow(deprecated)]
    fn detent() -> Command {
        Command::cargo_bin("detent").expect("Failed to find detent binary")
    }

    const REFERENCE_DIR: &str = "tests/assets/processes/hello-world";
    const EXPECTED_MDX_FILES: &[&str] = &[
        "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx",
        "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx",
        "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx",
        "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx",
        "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx",
    ];

    fn extract_frontmatter(content: &str) -> Option<String> {
        let content = content.trim();
        if !content.starts_with("---") { return None; }
        let rest = &content[3..];
        let end_pos = rest.find("\n---")?;
        Some(rest[..end_pos].trim().to_string())
    }

    #[test]
    fn test_import_help() {
        detent().arg("import").arg("--help").assert().success()
            .stdout(predicate::str::contains("BPMN XML file"));
    }

    #[test]
    fn test_import_generates_all_expected_files() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_dir = temp_dir.path();
        detent().arg("import").arg("tests/assets/processes/hello-world/hello-world.bpmn2")
            .arg("--output-directory").arg(output_dir).assert().success();
        for file_name in EXPECTED_MDX_FILES {
            assert!(output_dir.join(file_name).exists(), "Expected file not generated: {}", file_name);
        }
    }

    #[test]
    fn test_import_frontmatter_matches_reference() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let output_dir = temp_dir.path();
        detent().arg("import").arg("tests/assets/processes/hello-world/hello-world.bpmn2")
            .arg("--output-directory").arg(output_dir).assert().success();
        for file_name in EXPECTED_MDX_FILES {
            let generated = fs::read_to_string(output_dir.join(file_name)).expect("Failed to read generated");
            let reference = fs::read_to_string(Path::new(REFERENCE_DIR).join(file_name)).expect("Failed to read reference");
            let gen_fm = extract_frontmatter(&generated).unwrap();
            let ref_fm = extract_frontmatter(&reference).unwrap();
            let gen_yaml: serde_yaml::Value = serde_yaml::from_str(&gen_fm).expect("Invalid YAML");
            let ref_yaml: serde_yaml::Value = serde_yaml::from_str(&ref_fm).expect("Invalid YAML");
            assert_eq!(gen_yaml, ref_yaml, "Frontmatter mismatch for {}", file_name);
        }
    }

    #[test]
    fn test_import_missing_bpmn_file() {
        detent().arg("import").arg("nonexistent.bpmn").assert().failure()
            .stderr(predicate::str::contains("Failed to read"));
    }

    #[test]
    fn test_import_requires_bpmn_file() {
        detent().arg("import").assert().failure();
    }
}

// ============================================================
// CLI validate command
// ============================================================

mod cli_validate {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[allow(deprecated)]
    fn detent() -> Command {
        Command::cargo_bin("detent").expect("Failed to find detent binary")
    }

    #[test]
    fn test_validate_help() {
        detent().arg("validate").arg("--help").assert().success()
            .stdout(predicate::str::contains("BPMN"));
    }

    #[test]
    fn test_validate_bpmn_file() {
        detent().arg("validate").arg("tests/assets/processes/hello-world/hello-world.bpmn2")
            .assert().success().stdout(predicate::str::contains("✓"));
    }

    #[test]
    fn test_validate_mdx_start_event() {
        detent().arg("validate").arg("tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")
            .assert().success().stdout(predicate::str::contains("✓"));
    }

    #[test]
    fn test_validate_mdx_task() {
        detent().arg("validate").arg("tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")
            .assert().success().stdout(predicate::str::contains("✓"));
    }

    #[test]
    fn test_validate_mdx_end_event() {
        detent().arg("validate").arg("tests/assets/processes/hello-world/_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx")
            .assert().success().stdout(predicate::str::contains("✓"));
    }

    #[test]
    fn test_validate_mdx_sequence_flow() {
        detent().arg("validate").arg("tests/assets/processes/hello-world/_4083739B-66F0-4B92-A348-A37DF3B29083.mdx")
            .assert().success().stdout(predicate::str::contains("✓"));
    }

    #[test]
    fn test_validate_multiple_files() {
        detent().arg("validate")
            .arg("tests/assets/processes/hello-world/hello-world.bpmn2")
            .arg("tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")
            .assert().success().stdout(predicate::str::contains("✓").count(2));
    }

    #[test]
    fn test_validate_missing_file() {
        detent().arg("validate").arg("nonexistent.bpmn").assert().failure()
            .stderr(predicate::str::contains("✗"));
    }

    #[test]
    fn test_validate_unknown_extension() {
        detent().arg("validate")
            .arg("tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")
            .arg("Cargo.toml").assert().failure()
            .stderr(predicate::str::contains("Unknown file type"));
    }

    #[test]
    fn test_validate_requires_files() {
        detent().arg("validate").assert().failure();
    }
}

// ============================================================
// Compile pure logic (MDX → Definitions)
// ============================================================

mod compile {
    use crate::transpiler::compile::{compile_to_definitions, MdxInput};

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
    fn test_compile_roundtrip_with_import() {
        use crate::transpiler::import::import_to_mdx;
        let inputs = simple_mdx_inputs();
        let defs = compile_to_definitions(&inputs).unwrap();
        let outputs = import_to_mdx(&defs).unwrap();
        assert_eq!(outputs.len(), inputs.len());
        for input in &inputs {
            assert!(outputs.iter().any(|o| o.filename == input.filename), "Missing file {}", input.filename);
        }
    }

    #[test]
    fn test_compile_with_hello_world_reference_files() {
        let inputs = vec![
            MdxInput { filename: "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx".to_string(), content: std::fs::read_to_string("tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx").unwrap() },
            MdxInput { filename: "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx".to_string(), content: std::fs::read_to_string("tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx").unwrap() },
            MdxInput { filename: "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx".to_string(), content: std::fs::read_to_string("tests/assets/processes/hello-world/_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx").unwrap() },
            MdxInput { filename: "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx".to_string(), content: std::fs::read_to_string("tests/assets/processes/hello-world/_4083739B-66F0-4B92-A348-A37DF3B29083.mdx").unwrap() },
            MdxInput { filename: "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx".to_string(), content: std::fs::read_to_string("tests/assets/processes/hello-world/_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx").unwrap() },
        ];
        let defs = compile_to_definitions(&inputs).unwrap();
        let process = defs.process.as_ref().unwrap();
        assert_eq!(process.tasks[0].name, Some("Hello World".to_string()));
    }
}

// ============================================================
// Import pure logic (Definitions → MDX)
// ============================================================

mod import {
    use crate::transpiler::bpmn::{Definitions, EndEvent, Process, SequenceFlow, StartEvent, Task};
    use crate::transpiler::import::import_to_mdx;

    fn simple_definitions() -> Definitions {
        Definitions {
            id: "def_1".to_string(), name: Some("Test Process".to_string()),
            target_namespace: None, exporter: None, exporter_version: None,
            process: Some(Process {
                id: "process_1".to_string(), name: Some("Simple Process".to_string()),
                is_executable: Some(true), process_type: None, documentation: None,
                start_events: vec![StartEvent { id: "start_1".to_string(), name: None, outgoing: vec!["flow_1".to_string()], documentation: None }],
                end_events: vec![EndEvent { id: "end_1".to_string(), name: None, incoming: vec!["flow_2".to_string()], documentation: None }],
                tasks: vec![Task { id: "task_1".to_string(), name: Some("Do Something".to_string()), incoming: vec!["flow_1".to_string()], outgoing: vec!["flow_2".to_string()], documentation: None }],
                service_tasks: vec![], script_tasks: vec![], exclusive_gateways: vec![], parallel_gateways: vec![],
                sequence_flows: vec![
                    SequenceFlow { id: "flow_1".to_string(), name: None, source_ref: "start_1".to_string(), target_ref: "task_1".to_string(), condition_expression: None, documentation: None },
                    SequenceFlow { id: "flow_2".to_string(), name: None, source_ref: "task_1".to_string(), target_ref: "end_1".to_string(), condition_expression: None, documentation: None },
                ],
            }),
            bpmn_diagram: None,
        }
    }

    #[test]
    fn test_import_returns_one_mdx_per_flow_element() {
        assert_eq!(import_to_mdx(&simple_definitions()).unwrap().len(), 5);
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
    fn test_import_mdx_output_contains_frontmatter_delimiters() {
        for output in &import_to_mdx(&simple_definitions()).unwrap() {
            assert!(output.content.starts_with("---\n"), "MDX for {} should start with ---", output.filename);
            assert!(output.content.contains("\n---\n"), "MDX for {} should have closing ---", output.filename);
        }
    }

    #[test]
    fn test_import_mdx_output_contains_bpmn_type() {
        let outputs = import_to_mdx(&simple_definitions()).unwrap();
        assert!(outputs.iter().find(|o| o.filename == "start_1.mdx").unwrap().content.contains("type: bpmn:startEvent"));
        assert!(outputs.iter().find(|o| o.filename == "end_1.mdx").unwrap().content.contains("type: bpmn:endEvent"));
        assert!(outputs.iter().find(|o| o.filename == "task_1.mdx").unwrap().content.contains("type: bpmn:task"));
        assert!(outputs.iter().find(|o| o.filename == "flow_1.mdx").unwrap().content.contains("type: bpmn:sequenceFlow"));
    }

    #[test]
    fn test_import_errors_on_no_process() {
        let defs = Definitions { id: "def_1".to_string(), name: None, target_namespace: None, exporter: None, exporter_version: None, process: None, bpmn_diagram: None };
        assert!(import_to_mdx(&defs).is_err());
    }

    #[test]
    fn test_import_frontmatter_has_no_at_or_dollar_prefixes() {
        for output in &import_to_mdx(&simple_definitions()).unwrap() {
            assert!(!output.content.contains("'@"), "MDX for {} should not contain '@' keys", output.filename);
            assert!(!output.content.contains("\"@"), "MDX for {} should not contain \"@\" keys", output.filename);
            assert!(!output.content.contains("$text"), "MDX for {} should not contain $text keys", output.filename);
        }
    }
}

// ============================================================
// MDX types (parsing, round-trip)
// ============================================================

mod mdx_types {
    use crate::transpiler::bpmn::{SequenceFlow, StartEvent, Task};
    use crate::transpiler::mdx::MdxFile;
    use std::fs;

    const MDX_START_EVENT: &str = "tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx";
    const MDX_END_EVENT: &str = "tests/assets/processes/hello-world/_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx";
    const MDX_TASK: &str = "tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx";
    const MDX_SEQUENCE_FLOW_1: &str = "tests/assets/processes/hello-world/_4083739B-66F0-4B92-A348-A37DF3B29083.mdx";
    const MDX_SEQUENCE_FLOW_2: &str = "tests/assets/processes/hello-world/_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx";

    #[test]
    fn test_parse_mdx_file() {
        let content = fs::read_to_string(MDX_START_EVENT).expect("Failed to read MDX");
        let mdx = MdxFile::parse(&content).expect("Failed to parse MDX");
        assert!(mdx.frontmatter.contains("id: _1E892844-423C-464F-ADC4-22F1EC73851B"));
        assert!(mdx.body.contains("Start Event"));
    }

    #[test]
    fn test_parse_start_event() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_START_EVENT).unwrap()).unwrap();
        let event = mdx.parse_start_event().unwrap();
        assert_eq!(event.id, "_1E892844-423C-464F-ADC4-22F1EC73851B");
    }

    #[test]
    fn test_parse_end_event() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_END_EVENT).unwrap()).unwrap();
        let event = mdx.parse_end_event().unwrap();
        assert_eq!(event.id, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
    }

    #[test]
    fn test_parse_task() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_TASK).unwrap()).unwrap();
        let task = mdx.parse_task().unwrap();
        assert_eq!(task.id, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
        assert_eq!(task.name, Some("Hello World".to_string()));
    }

    #[test]
    fn test_parse_task_with_documentation() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_TASK).unwrap()).unwrap();
        let task = mdx.parse_task().unwrap();
        assert!(task.documentation.is_some());
        assert_eq!(task.documentation.unwrap().text, "T");
    }

    #[test]
    fn test_parse_sequence_flow_start_to_task() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_SEQUENCE_FLOW_1).unwrap()).unwrap();
        let flow = mdx.parse_sequence_flow().unwrap();
        assert_eq!(flow.source_ref, "_1E892844-423C-464F-ADC4-22F1EC73851B");
        assert_eq!(flow.target_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
    }

    #[test]
    fn test_parse_sequence_flow_task_to_end() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_SEQUENCE_FLOW_2).unwrap()).unwrap();
        let flow = mdx.parse_sequence_flow().unwrap();
        assert_eq!(flow.source_ref, "_808AA40C-EAA1-40C4-A2DC-27000FBF1866");
        assert_eq!(flow.target_ref, "_D3F6E97D-7783-492C-98CE-57EC815D304C");
    }

    #[test]
    fn test_roundtrip_start_event() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_START_EVENT).unwrap()).unwrap();
        let event = mdx.parse_start_event().unwrap();
        let yaml = serde_yaml::to_string(&event).unwrap();
        let parsed: StartEvent = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(event, parsed);
    }

    #[test]
    fn test_roundtrip_task() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_TASK).unwrap()).unwrap();
        let task = mdx.parse_task().unwrap();
        let yaml = serde_yaml::to_string(&task).unwrap();
        let parsed: Task = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(task, parsed);
    }

    #[test]
    fn test_roundtrip_sequence_flow() {
        let mdx = MdxFile::parse(&fs::read_to_string(MDX_SEQUENCE_FLOW_1).unwrap()).unwrap();
        let flow = mdx.parse_sequence_flow().unwrap();
        let yaml = serde_yaml::to_string(&flow).unwrap();
        let parsed: SequenceFlow = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(flow, parsed);
    }

    #[test]
    fn test_all_mdx_files_parseable() {
        for path in [MDX_START_EVENT, MDX_END_EVENT, MDX_TASK, MDX_SEQUENCE_FLOW_1, MDX_SEQUENCE_FLOW_2] {
            let content = fs::read_to_string(path).expect(&format!("Failed to read {}", path));
            let mdx = MdxFile::parse(&content).expect(&format!("Failed to parse {}", path));
            assert!(mdx.frontmatter.contains("type:"));
            assert!(mdx.frontmatter.contains("id:"));
        }
    }
}
