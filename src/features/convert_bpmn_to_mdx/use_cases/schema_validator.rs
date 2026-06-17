pub trait SchemaValidator {
    fn validate_xml(&self, xml: &str) -> Result<(), String>;
}
