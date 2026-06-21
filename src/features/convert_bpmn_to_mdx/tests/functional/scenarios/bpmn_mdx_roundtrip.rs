use std::collections::BTreeMap;

use cucumber::{given, then, when};
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn;
use detent::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;
use detent::features::convert_bpmn_to_mdx::use_cases::compile::{
    compile_to_definitions, MdxInput,
};
use detent::features::convert_bpmn_to_mdx::use_cases::import::{
    import_to_mdx, MdxOutput,
};

use super::ConvertWorld;

fn frontmatter_by_filename(outputs: &[MdxOutput]) -> BTreeMap<String, serde_yaml::Value> {
    outputs
        .iter()
        .map(|output| {
            let mdx = MdxFile::parse(&output.content).expect("MDX output must parse");
            let frontmatter = serde_yaml::from_str(&mdx.frontmatter).expect("frontmatter YAML");
            (output.filename.clone(), frontmatter)
        })
        .collect()
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

#[then("tdd import outputs include rich process metadata")]
fn then_tdd_metadata(world: &mut ConvertWorld) {
    let outputs = frontmatter_by_filename(
        world.import_outputs.as_ref().expect("import outputs"),
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
        .get(&serde_yaml::Value::String("type".to_string()))
        .expect("type key");
    assert_eq!(xsi_val.as_str().unwrap(), "bpmn:tFormalExpression");
    assert!(yes_flow.get("@xsi:type").is_none());
}

#[when("I import the fixture and compile-roundtrip its frontmatter")]
fn when_roundtrip(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    let definitions = parse_bpmn(xml).expect("fixture BPMN must parse");

    let first_outputs = import_to_mdx(&definitions).expect("fixture BPMN must import to MDX");
    let first_fm = frontmatter_by_filename(&first_outputs);

    let inputs: Vec<MdxInput> = first_outputs
        .iter()
        .map(|o| MdxInput {
            filename: o.filename.clone(),
            content: o.content.clone(),
        })
        .collect();
    let compiled = compile_to_definitions(&inputs).expect("imported MDX must compile back");
    let second_outputs = import_to_mdx(&compiled).expect("compiled definitions must import");
    let second_fm = frontmatter_by_filename(&second_outputs);

    world.e2e_file_content = Some(format!("{:?}|{:?}", first_fm, second_fm));
}

#[then("imported frontmatter matches after compile roundtrip")]
fn then_roundtrip_stable(world: &mut ConvertWorld) {
    let content = world.e2e_file_content.as_ref().expect("roundtrip result");
    let (left, right) = content.split_once('|').expect("roundtrip pair");
    assert_eq!(left, right);
}
