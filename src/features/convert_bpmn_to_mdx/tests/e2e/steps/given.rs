use std::fs;

use cucumber::given;

use super::super::super::ConvertWorld;

#[given("a fresh temp workspace")]
fn given_temp(world: &mut ConvertWorld) {
    let temp = tempfile::tempdir().expect("temp dir");
    world.e2e_work_dir = Some(temp.path().to_path_buf());
    world.e2e_temp = Some(temp);
}

#[given("a minimal workflow in the temp workspace")]
fn given_minimal_workflow(world: &mut ConvertWorld) {
    let dir = world.e2e_work_dir.as_ref().expect("temp workspace");
    fs::create_dir_all(dir).expect("create dir");
    fs::write(
        dir.join("start_1.mdx"),
        "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n",
    )
    .expect("write start");
    fs::write(
        dir.join("end_1.mdx"),
        "---\ntype: bpmn:endEvent\nid: end_1\nincoming:\n- flow_1\n---\n",
    )
    .expect("write end");
    fs::write(
        dir.join("flow_1.mdx"),
        "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: end_1\n---\n",
    )
    .expect("write flow");
}

#[given(regex = r#"^a minimal workflow in subdirectory "([^"]+)"$"#)]
fn given_named_workflow(world: &mut ConvertWorld, name: String) {
    let base = world.e2e_work_dir.as_ref().expect("temp workspace");
    let dir = base.join(&name);
    fs::create_dir_all(&dir).expect("create dir");
    fs::write(
        dir.join("start_1.mdx"),
        "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n",
    )
    .expect("write start");
    fs::write(
        dir.join("end_1.mdx"),
        "---\ntype: bpmn:endEvent\nid: end_1\nincoming:\n- flow_1\n---\n",
    )
    .expect("write end");
    fs::write(
        dir.join("flow_1.mdx"),
        "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: end_1\n---\n",
    )
    .expect("write flow");
}

#[given(regex = r#"^an output file path "([^"]+)" in the temp workspace$"#)]
fn given_output_path(world: &mut ConvertWorld, name: String) {
    let base = world.e2e_work_dir.as_ref().expect("temp workspace");
    world.e2e_output_file = Some(base.join(name));
}

#[given(regex = r#"^a non-MDX file "([^"]+)" in the temp workspace$"#)]
fn given_non_mdx(world: &mut ConvertWorld, name: String) {
    let base = world.e2e_work_dir.as_ref().expect("temp workspace");
    fs::write(base.join(&name), "not mdx content").expect("write file");
    world.e2e_output_file = Some(base.join(name));
}

#[given(regex = r#"^hello-world MDX file path for "([^"]+)"$"#)]
fn given_mdx_path(world: &mut ConvertWorld, name: String) {
    world.e2e_output_file = Some(super::super::super::hello_world_asset_path(&name));
}
