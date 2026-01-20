//! Process frontmatter for process definitions

use serde::{Deserialize, Serialize};

/// Frontmatter for a process definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessFrontmatter {
    /// Process identifier
    pub id: String,

    /// BPMN type (always "bpmn:process")
    #[serde(rename = "type")]
    pub process_type: String,

    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether the process is executable
    #[serde(rename = "isExecutable", skip_serializing_if = "Option::is_none")]
    pub is_executable: Option<bool>,

    /// Process type (Public, Private)
    #[serde(rename = "processType", skip_serializing_if = "Option::is_none")]
    pub process_type_attr: Option<String>,

    /// Documentation text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,

    /// List of flow element IDs in this process
    #[serde(rename = "flowElements", default, skip_serializing_if = "Vec::is_empty")]
    pub flow_elements: Vec<String>,
}
