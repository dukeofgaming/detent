use std::fs;

use cucumber::{given, then, when};
use std::process::ExitCode;

use super::cli_common::run_detent;
use super::super::ConvertWorld;

fn write_temp_mdx(world: &mut ConvertWorld, name: &str, content: &str) {
    let base = world.e2e_work_dir.as_ref().expect("temp workspace");
    let path = base.join(name);
    fs::write(&path, content).expect("write mdx");
    world.e2e_output_file = Some(path);
}

#[given(regex = r#"^hello-world MDX file path for "([^"]+)"$"#)]
fn given_mdx_path(world: &mut ConvertWorld, name: String) {
    world.e2e_output_file = Some(super::super::hello_world_asset_path(&name));
}

#[given("a temp exclusive gateway MDX file")]
fn given_exclusive_gateway(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "gateway_1.mdx",
        "---\ntype: bpmn:exclusiveGateway\nid: gateway_1\ngatewayDirection: Diverging\nincoming:\n- flow_in\noutgoing:\n- flow_a\n---\n",
    );
}

#[given("a temp parallel gateway MDX file")]
fn given_parallel_gateway(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "gateway_1.mdx",
        "---\ntype: bpmn:parallelGateway\nid: gateway_1\ngatewayDirection: Diverging\nincoming:\n- flow_in\noutgoing:\n- flow_a\n- flow_b\n---\n",
    );
}

#[given("a temp service task MDX file")]
fn given_service_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "svc_1.mdx",
        "---\ntype: bpmn:serviceTask\nid: svc_1\nname: Call Service\nimplementation: Java\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}

#[given("a temp script task MDX file")]
fn given_script_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "script_1.mdx",
        "---\ntype: bpmn:scriptTask\nid: script_1\nname: Run Script\nscriptFormat: javascript\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}

#[given("a temp manual task MDX file")]
fn given_manual_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "manual_1.mdx",
        "---\ntype: bpmn:manualTask\nid: manual_1\nname: Manual Step\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}

#[given("a temp user task MDX file")]
fn given_user_task(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "user_1.mdx",
        "---\ntype: bpmn:userTask\nid: user_1\nname: Review\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );
}

#[given("a temp gateway MDX file without id")]
fn given_gateway_without_id(world: &mut ConvertWorld) {
    write_temp_mdx(
        world,
        "gateway_bad.mdx",
        "---\ntype: bpmn:exclusiveGateway\nid: \"\"\ngatewayDirection: Diverging\n---\n",
    );
}

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

#[when("I validate the prepared file with detent")]
fn when_validate_file(world: &mut ConvertWorld) {
    let path = world
        .e2e_output_file
        .as_ref()
        .expect("file path")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["validate", &path]);
}

#[when("I validate hello-world BPMN and start event MDX")]
fn when_validate_two(world: &mut ConvertWorld) {
    let bpmn = super::super::hello_world_asset_path("hello-world.bpmn2")
        .to_string_lossy()
        .into_owned();
    let mdx = super::super::hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["validate", &bpmn, &mdx]);
}

#[when("I validate hello-world task MDX and Cargo.toml")]
fn when_validate_unknown_ext(world: &mut ConvertWorld) {
    let mdx = super::super::hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")
        .to_string_lossy()
        .into_owned();
    let cargo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Cargo.toml")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["validate", &mdx, &cargo]);
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
