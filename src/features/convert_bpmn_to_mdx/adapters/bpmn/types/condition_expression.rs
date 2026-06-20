//! Condition expression type for gateways

use serde::{Deserialize, Serialize};

/// Condition expression for gateways
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionExpression {
    #[serde(rename = "@xsi:type", alias = "xsi_type", skip_serializing_if = "Option::is_none")]
    pub xsi_type: Option<String>,

    #[serde(rename = "$text", alias = "text", default)]
    pub expression: String,
}
