use predicates::prelude::*;
use std::fs;
use std::path::Path;

use super::{hello_world_asset_dir, hello_world_asset_path};

fn tdd_asset_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/features/convert_bpmn_to_mdx/tests/assets/tdd/tdd.bpmn2")
}

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
        let mut gen_yaml: serde_yaml::Value = serde_yaml::from_str(&gen_fm).expect("Invalid YAML");
        let mut ref_yaml: serde_yaml::Value = serde_yaml::from_str(&ref_fm).expect("Invalid YAML");
        if let serde_yaml::Value::Mapping(ref mut m) = gen_yaml {
            m.remove("diagram");
        }
        if let serde_yaml::Value::Mapping(ref mut m) = ref_yaml {
            m.remove("diagram");
        }
        assert_eq!(gen_yaml, ref_yaml, "Frontmatter mismatch for {}", file_name);
    }
}

#[test]
fn test_imported_tdd_fixture_compiles() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output_dir = temp_dir.path();
    let output_file = temp_dir.path().join("roundtrip.bpmn");

    detent!()
        .arg("import")
        .arg(tdd_asset_path())
        .arg("--output-directory")
        .arg(output_dir)
        .assert()
        .success();

    detent!()
        .arg("compile")
        .arg(output_dir)
        .arg("--output")
        .arg(&output_file)
        .assert()
        .success();

    let content = fs::read_to_string(&output_file).expect("Failed to read round-trip output");
    assert!(content.contains("Task_WriteFeatureFile"));
    assert!(content.contains("Task_WriteFailingTest"));
    assert!(content.contains("Task_WriteCode"));
    assert!(content.contains("Task_RunLocalTests"));
    assert!(content.contains("Task_RefactorCode"));
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
