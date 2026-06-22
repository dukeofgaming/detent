use std::fs;

use cucumber::{given, then, when};

use super::cli_common::run_detent;
use super::super::ConvertWorld;

const EXPECTED_MDX_FILES: &[&str] = &[
    "hello_world.mdx",
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

#[given("hello-world BPMN file path")]
fn given_bpmn_path(world: &mut ConvertWorld) {
    world.e2e_output_file = Some(super::super::hello_world_asset_path("hello-world.bpmn2"));
}

#[given("tdd BPMN file path")]
fn given_tdd_path(world: &mut ConvertWorld) {
    world.e2e_output_file = Some(super::super::fixture_path("tdd/tdd.bpmn2"));
}

#[when("I import hello-world BPMN to the temp output directory")]
fn when_import_hello(world: &mut ConvertWorld) {
    let bpmn = super::super::hello_world_asset_path("hello-world.bpmn2")
        .to_string_lossy()
        .into_owned();
    let out = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .to_string_lossy()
        .into_owned();
    run_detent(
        world,
        &["import", &bpmn, "--output-directory", &out],
    );
}

#[when("I import tdd BPMN to the temp output directory")]
fn when_import_tdd(world: &mut ConvertWorld) {
    let bpmn = super::super::fixture_path("tdd/tdd.bpmn2")
        .to_string_lossy()
        .into_owned();
    let out = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .to_string_lossy()
        .into_owned();
    run_detent(
        world,
        &["import", &bpmn, "--output-directory", &out],
    );
}

#[then("all expected hello-world MDX files exist")]
fn then_expected_files(world: &mut ConvertWorld) {
    let out = world.e2e_work_dir.as_ref().expect("temp workspace");
    for file_name in EXPECTED_MDX_FILES {
        assert!(
            out.join(file_name).exists(),
            "Expected file not generated: {file_name}"
        );
    }
}

#[then("imported hello-world frontmatter matches reference")]
fn then_frontmatter_matches(world: &mut ConvertWorld) {
    let out = world.e2e_work_dir.as_ref().expect("temp workspace");
    let reference_dir = super::super::hello_world_asset_dir();
    for file_name in EXPECTED_MDX_FILES {
        let generated = fs::read_to_string(out.join(file_name)).expect("read generated");
        let reference = fs::read_to_string(reference_dir.join(file_name)).expect("read reference");
        let gen_fm = extract_frontmatter(&generated).unwrap();
        let ref_fm = extract_frontmatter(&reference).unwrap();
        let mut gen_yaml: serde_yaml::Value = serde_yaml::from_str(&gen_fm).expect("gen yaml");
        let mut ref_yaml: serde_yaml::Value = serde_yaml::from_str(&ref_fm).expect("ref yaml");
        if let serde_yaml::Value::Mapping(ref mut m) = gen_yaml {
            m.remove("diagram");
        }
        if let serde_yaml::Value::Mapping(ref mut m) = ref_yaml {
            m.remove("diagram");
        }
        assert_eq!(gen_yaml, ref_yaml, "Frontmatter mismatch for {file_name}");
    }
}

#[when("I compile the imported tdd MDX to BPMN in the temp workspace")]
fn when_compile_tdd(world: &mut ConvertWorld) {
    let out = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .to_path_buf();
    let output_file = out.join("roundtrip.bpmn");
    world.e2e_output_file = Some(output_file.clone());
    let out_s = out.to_string_lossy().into_owned();
    let output_s = output_file.to_string_lossy().into_owned();
    run_detent(
        world,
        &["compile", &out_s, "--output", &output_s],
    );
}

#[then("roundtrip BPMN contains tdd task ids")]
fn then_tdd_tasks(world: &mut ConvertWorld) {
    let content = fs::read_to_string(world.e2e_output_file.as_ref().expect("output"))
        .expect("read roundtrip");
    for id in [
        "Task_WriteFeatureFile",
        "Task_WriteFailingTest",
        "Task_WriteCode",
        "Task_RunLocalTests",
        "Task_RefactorCode",
    ] {
        assert!(content.contains(id), "missing {id}");
    }
}

#[then(regex = r#"^the temp workspace contains "([^"]+)"$"#)]
fn then_temp_contains(world: &mut ConvertWorld, name: String) {
    let out = world.e2e_work_dir.as_ref().expect("temp workspace");
    assert!(out.join(&name).exists(), "missing {name}");
}
