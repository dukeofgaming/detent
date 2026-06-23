use crate::features::bpmn_mdx_transpiler::adapters::bpmn::parse_bpmn;
use crate::features::graph_validation::application::validate::validate_bpmn_definitions;

use super::schema_validator::SchemaValidator;

pub fn validate_bpmn_string(content: &str, validator: &impl SchemaValidator) -> Result<(), String> {
    validator.validate_xml(content)?;
    let defs = parse_bpmn(content).map_err(|e| format!("Invalid BPMN: {}", e))?;
    defs.validate_for_bpmn()?;
    validate_bpmn_definitions(&defs).map_err(|errors| errors.join("\n"))?;
    Ok(())
}
