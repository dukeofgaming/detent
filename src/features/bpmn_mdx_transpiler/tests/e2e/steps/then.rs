use std::fs;

use cucumber::then;
use predicates::prelude::*;

use super::super::super::ConvertWorld;

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
    let reference_dir = super::super::super::hello_world_asset_dir();
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
