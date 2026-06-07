use std::fs;

use cucumber::given;

use super::super::ConvertWorld;

#[given("the hello-world BPMN fixture")]
fn given_hello_world(world: &mut ConvertWorld) {
    let xml = fs::read_to_string(super::super::hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read hello-world.bpmn2");
    world.bpmn_xml = Some(xml);
}
