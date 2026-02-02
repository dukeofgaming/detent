//! XSD Schema Validation for BPMN files
//!
//! This module provides XSD validation using libxml2 bindings.
//! It is only available when the `xsd-validation` feature is enabled.

use libxml::parser::Parser;
use libxml::schemas::{SchemaParserContext, SchemaValidationContext};
use std::path::Path;

/// Errors that can occur during XSD validation
#[derive(Debug)]
pub enum XsdValidationError {
    /// Failed to read the file
    FileReadError(String),
    /// Failed to parse the XML
    XmlParseError(String),
    /// Failed to parse the XSD schema
    SchemaParseError(Vec<String>),
    /// XML does not conform to XSD schema
    ValidationError(Vec<String>),
}

impl std::fmt::Display for XsdValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XsdValidationError::FileReadError(msg) => write!(f, "File read error: {}", msg),
            XsdValidationError::XmlParseError(msg) => write!(f, "XML parse error: {}", msg),
            XsdValidationError::SchemaParseError(errors) => {
                write!(f, "Schema parse errors:\n")?;
                for err in errors {
                    write!(f, "  - {}", err)?;
                }
                Ok(())
            }
            XsdValidationError::ValidationError(errors) => {
                write!(f, "XSD validation errors:\n")?;
                for err in errors {
                    write!(f, "  - {}", err)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for XsdValidationError {}

/// Get the path to the bundled BPMN20.xsd schema
fn get_bpmn_schema_path() -> &'static str {
    // The schema files are in src/assets/schemas/ relative to the crate root
    // We need to use the CARGO_MANIFEST_DIR at compile time
    concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/schemas/BPMN20.xsd")
}

/// Validate a BPMN XML string against the BPMN 2.0 XSD schema
///
/// # Arguments
/// * `xml_content` - The BPMN XML content as a string
///
/// # Returns
/// * `Ok(())` if validation succeeds
/// * `Err(XsdValidationError)` if validation fails
pub fn validate_bpmn_xsd(xml_content: &str) -> Result<(), XsdValidationError> {
    // Parse the XML document
    let parser = Parser::default();
    let doc = parser
        .parse_string(xml_content)
        .map_err(|e| XsdValidationError::XmlParseError(format!("{:?}", e)))?;

    // Load and parse the BPMN XSD schema
    let schema_path = get_bpmn_schema_path();
    let mut schema_parser = SchemaParserContext::from_file(schema_path);
    let schema_ctx = SchemaValidationContext::from_parser(&mut schema_parser).map_err(|errors| {
        XsdValidationError::SchemaParseError(
            errors
                .iter()
                .filter_map(|e| e.message.clone())
                .collect(),
        )
    })?;

    // Validate the document against the schema
    let mut validator = schema_ctx;
    validator.validate_document(&doc).map_err(|errors| {
        XsdValidationError::ValidationError(
            errors
                .iter()
                .filter_map(|e| e.message.clone())
                .collect(),
        )
    })?;

    Ok(())
}

/// Validate a BPMN file at the given path against the BPMN 2.0 XSD schema
///
/// # Arguments
/// * `path` - Path to the BPMN file
///
/// # Returns
/// * `Ok(())` if validation succeeds
/// * `Err(XsdValidationError)` if validation fails
pub fn validate_bpmn_file_xsd(path: &Path) -> Result<(), XsdValidationError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| XsdValidationError::FileReadError(e.to_string()))?;
    validate_bpmn_xsd(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bpmn_passes_xsd_validation() {
        let valid_bpmn = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI"
             id="Definitions_1"
             targetNamespace="http://example.com/bpmn">
  <process id="Process_1" isExecutable="true">
    <startEvent id="StartEvent_1"/>
    <endEvent id="EndEvent_1"/>
    <sequenceFlow id="Flow_1" sourceRef="StartEvent_1" targetRef="EndEvent_1"/>
  </process>
</definitions>"#;

        let result = validate_bpmn_xsd(valid_bpmn);
        assert!(result.is_ok(), "Expected valid BPMN to pass XSD validation: {:?}", result);
    }

    #[test]
    fn test_invalid_element_fails_xsd_validation() {
        let invalid_bpmn = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             id="Definitions_1"
             targetNamespace="http://example.com/bpmn">
  <invalidElement id="Bad_1"/>
</definitions>"#;

        let result = validate_bpmn_xsd(invalid_bpmn);
        assert!(result.is_err(), "Expected invalid BPMN to fail XSD validation");
    }

    #[test]
    fn test_missing_required_attribute_fails_xsd_validation() {
        // targetNamespace is required
        let invalid_bpmn = r#"<?xml version="1.0" encoding="UTF-8"?>
<definitions xmlns="http://www.omg.org/spec/BPMN/20100524/MODEL"
             id="Definitions_1">
  <process id="Process_1" isExecutable="true"/>
</definitions>"#;

        let result = validate_bpmn_xsd(invalid_bpmn);
        assert!(result.is_err(), "Expected BPMN without targetNamespace to fail XSD validation");
    }
}
