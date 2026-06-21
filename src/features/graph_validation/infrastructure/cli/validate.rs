use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[cfg(feature = "xsd-validation")]
use crate::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn;
#[cfg(feature = "xsd-validation")]
use crate::features::convert_bpmn_to_mdx::infrastructure::validate_bpmn_xsd;
use crate::features::convert_bpmn_to_mdx::adapters::mdx::MdxFile;
use crate::features::graph_validation::use_cases::schema_validator::NoopSchemaValidator;
#[cfg(feature = "xsd-validation")]
use crate::features::graph_validation::use_cases::validate::validate_bpmn_definitions;
use crate::features::graph_validation::use_cases::validate_workflow;

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
        validate_bpmn_xsd(&content).map_err(|e| e.to_string())?;
        let defs = parse_bpmn(&content).map_err(|e| format!("Invalid BPMN: {}", e))?;
        defs.validate_for_bpmn()?;
        return validate_bpmn_definitions(&defs).map_err(|errors| errors.join("\n"));
    }

    validate_workflow::validate_bpmn_string(&content, &NoopSchemaValidator)
}

fn validate_mdx(path: &PathBuf) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let mdx = MdxFile::parse(&content).map_err(|e| format!("Invalid MDX: {}", e))?;
    mdx.validate()
}
