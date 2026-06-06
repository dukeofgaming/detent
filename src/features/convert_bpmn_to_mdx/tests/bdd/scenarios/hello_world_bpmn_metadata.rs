use cucumber::{then, when};

use super::ConvertWorld;

#[when("I parse the BPMN to definitions")]
fn when_parse(world: &mut ConvertWorld) {
    let xml = world.bpmn_xml.as_ref().expect("bpmn_xml must be set");
    let defs = detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn(xml)
        .expect("parse_bpmn failed");
    world.parsed_defs = Some(defs);
}

#[then("the process id is \"hello_world\"")]
fn then_process_id(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    let process = defs.process.as_ref().expect("expected a process");
    assert_eq!(process.id, "hello_world");
}

#[then("the process name is \"hello-world\"")]
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

#[then("the process documentation is \"This is a hello world activity\"")]
fn then_process_doc(world: &mut ConvertWorld) {
    let defs = world.parsed_defs.as_ref().expect("expected parsed defs");
    let process = defs.process.as_ref().expect("expected a process");
    assert!(process.documentation.is_some());
    assert_eq!(
        process.documentation.as_ref().unwrap().text,
        "This is a hello world activity"
    );
}
