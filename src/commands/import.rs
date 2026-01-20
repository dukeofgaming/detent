//! Import command implementation - BPMN to MDX conversion
//!
//! Converts BPMN XML files to MDX files with YAML frontmatter.
//! The frontmatter uses the exact same structure as BPMN types.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use detent::bpmn::parse_bpmn;
use serde::Serialize;

/// Run the import command
pub fn run(bpmn_file: PathBuf, output_directory: PathBuf) -> ExitCode {
    // Read and parse BPMN file
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

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all(&output_directory) {
        eprintln!(
            "Failed to create output directory {}: {}",
            output_directory.display(),
            e
        );
        return ExitCode::FAILURE;
    }

    // Get process
    let process = match &definitions.process {
        Some(p) => p,
        None => {
            eprintln!("No process found in BPMN file");
            return ExitCode::FAILURE;
        }
    };

    // Generate MDX files for individual flow elements
    // (Process metadata is not duplicated - the individual element files are the canonical definitions)
    let mut generated_count = 0;

    // Generate node MDX files
    for event in &process.start_events {
        if let Err(e) = write_mdx(&event.id, "bpmn:startEvent", event, &output_directory) {
            eprintln!("Failed to generate startEvent MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for event in &process.end_events {
        if let Err(e) = write_mdx(&event.id, "bpmn:endEvent", event, &output_directory) {
            eprintln!("Failed to generate endEvent MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for task in &process.tasks {
        if let Err(e) = write_mdx(&task.id, "bpmn:task", task, &output_directory) {
            eprintln!("Failed to generate task MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for task in &process.service_tasks {
        if let Err(e) = write_mdx(&task.id, "bpmn:serviceTask", task, &output_directory) {
            eprintln!("Failed to generate serviceTask MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for task in &process.script_tasks {
        if let Err(e) = write_mdx(&task.id, "bpmn:scriptTask", task, &output_directory) {
            eprintln!("Failed to generate scriptTask MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for gateway in &process.exclusive_gateways {
        if let Err(e) = write_mdx(&gateway.id, "bpmn:exclusiveGateway", gateway, &output_directory) {
            eprintln!("Failed to generate exclusiveGateway MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    for gateway in &process.parallel_gateways {
        if let Err(e) = write_mdx(&gateway.id, "bpmn:parallelGateway", gateway, &output_directory) {
            eprintln!("Failed to generate parallelGateway MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    // Generate sequence flow MDX files
    for flow in &process.sequence_flows {
        if let Err(e) = write_mdx(&flow.id, "bpmn:sequenceFlow", flow, &output_directory) {
            eprintln!("Failed to generate sequenceFlow MDX: {}", e);
            return ExitCode::FAILURE;
        }
        generated_count += 1;
    }

    println!(
        "Generated {} MDX files in {}",
        generated_count,
        output_directory.display()
    );
    ExitCode::SUCCESS
}

/// Clean YAML output by stripping @ and $ prefixes that come from quick-xml conventions
/// Also removes unnecessary quotes around keys
fn clean_yaml_for_mdx(yaml: &str) -> String {
    use regex::Regex;
    
    // Pattern to match quoted keys with @ prefix: '@key': or "@key":
    let re_single_at = Regex::new(r"'@([a-zA-Z_][a-zA-Z0-9_]*)':").unwrap();
    let re_double_at = Regex::new(r#""@([a-zA-Z_][a-zA-Z0-9_]*)":"#).unwrap();
    
    // Pattern for $ prefix (quoted and unquoted)
    let re_single_dollar = Regex::new(r"'\$([a-zA-Z_][a-zA-Z0-9_]*)':").unwrap();
    let re_double_dollar = Regex::new(r#""\$([a-zA-Z_][a-zA-Z0-9_]*)":"#).unwrap();
    // Also handle unquoted $text: at start of line or after whitespace
    let re_unquoted_dollar = Regex::new(r"(\s)\$([a-zA-Z_][a-zA-Z0-9_]*):").unwrap();
    
    let result = re_single_at.replace_all(yaml, "$1:");
    let result = re_double_at.replace_all(&result, "$1:");
    let result = re_single_dollar.replace_all(&result, "$1:");
    let result = re_double_dollar.replace_all(&result, "$1:");
    let result = re_unquoted_dollar.replace_all(&result, "$1$2:");
    
    result.to_string()
}

/// Write any serializable BPMN type as MDX frontmatter
fn write_mdx<T: Serialize>(
    id: &str,
    bpmn_type: &str,
    data: &T,
    output_dir: &PathBuf,
) -> Result<(), std::io::Error> {
    let yaml = serde_yaml::to_string(data).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("YAML error: {}", e))
    })?;

    // Clean the YAML to remove @ and $ prefixes
    let clean_yaml = clean_yaml_for_mdx(&yaml);

    // Add type field at the beginning (after the first line which may be an id)
    let mdx_content = format!("---\ntype: {}\n{}---\n", bpmn_type, clean_yaml);

    let file_path = output_dir.join(format!("{}.mdx", id));
    fs::write(file_path, mdx_content)
}
