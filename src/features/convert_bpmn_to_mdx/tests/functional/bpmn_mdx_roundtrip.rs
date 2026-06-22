use std::collections::BTreeMap;
use std::fs;

use cucumber::{given, then, when};
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn;
use detent::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;
use detent::features::convert_bpmn_to_mdx::use_cases::compile::{
    compile_to_definitions, MdxInput,
};
use detent::features::convert_bpmn_to_mdx::use_cases::import::{
    import_to_mdx, MdxOutput,
};

use super::super::ConvertWorld;

fn import_fixture(_world: &mut ConvertWorld, relative_path: &str) -> Vec<MdxOutput> {
    // Arrange
    let xml = fs::read_to_string(super::super::fixture_path(relative_path)).expect("fixture must be readable");
    // Act
    let definitions = parse_bpmn(&xml).expect("fixture BPMN must parse");
    // Act
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
    // Arrange
    let inputs: Vec<MdxInput> = outputs
        .iter()
        .map(|output| MdxInput {
            filename: output.filename.clone(),
            content: output.content.clone(),
        })
        .collect();
    // Act
    let definitions = compile_to_definitions(&inputs).expect("imported MDX must compile");
    // Act
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

#[given("process metadata MDX inputs")]
fn given_process_metadata(world: &mut ConvertWorld) {
    world.mdx_inputs = vec![
        MdxInput {
            filename: "Process_DeveloperWorkflow.mdx".to_string(),
            content: r#"---
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
"#
            .to_string(),
        },
        MdxInput {
            filename: "StartEvent_NewFeature.mdx".to_string(),
            content: "---\ntype: bpmn:startEvent\nid: StartEvent_NewFeature\noutgoing:\n- Flow_1q9w0c5\n---\n".to_string(),
        },
        MdxInput {
            filename: "EndEvent_Done.mdx".to_string(),
            content: "---\ntype: bpmn:endEvent\nid: EndEvent_Done\nincoming:\n- Flow_1q9w0c5\n---\n".to_string(),
        },
        MdxInput {
            filename: "Flow_1q9w0c5.mdx".to_string(),
            content: "---\ntype: bpmn:sequenceFlow\nid: Flow_1q9w0c5\nsourceRef: StartEvent_NewFeature\ntargetRef: EndEvent_Done\n---\n".to_string(),
        },
    ];
}

#[when("I compile the process metadata inputs")]
fn when_compile_metadata(world: &mut ConvertWorld) {
    world.compile_result = Some(
        compile_to_definitions(&world.mdx_inputs).expect("process metadata should compile"),
    );
}

#[then("compiled definitions carry process and diagram metadata")]
fn then_metadata(world: &mut ConvertWorld) {
    let definitions = world.compile_result.as_ref().expect("definitions");
    let process = definitions.process.as_ref().expect("process");
    let diagram = definitions.bpmn_diagram.as_ref().expect("diagram");

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

#[given("the tdd BPMN fixture is imported to MDX")]
fn given_tdd_import(world: &mut ConvertWorld) {
    // Act
    world.import_outputs = Some(import_fixture(world, "tdd/tdd.bpmn2"));
}

#[then("tdd import outputs include rich process metadata")]
fn then_tdd_metadata(world: &mut ConvertWorld) {
    let outputs = frontmatter_by_filename(
        world.import_outputs.clone().expect("import outputs"),
    );
    let process = outputs
        .get("Process_DeveloperWorkflow.mdx")
        .expect("process metadata file");

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

    let gateway = outputs.get("Gateway_TestsPass.mdx").expect("gateway file");
    assert_eq!(string_at(gateway, &["diagram", "id"]), "Gateway_TestsPass_di");
    assert!(bool_at(gateway, &["diagram", "isMarkerVisible"]));

    let yes_flow = outputs.get("Flow_1w3e4r5.mdx").expect("flow file");
    let ce = yes_flow.get("conditionExpression").expect("conditionExpression");
    let ce_map = ce.as_mapping().expect("mapping");
    let xsi_val = ce_map
        .get(serde_yaml::Value::String("type".to_string()))
        .expect("type key");
    assert_eq!(xsi_val.as_str().unwrap(), "bpmn:tFormalExpression");
    assert!(yes_flow.get("@xsi:type").is_none());
}

#[given(regex = r#"^the "(.+)" fixture$"#)]
fn given_roundtrip_fixture(world: &mut ConvertWorld, path: String) {
    world.bpmn_xml = Some(path);
}

#[when("I import and compile-roundtrip the fixture")]
fn when_roundtrip(world: &mut ConvertWorld) {
    let relative = world.bpmn_xml.as_ref().expect("fixture path").clone();
    // Act
    let imported = import_fixture(world, &relative);
    // Act
    let roundtripped = compile_outputs(&imported);
    // Arrange
    world.e2e_file_content = Some(format!(
        "{:?}|{:?}",
        frontmatter_by_filename(imported),
        frontmatter_by_filename(roundtripped)
    ));
}

#[then("imported frontmatter matches after compile roundtrip")]
fn then_roundtrip_stable(world: &mut ConvertWorld) {
    let content = world.e2e_file_content.as_ref().expect("roundtrip result");
    let (left, right) = content.split_once('|').expect("roundtrip pair");
    assert_eq!(left, right);
}
