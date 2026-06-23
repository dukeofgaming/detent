use std::fs;
use std::process::ExitCode;

use cucumber::{given, then, when};

use super::super::ConvertWorld;

#[given("a dangling-target hello-world BPMN in the temp workspace")]
fn given_dangling_bpmn(world: &mut ConvertWorld) {
    let base = world.e2e_work_dir.as_ref().expect("temp workspace");
    let path = base.join("invalid-graph.bpmn2");
    let fixture = fs::read_to_string(super::super::hello_world_asset_path("hello-world.bpmn2"))
        .expect("read fixture");
    let mutated = fixture.replace(
        "targetRef=\"_808AA40C-EAA1-40C4-A2DC-27000FBF1866\"",
        "targetRef=\"ghost_task\"",
    );
    fs::write(&path, mutated).expect("write bpmn");
    world.e2e_output_file = Some(path);
}

#[when("I run validate directly on the dangling BPMN")]
fn when_direct_validate(world: &mut ConvertWorld) {
    let path = world.e2e_output_file.as_ref().expect("bpmn path").clone();
    let status =
        detent::features::convert_bpmn_to_mdx::infrastructure::cli::validate::run(vec![path]);
    world.e2e_last_success = status == ExitCode::SUCCESS;
}

#[then("direct validate succeeds")]
fn then_direct_validate_ok(world: &mut ConvertWorld) {
    assert!(world.e2e_last_success);
}
