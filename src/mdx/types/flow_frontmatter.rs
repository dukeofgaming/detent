//! Flow frontmatter for sequence flows

use serde::{Deserialize, Serialize};

/// Frontmatter for a sequence flow
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowFrontmatter {
    /// Unique identifier for this flow
    pub id: String,

    /// BPMN type (always "bpmn:sequenceFlow")
    #[serde(rename = "type")]
    pub flow_type: String,

    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Source node ID
    #[serde(rename = "sourceRef")]
    pub source_ref: String,

    /// Target node ID
    #[serde(rename = "targetRef")]
    pub target_ref: String,

    /// Condition expression (for conditional flows)
    #[serde(rename = "conditionExpression", skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
}
