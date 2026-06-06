use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by Cargo"));
    let features_dir = manifest_dir.join("src/features");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));

    println!("cargo:rerun-if-changed={}", features_dir.display());

    let mut integration_modules = Vec::new();
    let mut unit_modules = Vec::new();
    let mut cucumber_modules = Vec::new();
    for entry in fs::read_dir(&features_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let feature_name = entry.file_name().to_string_lossy().into_owned();
        let integration_test = entry.path().join("tests/integration/mod.rs");
        if integration_test.exists() {
            println!("cargo:rerun-if-changed={}", integration_test.display());
            integration_modules.push((feature_name.clone(), integration_test));
        }

        let unit_test = entry.path().join("tests/unit/mod.rs");
        if unit_test.exists() {
            println!("cargo:rerun-if-changed={}", unit_test.display());
            unit_modules.push((feature_name.clone(), unit_test));
        }

        let cucumber_test = entry.path().join("tests/bdd/world.rs");
        if cucumber_test.exists() {
            println!("cargo:rerun-if-changed={}", cucumber_test.display());
            cucumber_modules.push((feature_name, cucumber_test));
        }
    }

    integration_modules.sort_by(|left, right| left.0.cmp(&right.0));
    unit_modules.sort_by(|left, right| left.0.cmp(&right.0));
    cucumber_modules.sort_by(|left, right| left.0.cmp(&right.0));

    let mut integration_src = String::new();
    for (feature_name, integration_test) in &integration_modules {
        integration_src.push_str(&format!(
            "#[path = {:?}]\nmod {};\n\n",
            normalize_path(integration_test),
            feature_name
        ));
    }
    for (feature_name, unit_test) in &unit_modules {
        integration_src.push_str(&format!(
            "#[path = {:?}]\nmod {}_unit;\n\n",
            normalize_path(unit_test),
            feature_name
        ));
    }
    fs::write(out_dir.join("feature_slices.rs"), integration_src)?;

    let mut cucumber_src = String::new();
    for (feature_name, cucumber_test) in &cucumber_modules {
        cucumber_src.push_str(&format!(
            "#[path = {:?}]\nmod {}_cucumber;\n\n",
            normalize_path(cucumber_test),
            feature_name
        ));
    }
    for (feature_name, _) in &cucumber_modules {
        cucumber_src.push_str(&format!(
            "#[test]\nfn {feature}_cucumber_scenarios() {{\n    let runtime = tokio::runtime::Builder::new_current_thread()\n        .enable_all()\n        .build()\n        .expect(\"failed to build tokio runtime\");\n    let failed = runtime.block_on({feature}_cucumber::run());\n    assert!(!failed, \"{feature} cucumber scenarios failed\");\n}}\n\n",
            feature = feature_name
        ));
    }
    fs::write(out_dir.join("feature_slices_cucumber.rs"), cucumber_src)?;

    Ok(())
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
