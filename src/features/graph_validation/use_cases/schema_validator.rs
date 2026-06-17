pub trait SchemaValidator {
    fn validate_xml(&self, xml: &str) -> Result<(), String>;
}

pub struct NoopSchemaValidator;

impl SchemaValidator for NoopSchemaValidator {
    fn validate_xml(&self, _xml: &str) -> Result<(), String> {
        Ok(())
    }
}
