use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn;
use crate::features::convert_bpmn_to_mdx::application::import::import_to_mdx;

pub fn run(bpmn_file: PathBuf, output_directory: PathBuf) -> ExitCode {
    let xml = match fs::read_to_string(&bpmn_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {}: {}", bpmn_file.display(), e);
            return ExitCode::FAILURE;
        }
    };

    let definitions = match parse_bpmn(&xml) {
        Ok(defs) => defs,
        Err(e) => {
            eprintln!("Failed to parse BPMN: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = fs::create_dir_all(&output_directory) {
        eprintln!(
            "Failed to create output directory {}: {}",
            output_directory.display(),
            e
        );
        return ExitCode::FAILURE;
    }

    let outputs = match import_to_mdx(&definitions) {
        Ok(outputs) => outputs,
        Err(e) => {
            eprintln!("Import failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    for output in &outputs {
        let file_path = output_directory.join(&output.filename);
        if let Err(e) = fs::write(&file_path, &output.content) {
            eprintln!("Failed to write {}: {}", file_path.display(), e);
            return ExitCode::FAILURE;
        }
    }

    println!(
        "Generated {} MDX files in {}",
        outputs.len(),
        output_directory.display()
    );
    ExitCode::SUCCESS
}
