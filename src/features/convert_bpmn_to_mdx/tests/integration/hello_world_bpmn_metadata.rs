use cucumber::{then, when};

use super::super::ConvertWorld;

#[when("I parse the BPMN to definitions")]
fn when_parse(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    let defs = detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(xml)
        .expect("parse_bpmn failed");
    world.parsed_defs = Some(defs);
}

#[then(r#"the process id is "hello_world""#)]
fn then_process_id(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    let process = defs.process.as_ref().expect("expected a process");
    assert_eq!(process.id, "hello_world");
}

#[then(r#"the process name is "hello-world""#)]
fn then_process_name(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    let process = defs.process.as_ref().expect("expected a process");
    assert_eq!(process.name, Some("hello-world".to_string()));
}

#[then("the process is executable and public")]
fn then_process_type(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    let process = defs.process.as_ref().expect("expected a process");
    assert_eq!(process.is_executable, Some(true));
    assert_eq!(process.process_type, Some("Public".to_string()));
}

#[then(r#"the process documentation is "This is a hello world activity""#)]
fn then_process_doc(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    let process = defs.process.as_ref().expect("expected a process");
    assert!(process.documentation.is_some());
    assert_eq!(
        process.documentation.as_ref().unwrap().text,
        "This is a hello world activity"
    );
}

#[then(r#"the definitions were exported by "jBPM Process Modeler" version "2.0""#)]
fn then_definitions_exporter(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    assert_eq!(defs.exporter, Some("jBPM Process Modeler".to_string()));
    assert_eq!(defs.exporter_version, Some("2.0".to_string()));
}

#[then(r#"the definitions target namespace is "http://www.omg.org/bpmn20""#)]
fn then_definitions_target_namespace(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    assert_eq!(
        defs.target_namespace,
        Some("http://www.omg.org/bpmn20".to_string())
    );
}
