use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::bpmn_mdx_transpiler::adapters::bpmn::Definitions;
use detent::features::bpmn_mdx_transpiler::application::compile::MdxInput;
use detent::features::bpmn_mdx_transpiler::application::import::MdxOutput;
use std::path::PathBuf;

mod steps;
mod functional;
mod integration;
mod unit;
mod e2e;

const HELLO_WORLD_ASSET_DIR: &str = "src/features/bpmn_mdx_transpiler/tests/assets/hello_world";

pub(crate) fn hello_world_asset_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(HELLO_WORLD_ASSET_DIR)
        .join(name)
}

pub(crate) fn hello_world_asset_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(HELLO_WORLD_ASSET_DIR)
}

pub(crate) fn fixture_path(relative_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/features/bpmn_mdx_transpiler/tests/assets")
        .join(relative_path)
}

macro_rules! detent {
    () => {{
        assert_cmd::Command::new(env!("CARGO_BIN_EXE_detent"))
    }};
}

pub(crate) fn run_detent(world: &mut ConvertWorld, args: &[&str]) {
    let mut cmd = detent!();
    if let Some(dir) = &world.e2e_work_dir {
        cmd.current_dir(dir);
    }
    for arg in args {
        if arg.starts_with('@') || arg.starts_with('#') {
            cmd.arg(world.e2e_output_file.as_ref().expect("output file path"));
        } else {
            cmd.arg(arg);
        }
    }
    let output = cmd.output().expect("detent command failed");
    world.e2e_last_success = output.status.success();
    world.e2e_last_stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    world.e2e_last_stderr = String::from_utf8_lossy(&output.stderr).into_owned();
}

pub(crate) fn write_temp_mdx(world: &mut ConvertWorld, name: &str, content: &str) {
    let base = world.e2e_work_dir.as_ref().expect("temp workspace");
    let path = base.join(name);
    std::fs::write(&path, content).expect("write mdx");
    world.e2e_output_file = Some(path);
}

#[derive(Debug, Default, World)]
pub struct ConvertWorld {
    pub bpmn_xml: Option<String>,
    pub mdx_inputs: Vec<MdxInput>,
    pub compile_result: Option<Definitions>,
    pub compile_failed: bool,
    pub parsed_defs: Option<Definitions>,
    pub import_failed: bool,
    pub import_outputs: Option<Vec<MdxOutput>>,
    pub e2e_temp: Option<tempfile::TempDir>,
    pub e2e_work_dir: Option<PathBuf>,
    pub e2e_last_stdout: String,
    pub e2e_last_stderr: String,
    pub e2e_last_success: bool,
    pub e2e_output_file: Option<PathBuf>,
    pub e2e_file_content: Option<String>,
    pub unit_last_error: Option<String>,
}

pub async fn run() -> bool {
    let features_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/features/bpmn_mdx_transpiler/tests"
    );
    let json_file = std::fs::File::create(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/cucumber-report/bpmn_mdx_transpiler.json"
        ),
    )
    .expect("Failed to create JSON output");
    let junit_file = std::fs::File::create(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/cucumber-report/bpmn_mdx_transpiler.junit.xml"
        ),
    )
    .expect("Failed to create JUnit output");
    let json_writer = writer::Json::for_tee(json_file).normalized();
    let junit_writer = writer::JUnit::for_tee(junit_file, 0).normalized();
    let combined = writer::Basic::stdout()
        .summarized()
        .tee(json_writer)
        .tee(junit_writer);
    let summarized = ConvertWorld::cucumber()
        .with_writer(combined)
        .filter_run(features_path, |_, _, scenario| {
            #[cfg(not(feature = "xsd-validation"))]
            if scenario
                .tags
                .iter()
                .any(|tag| tag == "xsd-validation")
            {
                return false;
            }
            true
        })
        .await;
    summarized.execution_has_failed()
}
