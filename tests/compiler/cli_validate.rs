//! CLI integration tests for the detent validate command
//!
//! These tests use the built executable to validate files in tests/assets/processes/hello-world/

use assert_cmd::Command;
use predicates::prelude::*;

/// Get a Command for the detent binary
fn detent() -> Command {
    Command::cargo_bin("detent").expect("Failed to find detent binary")
}

#[test]
fn test_validate_help() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("BPMN"));
}

#[test]
fn test_validate_bpmn_file() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("tests/assets/processes/hello-world/hello-world.bpmn2")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_start_event() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_task() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_end_event() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("tests/assets/processes/hello-world/_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_mdx_sequence_flow() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("tests/assets/processes/hello-world/_4083739B-66F0-4B92-A348-A37DF3B29083.mdx")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_validate_multiple_files() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("tests/assets/processes/hello-world/hello-world.bpmn2")
        .arg("tests/assets/processes/hello-world/_1E892844-423C-464F-ADC4-22F1EC73851B.mdx")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓").count(2));
}

#[test]
fn test_validate_missing_file() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("nonexistent.bpmn")
        .assert()
        .failure()
        .stderr(predicate::str::contains("✗"));
}

#[test]
fn test_validate_unknown_extension() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .arg("tests/assets/processes/hello-world/_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")
        .arg("Cargo.toml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown file type"));
}

#[test]
fn test_validate_requires_files() {
    // Arrange
    let mut cmd = detent();

    // Act & Assert
    cmd.arg("validate")
        .assert()
        .failure();
}
