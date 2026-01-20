//! CLI integration tests for the detent import command
//!
//! These tests verify that `detent import` generates MDX files that match
//! the reference files in tests/assets/processes/hello-world/

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

/// Get a Command for the detent binary
fn detent() -> Command {
    Command::cargo_bin("detent").expect("Failed to find detent binary")
}

/// Reference directory with expected MDX files
const REFERENCE_DIR: &str = "tests/assets/processes/hello-world";

/// Expected MDX files (excluding the BPMN source file)
const EXPECTED_MDX_FILES: &[&str] = &[
    "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx", // startEvent
    "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx", // task
    "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx", // endEvent
    "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx", // sequenceFlow
    "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx", // sequenceFlow
    "_process.mdx",                              // process
];

#[test]
fn test_import_help() {
    detent()
        .arg("import")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Import a BPMN file"));
}

#[test]
fn test_import_generates_all_expected_files() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();

    // Run import command
    detent()
        .arg("import")
        .arg("tests/assets/processes/hello-world/hello-world.bpmn2")
        .arg("--output-directory")
        .arg(output_dir)
        .assert()
        .success();

    // Check that all expected files were generated
    for file_name in EXPECTED_MDX_FILES {
        let generated_path = output_dir.join(file_name);
        assert!(
            generated_path.exists(),
            "Expected file not generated: {}",
            file_name
        );
    }

    // Check that no extra files were generated
    let generated_files: Vec<_> = fs::read_dir(output_dir)
        .expect("Failed to read output dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    assert_eq!(
        generated_files.len(),
        EXPECTED_MDX_FILES.len(),
        "Wrong number of files generated. Expected {:?}, got {:?}",
        EXPECTED_MDX_FILES,
        generated_files
    );
}

#[test]
fn test_import_frontmatter_matches_reference() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();

    // Run import command
    detent()
        .arg("import")
        .arg("tests/assets/processes/hello-world/hello-world.bpmn2")
        .arg("--output-directory")
        .arg(output_dir)
        .assert()
        .success();

    // Compare frontmatter of each generated file with reference
    for file_name in EXPECTED_MDX_FILES {
        let generated_path = output_dir.join(file_name);
        let reference_path = Path::new(REFERENCE_DIR).join(file_name);

        let generated_content =
            fs::read_to_string(&generated_path).expect("Failed to read generated file");
        let reference_content =
            fs::read_to_string(&reference_path).expect("Failed to read reference file");

        // Extract frontmatter from both files
        let generated_frontmatter = extract_frontmatter(&generated_content)
            .unwrap_or_else(|| panic!("No frontmatter in generated file: {}", file_name));
        let reference_frontmatter = extract_frontmatter(&reference_content)
            .unwrap_or_else(|| panic!("No frontmatter in reference file: {}", file_name));

        // Parse as YAML and compare
        let generated_yaml: serde_yaml::Value =
            serde_yaml::from_str(&generated_frontmatter).expect("Invalid YAML in generated file");
        let reference_yaml: serde_yaml::Value =
            serde_yaml::from_str(&reference_frontmatter).expect("Invalid YAML in reference file");

        assert_eq!(
            generated_yaml, reference_yaml,
            "Frontmatter mismatch for {}\nGenerated:\n{}\nReference:\n{}",
            file_name, generated_frontmatter, reference_frontmatter
        );
    }
}

/// Extract frontmatter from MDX content (between --- delimiters)
fn extract_frontmatter(content: &str) -> Option<String> {
    let content = content.trim();
    if !content.starts_with("---") {
        return None;
    }

    let rest = &content[3..];
    let end_pos = rest.find("\n---")?;
    Some(rest[..end_pos].trim().to_string())
}

#[test]
fn test_import_missing_bpmn_file() {
    detent()
        .arg("import")
        .arg("nonexistent.bpmn")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read"));
}

#[test]
fn test_import_requires_bpmn_file() {
    detent()
        .arg("import")
        .assert()
        .failure();
}
