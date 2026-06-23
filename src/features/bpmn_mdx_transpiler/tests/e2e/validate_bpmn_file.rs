use cucumber::given;

use super::super::ConvertWorld;

#[given("hello-world BPMN file path")]
fn given_bpmn_path(world: &mut ConvertWorld) {
    world.e2e_output_file = Some(super::super::hello_world_asset_path("hello-world.bpmn2"));
}
