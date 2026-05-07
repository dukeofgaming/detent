//! CLI integration tests for the detent compile command
//!
//! These tests verify that `detent compile` reads a directory of MDX files
//! and produces valid BPMN XML output.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

/// Get a Command for the detent binary
fn detent() -> Command {
    Command::cargo_bin("detent").expect("Failed to find detent binary")
}

#[test]
fn test_compile_help() {
    detent()
        .arg("compile")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Compile MDX files"));
}

#[test]
fn test_compile_requires_directory() {
    detent().arg("compile").assert().failure();
}

#[test]
fn test_compile_missing_directory() {
    detent()
        .arg("compile")
        .arg("nonexistent-dir")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read"));
}

#[test]
fn test_compile_produces_bpmn_xml_file() {
    // Arrange
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("output.bpmn");

    // Act
    detent()
        .arg("compile")
        .arg("tests/assets/processes/hello-world")
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();

    // Assert
    assert!(output_file.exists(), "Output BPMN file should exist");
    let content = fs::read_to_string(&output_file).expect("Failed to read output");
    assert!(
        content.contains("definitions"),
        "Output should contain BPMN definitions element"
    );
    assert!(
        content.contains("process"),
        "Output should contain a process element"
    );
}

#[test]
fn test_compile_output_contains_all_elements() {
    // Arrange
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("output.bpmn");

    // Act
    detent()
        .arg("compile")
        .arg("tests/assets/processes/hello-world")
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();

    // Assert
    let content = fs::read_to_string(&output_file).expect("Failed to read output");

    // Should contain the IDs from the hello-world process
    assert!(
        content.contains("_1E892844-423C-464F-ADC4-22F1EC73851B"),
        "Should contain start event ID"
    );
    assert!(
        content.contains("_808AA40C-EAA1-40C4-A2DC-27000FBF1866"),
        "Should contain task ID"
    );
    assert!(
        content.contains("_D3F6E97D-7783-492C-98CE-57EC815D304C"),
        "Should contain end event ID"
    );
}

#[test]
fn test_compile_writes_to_stdout_by_default() {
    // Act & Assert: without --output, should write to stdout
    detent()
        .arg("compile")
        .arg("tests/assets/processes/hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("definitions"))
        .stdout(predicate::str::contains("process"));
}

#[test]
fn test_compile_ignores_non_mdx_files() {
    // The hello-world directory also contains .bpmn and .bpmn2 files.
    // Compile should only read .mdx files.
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("output.bpmn");

    detent()
        .arg("compile")
        .arg("tests/assets/processes/hello-world")
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();

    // If it tried to parse .bpmn files as MDX, it would fail
    // Success here means it correctly filtered to .mdx only
}
