use predicates::prelude::*;
use std::fs;
use std::path::Path;

use super::hello_world_asset_dir;

#[test]
fn test_compile_help() {
    detent!()
        .arg("compile")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("MDX files"));
}

#[test]
fn test_compile_requires_path() {
    detent!().arg("compile").assert().failure();
}

#[test]
fn test_compile_missing_directory() {
    detent!()
        .arg("compile")
        .arg("nonexistent-dir")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read"));
}

#[test]
fn test_compile_produces_bpmn_xml_file() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("output.bpmn");
    detent!()
        .arg("compile")
        .arg(hello_world_asset_dir())
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();
    assert!(output_file.exists(), "Output BPMN file should exist");
    let content = fs::read_to_string(&output_file).expect("Failed to read output");
    assert!(content.contains("definitions"));
    assert!(content.contains("process"));
}

#[test]
fn test_compile_output_contains_all_elements() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("output.bpmn");
    detent!()
        .arg("compile")
        .arg(hello_world_asset_dir())
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();
    let content = fs::read_to_string(&output_file).expect("Failed to read output");
    assert!(content.contains("_1E892844-423C-464F-ADC4-22F1EC73851B"));
    assert!(content.contains("_808AA40C-EAA1-40C4-A2DC-27000FBF1866"));
    assert!(content.contains("_D3F6E97D-7783-492C-98CE-57EC815D304C"));
}

#[test]
fn test_compile_writes_to_stdout_by_default() {
    detent!()
        .arg("compile")
        .arg(hello_world_asset_dir())
        .assert()
        .success()
        .stdout(predicate::str::contains("definitions"))
        .stdout(predicate::str::contains("process"));
}

#[test]
fn test_compile_ignores_non_mdx_files() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("output.bpmn");
    detent!()
        .arg("compile")
        .arg(hello_world_asset_dir())
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();
}

#[test]
fn test_compile_with_individual_files() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("output.bpmn");
    let asset_dir = hello_world_asset_dir();
    let mdx_files: Vec<_> = std::fs::read_dir(&asset_dir)
        .expect("Failed to read asset dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("mdx"))
        .collect();

    let mut cmd = detent!();
    cmd.arg("compile");
    for f in &mdx_files {
        cmd.arg(f);
    }
    cmd.arg("--output").arg(&output_file).assert().success();

    let content = std::fs::read_to_string(&output_file).expect("Failed to read output");
    assert!(content.contains("definitions"));
    assert!(content.contains("process"));
}

#[test]
fn test_compile_rejects_non_mdx_file() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let non_mdx = temp_dir.path().join("data.txt");
    std::fs::write(&non_mdx, "not mdx content").expect("Failed to write");
    detent!()
        .arg("compile")
        .arg(&non_mdx)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not an .mdx file"));
}

#[test]
fn test_compile_tolerates_dangling_flow_target() {
    use std::process::ExitCode;

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let input_dir = temp_dir.path().join("input");
    let output_file = temp_dir.path().join("output.bpmn");
    std::fs::create_dir_all(&input_dir).expect("Failed to create input dir");

    let files = [
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
    ];

    for (name, content) in files {
        std::fs::write(input_dir.join(name), content).expect("Failed to write MDX input");
    }

    let status = detent::features::convert_bpmn_to_mdx::infrastructure::cli::compile::run(
        vec![Path::new(&input_dir).to_path_buf()],
        Some(output_file.clone()),
    );
    assert_eq!(status, ExitCode::SUCCESS);

    let content = fs::read_to_string(&output_file).expect("Failed to read output");
    assert!(content.contains("ghost_task"));
}
