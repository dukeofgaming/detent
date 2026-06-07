use predicates::prelude::*;
use std::fs;
use std::path::Path;

use super::{hello_world_asset_dir, hello_world_asset_path};

const EXPECTED_MDX_FILES: &[&str] = &[
    "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx",
    "_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx",
    "_D3F6E97D-7783-492C-98CE-57EC815D304C.mdx",
    "_4083739B-66F0-4B92-A348-A37DF3B29083.mdx",
    "_44A6FA69-CAAD-4DCE-BAE3-5F38D0A709FB.mdx",
];

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
fn test_import_help() {
    detent!()
        .arg("import")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("BPMN XML file"));
}

#[test]
fn test_import_generates_all_expected_files() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();
    detent!()
        .arg("import")
        .arg(hello_world_asset_path("hello-world.bpmn2"))
        .arg("--output-directory")
        .arg(output_dir)
        .assert()
        .success();
    for file_name in EXPECTED_MDX_FILES {
        assert!(
            output_dir.join(file_name).exists(),
            "Expected file not generated: {}",
            file_name
        );
    }
}

#[test]
fn test_import_frontmatter_matches_reference() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();
    detent!()
        .arg("import")
        .arg(hello_world_asset_path("hello-world.bpmn2"))
        .arg("--output-directory")
        .arg(output_dir)
        .assert()
        .success();
    for file_name in EXPECTED_MDX_FILES {
        let generated =
            fs::read_to_string(output_dir.join(file_name)).expect("Failed to read generated");
        let reference =
            fs::read_to_string(Path::new(&hello_world_asset_dir()).join(file_name))
                .expect("Failed to read reference");
        let gen_fm = extract_frontmatter(&generated).unwrap();
        let ref_fm = extract_frontmatter(&reference).unwrap();
        let gen_yaml: serde_yaml::Value = serde_yaml::from_str(&gen_fm).expect("Invalid YAML");
        let ref_yaml: serde_yaml::Value = serde_yaml::from_str(&ref_fm).expect("Invalid YAML");
        assert_eq!(gen_yaml, ref_yaml, "Frontmatter mismatch for {}", file_name);
    }
}

#[test]
fn test_import_missing_bpmn_file() {
    detent!()
        .arg("import")
        .arg("nonexistent.bpmn")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read"));
}

#[test]
fn test_import_requires_bpmn_file() {
    detent!().arg("import").assert().failure();
}
