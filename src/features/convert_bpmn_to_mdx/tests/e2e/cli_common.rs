use std::fs;

use cucumber::{given, then, when};
use predicates::prelude::*;

use super::ConvertWorld;

macro_rules! detent {
    () => {{
        #[allow(deprecated)]
        assert_cmd::Command::cargo_bin("detent").expect("Failed to find detent binary")
    }};
}

pub(crate) fn run_detent(world: &mut ConvertWorld, args: &[&str]) {
    let mut cmd = detent!();
    if let Some(dir) = &world.e2e_work_dir {
        cmd.current_dir(dir);
    }
    for arg in args {
        if arg.starts_with('@') {
            cmd.arg(&world.e2e_output_file.as_ref().expect("output file path"));
        } else if arg.starts_with('#') {
            cmd.arg(world.e2e_output_file.as_ref().expect("output file path"));
        } else {
            cmd.arg(arg);
        }
    }
    let output = cmd.output().expect("detent command failed");
    world.e2e_last_success = output.status.success();
    world.e2e_last_stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    world.e2e_last_stderr = String::from_utf8_lossy(&output.stderr).into_owned();
}

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

#[when(regex = r#"^I run detent with args "(.*)"$"#)]
fn when_run_detent(world: &mut ConvertWorld, args: String) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    run_detent(world, &parts);
}

#[then("the command succeeds")]
fn then_success(world: &mut ConvertWorld) {
    assert!(
        world.e2e_last_success,
        "expected success; stderr={} stdout={}",
        world.e2e_last_stderr, world.e2e_last_stdout
    );
}

#[then("the command fails")]
fn then_failure(world: &mut ConvertWorld) {
    assert!(
        !world.e2e_last_success,
        "expected failure; stdout={}",
        world.e2e_last_stdout
    );
}

#[then(regex = r#"^stdout contains "(.*)"$"#)]
fn then_stdout_contains(world: &mut ConvertWorld, snippet: String) {
    assert!(
        predicate::str::contains(&snippet).eval(&world.e2e_last_stdout),
        "stdout {:?} missing {:?}",
        world.e2e_last_stdout,
        snippet
    );
}

#[then(regex = r#"^stderr contains "(.*)"$"#)]
fn then_stderr_contains(world: &mut ConvertWorld, snippet: String) {
    assert!(
        predicate::str::contains(&snippet).eval(&world.e2e_last_stderr),
        "stderr {:?} missing {:?}",
        world.e2e_last_stderr,
        snippet
    );
}

#[then(regex = r#"^the file "([^"]+)" exists in the temp workspace$"#)]
fn then_file_exists(world: &mut ConvertWorld, name: String) {
    let path = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .join(name);
    assert!(path.exists(), "expected {} to exist", path.display());
}

#[then(regex = r#"^the file "([^"]+)" does not exist in the temp workspace$"#)]
fn then_file_missing(world: &mut ConvertWorld, name: String) {
    let path = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .join(name);
    assert!(!path.exists(), "expected {} to be absent", path.display());
}

#[then(regex = r#"^the file "([^"]+)" contains "(.*)"$"#)]
fn then_file_contains(world: &mut ConvertWorld, name: String, snippet: String) {
    let path = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .join(name);
    let content = fs::read_to_string(&path).expect("read file");
    assert!(content.contains(&snippet), "missing {snippet} in {content}");
}

#[then("the dangling target id appears in compiled output")]
fn then_dangling_target(world: &mut ConvertWorld) {
    let content = world.e2e_file_content.as_ref().expect("output content");
    assert!(content.contains("ghost_task"));
}

#[then(regex = r#"^stdout contains "✓" (\d+) times$"#)]
fn then_checkmark_count(world: &mut ConvertWorld, count: usize) {
    assert_eq!(
        world.e2e_last_stdout.matches('✓').count(),
        count,
        "stdout={}",
        world.e2e_last_stdout
    );
}
