use predicates::prelude::*;
use std::fs;

use super::hello_world_asset_path;

#[test]
fn test_validate_help() {
    detent!()
        .arg("validate")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("BPMN"));
}

#[test]
fn test_validate_bpmn_file() {
    detent!()
        .arg("validate")
        .arg(hello_world_asset_path("hello-world.bpmn2"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_start_event() {
    detent!()
        .arg("validate")
        .arg(hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_task() {
    detent!()
        .arg("validate")
        .arg(hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_end_event() {
    detent!()
        .arg("validate")
        .arg(hello_world_asset_path("_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_sequence_flow() {
    detent!()
        .arg("validate")
        .arg(hello_world_asset_path("_4083739B-66F0-4B92-A348-A37DF3B29083.mdx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_multiple_files() {
    detent!()
        .arg("validate")
        .arg(hello_world_asset_path("hello-world.bpmn2"))
        .arg(hello_world_asset_path("_1E892844-423C-464F-ADC4-22F1EC73851B.mdx"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓").count(2));
}

#[test]
fn test_validate_missing_file() {
    detent!()
        .arg("validate")
        .arg("nonexistent.bpmn")
        .assert()
        .failure()
        .stderr(predicate::str::contains("✗"));
}

#[test]
fn test_validate_unknown_extension() {
    detent!()
        .arg("validate")
        .arg(hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx"))
        .arg("Cargo.toml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown file type"));
}

#[test]
fn test_validate_requires_files() {
    detent!().arg("validate").assert().failure();
}

#[test]
fn test_validate_tolerates_dangling_flow_target() {
    use std::process::ExitCode;

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let invalid_bpmn = temp_dir.path().join("invalid-graph.bpmn2");
    let fixture = fs::read_to_string(hello_world_asset_path("hello-world.bpmn2"))
        .expect("Failed to read fixture");
    let mutated = fixture.replace(
        "targetRef=\"_808AA40C-EAA1-40C4-A2DC-27000FBF1866\"",
        "targetRef=\"ghost_task\"",
    );
    fs::write(&invalid_bpmn, mutated).expect("Failed to write invalid BPMN");

    let status =
        detent::features::convert_bpmn_to_mdx::infrastructure::cli::validate::run(vec![invalid_bpmn]);
    assert_eq!(status, ExitCode::SUCCESS);
}

fn write_temp_mdx(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("Failed to write temp MDX");
    path
}

#[test]
fn test_validate_mdx_exclusive_gateway() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let path = write_temp_mdx(
        temp_dir.path(),
        "gateway_1.mdx",
        "---\ntype: bpmn:exclusiveGateway\nid: gateway_1\ngatewayDirection: Diverging\nincoming:\n- flow_in\noutgoing:\n- flow_a\n---\n",
    );

    detent!()
        .arg("validate")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_parallel_gateway() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let path = write_temp_mdx(
        temp_dir.path(),
        "gateway_1.mdx",
        "---\ntype: bpmn:parallelGateway\nid: gateway_1\ngatewayDirection: Diverging\nincoming:\n- flow_in\noutgoing:\n- flow_a\n- flow_b\n---\n",
    );

    detent!()
        .arg("validate")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_service_task() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let path = write_temp_mdx(
        temp_dir.path(),
        "svc_1.mdx",
        "---\ntype: bpmn:serviceTask\nid: svc_1\nname: Call Service\nimplementation: Java\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );

    detent!()
        .arg("validate")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_script_task() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let path = write_temp_mdx(
        temp_dir.path(),
        "script_1.mdx",
        "---\ntype: bpmn:scriptTask\nid: script_1\nname: Run Script\nscriptFormat: javascript\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );

    detent!()
        .arg("validate")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_manual_task() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let path = write_temp_mdx(
        temp_dir.path(),
        "manual_1.mdx",
        "---\ntype: bpmn:manualTask\nid: manual_1\nname: Manual Step\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );

    detent!()
        .arg("validate")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_user_task() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let path = write_temp_mdx(
        temp_dir.path(),
        "user_1.mdx",
        "---\ntype: bpmn:userTask\nid: user_1\nname: Review\nincoming:\n- flow_in\noutgoing:\n- flow_out\n---\n",
    );

    detent!()
        .arg("validate")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_rejects_gateway_without_id() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let path = write_temp_mdx(
        temp_dir.path(),
        "gateway_bad.mdx",
        "---\ntype: bpmn:exclusiveGateway\nid: \"\"\ngatewayDirection: Diverging\n---\n",
    );

    detent!()
        .arg("validate")
        .arg(&path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("ExclusiveGateway must have an id"));
}
