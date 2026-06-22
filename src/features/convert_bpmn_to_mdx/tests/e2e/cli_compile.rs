use cucumber::{given, when};

use super::cli_common::run_detent;
use super::super::ConvertWorld;

#[when("I compile hello-world MDX to the output file")]
fn when_compile_hello_world(world: &mut ConvertWorld) {
    let out = world
        .e2e_output_file
        .as_ref()
        .expect("output file")
        .clone();
    run_detent(
        world,
        &[
            "compile",
            &super::super::hello_world_asset_dir().to_string_lossy(),
            "--output",
            &out.to_string_lossy(),
        ],
    );
}

#[when("I compile hello-world MDX files individually to the output file")]
fn when_compile_individual(world: &mut ConvertWorld) {
    let out = world
        .e2e_output_file
        .as_ref()
        .expect("output file")
        .clone();
    let dir = super::super::hello_world_asset_dir();
    let mut args = vec!["compile".to_string()];
    for entry in std::fs::read_dir(&dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("mdx") {
            args.push(path.to_string_lossy().to_string());
        }
    }
    args.push("--output".to_string());
    args.push(out.to_string_lossy().to_string());
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_detent(world, &arg_refs);
}

#[when("I compile all MDX files in the temp workspace to stdout")]
fn when_compile_stdout(world: &mut ConvertWorld) {
    let dir = world.e2e_work_dir.as_ref().expect("temp workspace");
    let mut args = vec!["compile".to_string()];
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("mdx") {
            args.push(path.to_string_lossy().to_string());
        }
    }
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_detent(world, &arg_refs);
}

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
    use std::process::ExitCode;

    let output = world.e2e_output_file.as_ref().expect("output").clone();
    let input = world.e2e_work_dir.as_ref().expect("input").clone();
    let status = detent::features::convert_bpmn_to_mdx::infrastructure::cli::compile::run(
        vec![input],
        Some(output.clone()),
    );
    world.e2e_last_success = status == ExitCode::SUCCESS;
    world.e2e_file_content = std::fs::read_to_string(output).ok();
}
