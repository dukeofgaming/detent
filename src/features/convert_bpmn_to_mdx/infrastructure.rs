pub mod cli {
    pub mod compile;
    pub mod import;
    pub mod validate;
}

#[cfg(feature = "xsd-validation")]
pub mod xsd_validator;

#[cfg(feature = "xsd-validation")]
pub use xsd_validator::{validate_bpmn_file_xsd, validate_bpmn_xsd, XsdValidationError};
