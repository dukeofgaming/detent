//! XSD Schema Validation for BPMN files
//!
//! This module provides XSD validation using libxml2 bindings.
//! It is only available when the `xsd-validation` feature is enabled.

use crate::features::graph_validation::use_cases::schema_validator::SchemaValidator;
use libxml::parser::Parser;
use libxml::schemas::{SchemaParserContext, SchemaValidationContext};
use std::path::Path;

pub struct LibxmlSchemaValidator;

impl SchemaValidator for LibxmlSchemaValidator {
    fn validate_xml(&self, xml: &str) -> Result<(), String> {
        validate_bpmn_xsd(xml).map_err(|e| e.to_string())
    }
}

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
    let schema_ctx =
        SchemaValidationContext::from_parser(&mut schema_parser).map_err(|errors| {
            XsdValidationError::SchemaParseError(
                errors.iter().filter_map(|e| e.message.clone()).collect(),
            )
        })?;

    // Validate the document against the schema
    let mut validator = schema_ctx;
    validator.validate_document(&doc).map_err(|errors| {
        XsdValidationError::ValidationError(
            errors.iter().filter_map(|e| e.message.clone()).collect(),
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
