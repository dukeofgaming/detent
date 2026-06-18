use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::serialize_bpmn;
use crate::features::convert_bpmn_to_mdx::use_cases::compile::{compile_to_definitions, MdxInput};

pub fn run(files: Vec<PathBuf>, output: Option<PathBuf>) -> ExitCode {
    let mut inputs: Vec<MdxInput> = Vec::new();

    for path in &files {
        if !path.exists() {
            if path.extension().and_then(|e| e.to_str()) == Some("mdx")
                || path.extension().is_none()
            {
                eprintln!("Failed to read {}: no such file or directory", path.display());
                return ExitCode::FAILURE;
            }
            eprintln!("Not an .mdx file: {}", path.display());
            return ExitCode::FAILURE;
        }

        if path.is_dir() {
            let entries = match fs::read_dir(path) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("Failed to read directory {}: {}", path.display(), e);
                    return ExitCode::FAILURE;
                }
            };

            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Failed to read directory entry: {}", e);
                        return ExitCode::FAILURE;
                    }
                };

                let entry_path = entry.path();
                if entry_path.extension().and_then(|e| e.to_str()) != Some("mdx") {
                    continue;
                }

                let filename = entry_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let content = match fs::read_to_string(&entry_path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to read {}: {}", entry_path.display(), e);
                        return ExitCode::FAILURE;
                    }
                };

                inputs.push(MdxInput { filename, content });
            }
        } else {
            if path.extension().and_then(|e| e.to_str()) != Some("mdx") {
                eprintln!("Not an .mdx file: {}", path.display());
                return ExitCode::FAILURE;
            }

            let filename = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to read {}: {}", path.display(), e);
                    return ExitCode::FAILURE;
                }
            };

            inputs.push(MdxInput { filename, content });
        }
    }

    if inputs.is_empty() {
        eprintln!("No .mdx files found");
        return ExitCode::FAILURE;
    }

    let definitions = match compile_to_definitions(&inputs) {
        Ok(defs) => defs,
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let xml = match serialize_bpmn(&definitions) {
        Ok(xml) => xml,
        Err(e) => {
            eprintln!("Failed to serialize BPMN: {}", e);
            return ExitCode::FAILURE;
        }
    };

    match output {
        Some(path) => {
            if let Err(e) = fs::write(&path, &xml) {
                eprintln!("Failed to write {}: {}", path.display(), e);
                return ExitCode::FAILURE;
            }
            println!("Compiled BPMN written to {}", path.display());
        }
        None => {
            print!("{}", xml);
        }
    }

    ExitCode::SUCCESS
}
