use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::Definitions;
use detent::features::convert_bpmn_to_mdx::use_cases::compile::MdxInput;
use detent::features::convert_bpmn_to_mdx::use_cases::import::MdxOutput;
use std::path::PathBuf;

#[path = "unit/steps.rs"]
mod unit_steps;
#[path = "functional/steps.rs"]
mod functional_steps;
#[path = "integration/steps.rs"]
mod integration_steps;
#[path = "e2e/steps.rs"]
mod e2e_steps;

#[path = "functional/scenarios/minimal_mdx_compiles_to_process.rs"]
mod minimal_mdx_compiles_to_process;
#[path = "functional/scenarios/empty_input_rejected.rs"]
mod empty_input_rejected;
#[path = "functional/scenarios/missing_type_rejected.rs"]
mod missing_type_rejected;
#[path = "functional/scenarios/dangling_flow_tolerated.rs"]
mod dangling_flow_tolerated;
#[path = "functional/scenarios/missing_frontmatter_rejected.rs"]
mod missing_frontmatter_rejected;
#[path = "functional/scenarios/unknown_type_rejected.rs"]
mod unknown_type_rejected;
#[path = "functional/scenarios/import_rejects_no_process.rs"]
mod import_rejects_no_process;
#[path = "functional/scenarios/import_filenames_match_ids.rs"]
mod import_filenames_match_ids;
#[path = "functional/scenarios/frontmatter_has_no_xml_artifacts.rs"]
mod frontmatter_has_no_xml_artifacts;
#[path = "functional/scenarios/hello_world_mdx_compiles.rs"]
mod hello_world_mdx_compiles;
#[path = "functional/scenarios/mdx_compiles_and_imports_back.rs"]
mod mdx_compiles_and_imports_back;
#[path = "functional/scenarios/full_process_with_gateways_compiles.rs"]
mod full_process_with_gateways_compiles;
#[path = "functional/scenarios/condition_expression_survives_import.rs"]
mod condition_expression_survives_import;
#[path = "functional/scenarios/bpmn_mdx_roundtrip.rs"]
mod bpmn_mdx_roundtrip;
#[path = "integration/scenarios/hello_world_imports_to_mdx.rs"]
mod hello_world_imports_to_mdx;
#[path = "integration/scenarios/hello_world_bpmn_metadata.rs"]
mod hello_world_bpmn_metadata;
#[path = "integration/scenarios/bpmn_parsing.rs"]
mod bpmn_parsing;
#[cfg(feature = "xsd-validation")]
#[path = "integration/scenarios/bpmn_xsd_validation.rs"]
mod bpmn_xsd_validation;
#[path = "unit/scenarios/mdx_types.rs"]
mod mdx_types;
#[path = "unit/scenarios/bpmn_types.rs"]
mod bpmn_types;
#[path = "e2e/scenarios/cli_common.rs"]
mod cli_common;
#[path = "e2e/scenarios/cli_compile.rs"]
mod cli_compile;
#[path = "e2e/scenarios/cli_import.rs"]
mod cli_import;
#[path = "e2e/scenarios/cli_validate.rs"]
mod cli_validate;

const HELLO_WORLD_ASSET_DIR: &str = "src/features/convert_bpmn_to_mdx/tests/assets/hello_world";

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
        .join("src/features/convert_bpmn_to_mdx/tests/assets")
        .join(relative_path)
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
        "/src/features/convert_bpmn_to_mdx/tests"
    );
    let json_file = std::fs::File::create(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/cucumber-report/convert_bpmn_to_mdx.json"
        ),
    )
    .expect("Failed to create JSON output");
    let junit_file = std::fs::File::create(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/target/cucumber-report/convert_bpmn_to_mdx.junit.xml"
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
