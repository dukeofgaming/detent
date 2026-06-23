use cucumber::{given, when};
use std::process::ExitCode;

use super::super::ConvertWorld;

#[given("a dangling-flow compile workspace")]
fn given_dangling_compile(world: &mut ConvertWorld) {
    let temp = tempfile::tempdir().expect("temp dir");
    let base = temp.path().to_path_buf();
    let input_dir = base.join("input");
    std::fs::create_dir_all(&input_dir).expect("create input");
    for (name, content) in [
        (
            "start_1.mdx",
            "---\ntype: bpmn:startEvent\nid: start_1\noutgoing:\n- flow_1\n---\n",
        ),
        (
            "end_1.mdx",
            "---\ntype: bpmn:endEvent\nid: end_1\nincoming:\n- flow_2\n---\n",
        ),
        (
            "flow_1.mdx",
            "---\ntype: bpmn:sequenceFlow\nid: flow_1\nsourceRef: start_1\ntargetRef: ghost_task\n---\n",
        ),
    ] {
        std::fs::write(input_dir.join(name), content).expect("write mdx");
    }
    world.e2e_temp = Some(temp);
    world.e2e_work_dir = Some(input_dir);
    world.e2e_output_file = Some(base.join("output.bpmn"));
}

#[when("I run the compile CLI directly on the dangling workflow")]
fn when_direct_dangling_compile(world: &mut ConvertWorld) {
    let output = world.e2e_output_file.as_ref().expect("output").clone();
    let input = world.e2e_work_dir.as_ref().expect("input").clone();
    let status = detent::features::convert_bpmn_to_mdx::infrastructure::cli::compile::run(
        vec![input],
        Some(output.clone()),
    );
    world.e2e_last_success = status == ExitCode::SUCCESS;
    world.e2e_file_content = std::fs::read_to_string(output).ok();
}
