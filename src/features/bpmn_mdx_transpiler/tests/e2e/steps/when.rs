use cucumber::when;

use super::super::super::ConvertWorld;
use super::super::super::run_detent;

#[when(regex = r#"^I run detent with args "(.*)"$"#)]
fn when_run_detent(world: &mut ConvertWorld, args: String) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    run_detent(world, &parts);
}

#[when("I compile hello-world MDX to the output file")]
fn when_compile_hello_world(world: &mut ConvertWorld) {
    let out = world
        .e2e_output_file
        .as_ref()
        .expect("output file")
        .clone();
    run_detent(
        world,
        &[
            "compile",
            &super::super::super::hello_world_asset_dir().to_string_lossy(),
            "--output",
            &out.to_string_lossy(),
        ],
    );
}

#[when("I compile hello-world MDX files individually to the output file")]
fn when_compile_individual(world: &mut ConvertWorld) {
    let out = world
        .e2e_output_file
        .as_ref()
        .expect("output file")
        .clone();
    let dir = super::super::super::hello_world_asset_dir();
    let mut args = vec!["compile".to_string()];
    for entry in std::fs::read_dir(&dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("mdx") {
            args.push(path.to_string_lossy().to_string());
        }
    }
    args.push("--output".to_string());
    args.push(out.to_string_lossy().to_string());
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_detent(world, &arg_refs);
}

#[when("I compile all MDX files in the temp workspace to stdout")]
fn when_compile_stdout(world: &mut ConvertWorld) {
    let dir = world.e2e_work_dir.as_ref().expect("temp workspace");
    let mut args = vec!["compile".to_string()];
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("mdx") {
            args.push(path.to_string_lossy().to_string());
        }
    }
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_detent(world, &arg_refs);
}

#[when("I import hello-world BPMN to the temp output directory")]
fn when_import_hello(world: &mut ConvertWorld) {
    let bpmn = super::super::super::hello_world_asset_path("hello-world.bpmn2")
        .to_string_lossy()
        .into_owned();
    let out = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["import", &bpmn, "--output-directory", &out]);
}

#[when("I import tdd BPMN to the temp output directory")]
fn when_import_tdd(world: &mut ConvertWorld) {
    let bpmn = super::super::super::fixture_path("tdd/tdd.bpmn2")
        .to_string_lossy()
        .into_owned();
    let out = world
        .e2e_work_dir
        .as_ref()
        .expect("temp workspace")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["import", &bpmn, "--output-directory", &out]);
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
    run_detent(world, &["compile", &out_s, "--output", &output_s]);
}

#[when("I validate the prepared file with detent")]
fn when_validate_file(world: &mut ConvertWorld) {
    let path = world
        .e2e_output_file
        .as_ref()
        .expect("file path")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["validate", &path]);
}

#[when("I validate hello-world BPMN and start event MDX")]
fn when_validate_two(world: &mut ConvertWorld) {
    let bpmn = super::super::super::hello_world_asset_path("hello-world.bpmn2")
        .to_string_lossy()
        .into_owned();
    let mdx = super::super::super::hello_world_asset_path(
        "_1E892844-423C-464F-ADC4-22F1EC73851B.mdx",
    )
    .to_string_lossy()
    .into_owned();
    run_detent(world, &["validate", &bpmn, &mdx]);
}

#[when("I validate hello-world task MDX and Cargo.toml")]
fn when_validate_unknown_ext(world: &mut ConvertWorld) {
    let mdx = super::super::super::hello_world_asset_path("_808AA40C-EAA1-40C4-A2DC-27000FBF1866.mdx")
        .to_string_lossy()
        .into_owned();
    let cargo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Cargo.toml")
        .to_string_lossy()
        .into_owned();
    run_detent(world, &["validate", &mdx, &cargo]);
}
