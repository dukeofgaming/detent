//! Node frontmatter for MDX files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{IoSpec, RetryConfig};

/// MDX frontmatter for a BPMN node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeFrontmatter {
    /// Unique identifier for this node
    pub id: String,

    /// BPMN type (e.g., "bpmn:startEvent", "bpmn:task")
    #[serde(rename = "type")]
    pub node_type: String,

    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// IDs of incoming sequence flows
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub incoming: Vec<String>,

    /// IDs of outgoing sequence flows
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outgoing: Vec<String>,

    /// Documentation text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,

    /// Service implementation type (for ServiceTask)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation: Option<String>,

    /// Service reference (for ServiceTask)
    #[serde(rename = "serviceRef", skip_serializing_if = "Option::is_none")]
    pub service_ref: Option<String>,

    /// Script format (for ScriptTask)
    #[serde(rename = "scriptFormat", skip_serializing_if = "Option::is_none")]
    pub script_format: Option<String>,

    /// Script content (for ScriptTask)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

    /// Gateway direction (Diverging, Converging, Mixed)
    #[serde(rename = "gatewayDirection", skip_serializing_if = "Option::is_none")]
    pub gateway_direction: Option<String>,

    /// Default flow for exclusive gateway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// Condition expressions for gateway outgoing flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<HashMap<String, String>>,

    /// IO mapping specification (for tasks)
    #[serde(rename = "ioSpec", skip_serializing_if = "Option::is_none")]
    pub io_spec: Option<IoSpec>,

    /// Retry configuration (for tasks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryConfig>,
}
