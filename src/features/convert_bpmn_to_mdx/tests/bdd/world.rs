use cucumber::{writer, StatsWriter as _, World, WriterExt as _};
use detent::features::convert_bpmn_to_mdx::use_cases::compile::MdxInput;
use detent::features::convert_bpmn_to_mdx::use_cases::import::MdxOutput;
use std::path::PathBuf;

mod dangling_flow_tolerated;
mod empty_input_rejected;
mod hello_world_imports_to_5_mdx;
mod import_rejects_no_process;
mod minimal_mdx_compiles_to_process;
mod missing_frontmatter_rejected;
mod missing_type_rejected;
mod steps;
mod unknown_type_rejected;

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
    pub compile_result: Option<detent::features::convert_bpmn_to_mdx::adapters::bpmn::Definitions>,
    pub compile_failed: bool,
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
