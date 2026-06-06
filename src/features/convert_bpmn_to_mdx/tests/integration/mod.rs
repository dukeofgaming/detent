use std::path::PathBuf;

const HELLO_WORLD_ASSET_DIR: &str = "src/features/convert_bpmn_to_mdx/tests/assets/hello_world";

fn hello_world_asset_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(HELLO_WORLD_ASSET_DIR)
        .join(name)
}

fn hello_world_asset_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(HELLO_WORLD_ASSET_DIR)
}

macro_rules! detent {
    () => {{
        #[allow(deprecated)]
        assert_cmd::Command::cargo_bin("detent").expect("Failed to find detent binary")
    }};
}

mod bpmn_parsing;
mod bpmn_types;
#[cfg(feature = "xsd-validation")]
mod bpmn_xsd_validation;
mod cli_compile;
mod cli_import;
mod cli_validate;
mod compile;
mod mdx_types;
