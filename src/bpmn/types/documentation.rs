//! Documentation element type

use serde::{Deserialize, Serialize};

/// Documentation element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Documentation {
    #[serde(rename = "$text", default)]
    pub text: String,
}
