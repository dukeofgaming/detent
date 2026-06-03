use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by Cargo"));
    let features_dir = manifest_dir.join("src/features");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));

    println!("cargo:rerun-if-changed={}", features_dir.display());

    let mut modules = Vec::new();
    for entry in fs::read_dir(&features_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let feature_name = entry.file_name().to_string_lossy().into_owned();
        let integration_test = entry.path().join("tests/integration.rs");
        if integration_test.exists() {
            println!("cargo:rerun-if-changed={}", integration_test.display());
            modules.push((feature_name, integration_test));
        }
    }

    modules.sort_by(|left, right| left.0.cmp(&right.0));

    let mut generated = String::new();
    for (feature_name, integration_test) in modules {
        generated.push_str(&format!(
            "#[path = {:?}]\nmod {};\n\n",
            normalize_path(&integration_test),
            feature_name
        ));
    }

    fs::write(out_dir.join("feature_slices.rs"), generated)
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
