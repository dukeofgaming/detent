use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::convert_bpmn_to_mdx::adapters::bpmn::Definitions;
use detent::features::convert_bpmn_to_mdx::use_cases::compile::MdxInput;
use detent::features::convert_bpmn_to_mdx::use_cases::import::MdxOutput;
use std::path::PathBuf;

mod steps;

#[path = "scenarios/hello_world_imports_to_5_mdx.rs"]
mod hello_world_imports_to_5_mdx;
#[path = "scenarios/minimal_mdx_compiles_to_process.rs"]
mod minimal_mdx_compiles_to_process;
#[path = "scenarios/empty_input_rejected.rs"]
mod empty_input_rejected;
#[path = "scenarios/missing_type_rejected.rs"]
mod missing_type_rejected;
#[path = "scenarios/dangling_flow_tolerated.rs"]
mod dangling_flow_tolerated;
#[path = "scenarios/missing_frontmatter_rejected.rs"]
mod missing_frontmatter_rejected;
#[path = "scenarios/unknown_type_rejected.rs"]
mod unknown_type_rejected;
#[path = "scenarios/import_rejects_no_process.rs"]
mod import_rejects_no_process;
#[path = "scenarios/import_filenames_match_ids.rs"]
mod import_filenames_match_ids;
#[path = "scenarios/frontmatter_has_no_xml_artifacts.rs"]
mod frontmatter_has_no_xml_artifacts;
#[path = "scenarios/hello_world_bpmn_metadata.rs"]
mod hello_world_bpmn_metadata;
#[path = "scenarios/hello_world_mdx_compiles.rs"]
mod hello_world_mdx_compiles;

const HELLO_WORLD_ASSET_DIR: &str = "src/features/convert_bpmn_to_mdx/tests/assets/hello_world";

fn hello_world_asset_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(HELLO_WORLD_ASSET_DIR)
        .join(name)
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
}

pub async fn run() -> bool {
    let features_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/features/convert_bpmn_to_mdx/tests/bdd/slice.feature"
    );
    let summarized = ConvertWorld::cucumber()
        .with_writer(writer::Basic::stdout().summarized())
        .run(features_path)
        .await;
    summarized.execution_has_failed()
}
