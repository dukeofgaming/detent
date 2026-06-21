use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by Cargo"));
    let features_dir = manifest_dir.join("src/features");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));

    println!("cargo:rerun-if-changed={}", features_dir.display());

    let mut world_modules = Vec::new();
    for entry in fs::read_dir(&features_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let feature_name = entry.file_name().to_string_lossy().into_owned();
        if !is_slice_enabled(&feature_name) {
            continue;
        }

        let world_test = entry.path().join("tests/world.rs");
        if world_test.exists() {
            println!("cargo:rerun-if-changed={}", world_test.display());
            world_modules.push((feature_name, world_test));
        }
    }

    world_modules.sort_by(|left, right| left.0.cmp(&right.0));

    let mut harness_src = String::new();
    for (feature_name, world_test) in &world_modules {
        harness_src.push_str(&format!(
            "#[path = {:?}]\nmod {};\n\n",
            normalize_path(world_test),
            feature_name
        ));
    }
    for (feature_name, _) in &world_modules {
        harness_src.push_str(&format!(
            "#[test]\nfn {feature}() {{\n    let runtime = tokio::runtime::Builder::new_current_thread()\n        .enable_all()\n        .build()\n        .expect(\"failed to build tokio runtime\");\n    let failed = runtime.block_on({feature}::run());\n    assert!(!failed, \"{feature} scenarios failed\");\n}}\n\n",
            feature = feature_name
        ));
    }
    fs::write(out_dir.join("feature_slices.rs"), harness_src)?;

    Ok(())
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn is_slice_enabled(name: &str) -> bool {
    match name {
        "graph_validation" => env::var("CARGO_FEATURE_GRAPH_VALIDATION").is_ok(),
        _ => true,
    }
}
