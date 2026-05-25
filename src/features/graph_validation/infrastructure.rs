pub mod cli {
    pub mod compile {
        use std::fs;
        use std::path::PathBuf;
        use std::process::ExitCode;

        use crate::features::graph_validation::use_cases::validate::validate_bpmn_definitions;
        use crate::transpiler::bpmn::serialize_bpmn;
        use crate::transpiler::compile::{compile_to_definitions, MdxInput};

        pub fn run(directory: PathBuf, output: Option<PathBuf>) -> ExitCode {
            let entries = match fs::read_dir(&directory) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("Failed to read directory {}: {}", directory.display(), e);
                    return ExitCode::FAILURE;
                }
            };

            let mut inputs: Vec<MdxInput> = Vec::new();

            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Failed to read directory entry: {}", e);
                        return ExitCode::FAILURE;
                    }
                };

                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("mdx") {
                    continue;
                }

                let filename = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let content = match fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to read {}: {}", path.display(), e);
                        return ExitCode::FAILURE;
                    }
                };

                inputs.push(MdxInput { filename, content });
            }

            if inputs.is_empty() {
                eprintln!("No .mdx files found in {}", directory.display());
                return ExitCode::FAILURE;
            }

            let definitions = match compile_to_definitions(&inputs) {
                Ok(defs) => defs,
                Err(e) => {
                    eprintln!("Compilation failed: {}", e);
                    return ExitCode::FAILURE;
                }
            };

            if let Err(errors) = validate_bpmn_definitions(&definitions) {
                eprintln!("Compilation failed: Graph validation failed:\n{}", errors.join("\n"));
                return ExitCode::FAILURE;
            }

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
    }

    pub mod validate {
        use std::fs;
        use std::path::PathBuf;
        use std::process::ExitCode;

        use crate::features::graph_validation::use_cases::validate::validate_bpmn_definitions;
        #[cfg(feature = "xsd-validation")]
        use crate::transpiler::bpmn::validate_bpmn_xsd;
        use crate::transpiler::bpmn::{parse_bpmn, Validate};
        use crate::transpiler::mdx::MdxFile;

        pub fn run(files: Vec<PathBuf>) -> ExitCode {
            let mut has_errors = false;

            for path in files {
                match validate_file(&path) {
                    Ok(()) => println!("✓ {}", path.display()),
                    Err(e) => {
                        eprintln!("✗ {}: {}", path.display(), e);
                        has_errors = true;
                    }
                }
            }

            if has_errors {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }

        fn validate_file(path: &PathBuf) -> Result<(), String> {
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            match extension {
                "bpmn" | "bpmn2" => validate_bpmn(path),
                "mdx" => validate_mdx(path),
                _ => Err(format!("Unknown file type: .{}", extension)),
            }
        }

        fn validate_bpmn(path: &PathBuf) -> Result<(), String> {
            let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

            #[cfg(feature = "xsd-validation")]
            {
                validate_bpmn_xsd(&content).map_err(|e| format!("XSD validation failed: {}", e))?;
            }

            let defs = parse_bpmn(&content).map_err(|e| format!("Invalid BPMN: {}", e))?;
            defs.validate_for_bpmn()?;

            if let Err(errors) = validate_bpmn_definitions(&defs) {
                return Err(format!("Graph validation failed:\n{}", errors.join("\n")));
            }

            Ok(())
        }

        fn validate_mdx(path: &PathBuf) -> Result<(), String> {
            let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
            let mdx = MdxFile::parse(&content).map_err(|e| format!("Invalid MDX: {}", e))?;

            if let Ok(process) = mdx.parse_process() {
                return process.validate();
            }
            if let Ok(event) = mdx.parse_start_event() {
                return event.validate();
            }
            if let Ok(event) = mdx.parse_end_event() {
                return event.validate();
            }
            if let Ok(task) = mdx.parse_task() {
                return task.validate();
            }
            if let Ok(flow) = mdx.parse_sequence_flow() {
                return flow.validate();
            }

            Err("Could not parse frontmatter as a valid BPMN type".to_string())
        }
    }
}
