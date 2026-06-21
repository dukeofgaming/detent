use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn;
use detent::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;
use detent::features::convert_bpmn_to_mdx::use_cases::compile::{
    compile_to_definitions, MdxInput,
};
use detent::features::convert_bpmn_to_mdx::use_cases::import::{
    import_to_mdx, MdxOutput,
};

fn fixture_path(relative_path: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/features/convert_bpmn_to_mdx/tests/assets")
        .join(relative_path)
        .to_string_lossy()
        .to_string()
}

fn import_fixture(relative_path: &str) -> Vec<MdxOutput> {
    let xml = fs::read_to_string(fixture_path(relative_path)).expect("fixture must be readable");
    let definitions = parse_bpmn(&xml).expect("fixture BPMN must parse");
    import_to_mdx(&definitions).expect("fixture BPMN must import to MDX")
}

fn frontmatter_by_filename(outputs: Vec<MdxOutput>) -> BTreeMap<String, serde_yaml::Value> {
    outputs
        .into_iter()
        .map(|output| {
            let mdx = MdxFile::parse(&output.content).expect("MDX output must parse");
            let frontmatter = serde_yaml::from_str(&mdx.frontmatter).expect("frontmatter YAML");
            (output.filename, frontmatter)
        })
        .collect()
}

fn compile_outputs(outputs: &[MdxOutput]) -> Vec<MdxOutput> {
    let inputs: Vec<MdxInput> = outputs
        .iter()
        .map(|output| MdxInput {
            filename: output.filename.clone(),
            content: output.content.clone(),
        })
        .collect();
    let definitions = compile_to_definitions(&inputs).expect("imported MDX must compile");
    import_to_mdx(&definitions).expect("compiled definitions must import")
}

fn string_at<'a>(value: &'a serde_yaml::Value, keys: &[&str]) -> &'a str {
    let mut current = value;
    for key in keys {
        current = &current[*key];
    }
    current.as_str().expect("expected string value")
}

fn bool_at(value: &serde_yaml::Value, keys: &[&str]) -> bool {
    let mut current = value;
    for key in keys {
        current = &current[*key];
    }
    current.as_bool().expect("expected bool value")
}

#[test]
fn process_metadata_mdx_controls_compiled_definitions() {
    let process_mdx = r#"---
type: bpmn:process
id: Process_DeveloperWorkflow
name: TDD Developer Workflow
isExecutable: true
isClosed: false
processType: None
definitions:
  id: bpmn_copilot_Definition_id
  targetNamespace: http://bpmn.io/schema/bpmn
  exporter: Camunda Web Modeler
  exporterVersion: f0f3e8d
diagram:
  id: BPMNDiagram_Process_DeveloperWorkflow
  plane:
    id: BPMNPlane_Process_DeveloperWorkflow
    bpmnElement: Process_DeveloperWorkflow
---
"#;
    let start_event_mdx = r#"---
type: bpmn:startEvent
id: StartEvent_NewFeature
outgoing:
- Flow_1q9w0c5
---
"#;
    let end_event_mdx = r#"---
type: bpmn:endEvent
id: EndEvent_Done
incoming:
- Flow_1q9w0c5
---
"#;
    let flow_mdx = r#"---
type: bpmn:sequenceFlow
id: Flow_1q9w0c5
sourceRef: StartEvent_NewFeature
targetRef: EndEvent_Done
---
"#;

    let definitions = compile_to_definitions(&[
        MdxInput {
            filename: "Process_DeveloperWorkflow.mdx".to_string(),
            content: process_mdx.to_string(),
        },
        MdxInput {
            filename: "StartEvent_NewFeature.mdx".to_string(),
            content: start_event_mdx.to_string(),
        },
        MdxInput {
            filename: "EndEvent_Done.mdx".to_string(),
            content: end_event_mdx.to_string(),
        },
        MdxInput {
            filename: "Flow_1q9w0c5.mdx".to_string(),
            content: flow_mdx.to_string(),
        },
    ])
    .expect("process metadata should compile");

    let process = definitions.process.as_ref().expect("process should compile");
    let diagram = definitions
        .bpmn_diagram
        .as_ref()
        .expect("diagram metadata should compile");

    assert_eq!(definitions.id, "bpmn_copilot_Definition_id");
    assert_eq!(
        definitions.target_namespace.as_deref(),
        Some("http://bpmn.io/schema/bpmn")
    );
    assert_eq!(definitions.exporter.as_deref(), Some("Camunda Web Modeler"));
    assert_eq!(definitions.exporter_version.as_deref(), Some("f0f3e8d"));
    assert_eq!(process.id, "Process_DeveloperWorkflow");
    assert_eq!(process.name.as_deref(), Some("TDD Developer Workflow"));
    assert_eq!(process.is_executable, Some(true));
    assert_eq!(process.is_closed, Some(false));
    assert_eq!(process.process_type.as_deref(), Some("None"));
    assert_eq!(diagram.id, "BPMNDiagram_Process_DeveloperWorkflow");
    assert_eq!(diagram.plane.id, "BPMNPlane_Process_DeveloperWorkflow");
    assert_eq!(diagram.plane.bpmn_element, "Process_DeveloperWorkflow");
}

#[test]
fn imports_include_process_and_definitions_metadata() {
    let outputs = frontmatter_by_filename(import_fixture("tdd/tdd.bpmn2"));
    let process = outputs
        .get("Process_DeveloperWorkflow.mdx")
        .expect("process metadata file should be imported");

    assert_eq!(string_at(process, &["type"]), "bpmn:process");
    assert_eq!(string_at(process, &["id"]), "Process_DeveloperWorkflow");
    assert_eq!(string_at(process, &["name"]), "TDD Developer Workflow");
    assert_eq!(string_at(process, &["processType"]), "None");
    assert!(!bool_at(process, &["isClosed"]));
    assert!(bool_at(process, &["isExecutable"]));
    assert_eq!(
        string_at(process, &["definitions", "id"]),
        "bpmn_copilot_Definition_id"
    );
    assert_eq!(
        string_at(process, &["definitions", "targetNamespace"]),
        "http://bpmn.io/schema/bpmn"
    );
    assert_eq!(
        string_at(process, &["definitions", "exporter"]),
        "Camunda Web Modeler"
    );
    assert_eq!(
        string_at(process, &["definitions", "exporterVersion"]),
        "f0f3e8d"
    );
    assert_eq!(
        string_at(process, &["diagram", "id"]),
        "BPMNDiagram_Process_DeveloperWorkflow"
    );
    assert_eq!(
        string_at(process, &["diagram", "plane", "id"]),
        "BPMNPlane_Process_DeveloperWorkflow"
    );
    assert_eq!(
        string_at(process, &["diagram", "plane", "bpmnElement"]),
        "Process_DeveloperWorkflow"
    );

    let gateway = outputs
        .get("Gateway_TestsPass.mdx")
        .expect("gateway file should be imported");
    assert_eq!(
        string_at(gateway, &["diagram", "id"]),
        "Gateway_TestsPass_di"
    );
    assert!(bool_at(gateway, &["diagram", "isMarkerVisible"]));

    let yes_flow = outputs
        .get("Flow_1w3e4r5.mdx")
        .expect("conditional flow file should be imported");
    let ce = yes_flow.get("conditionExpression").expect("should have conditionExpression");
    let ce_map = ce.as_mapping().expect("conditionExpression should be mapping");
    let xsi_val = ce_map.get(&serde_yaml::Value::String("type".to_string()))
        .expect("expected type key in conditionExpression");
    assert_eq!(xsi_val.as_str().unwrap(), "bpmn:tFormalExpression");
    assert!(
        yes_flow.get("@xsi:type").is_none(),
        "condition expression should not expose XML attribute keys"
    );
}

#[test]
fn imported_fixture_frontmatter_is_stable_after_compile_roundtrip() {
    for fixture in [
        "tdd/tdd.bpmn2",
        "blog_post/blog-post.bpmn2",
        "hello_world/hello-world.bpmn2",
    ] {
        let imported = import_fixture(fixture);
        let roundtripped = compile_outputs(&imported);

        assert_eq!(
            frontmatter_by_filename(imported),
            frontmatter_by_filename(roundtripped),
            "frontmatter roundtrip changed for {fixture}"
        );
    }
}
