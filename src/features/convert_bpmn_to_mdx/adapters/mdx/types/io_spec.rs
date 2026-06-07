//! IO mapping specification

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// IO mapping specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IoSpec {
    /// Input mapping (JMESPath expressions)
    #[serde(rename = "inputMapping", skip_serializing_if = "Option::is_none")]
    pub input_mapping: Option<HashMap<String, String>>,

    /// Output mapping (JMESPath expressions)
    #[serde(rename = "outputMapping", skip_serializing_if = "Option::is_none")]
    pub output_mapping: Option<HashMap<String, String>>,
}
