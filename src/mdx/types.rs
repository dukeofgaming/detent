//! MDX types for BPMN node representation
//!
//! These types represent the YAML frontmatter structure used in MDX files.
//! The frontmatter is a YAML representation of BPMN elements, designed for
//! human readability while maintaining full round-trip fidelity with BPMN XML.

use serde::{Deserialize, Serialize};

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

    // Task-specific fields
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

    // Gateway-specific fields
    /// Gateway direction (Diverging, Converging, Mixed)
    #[serde(rename = "gatewayDirection", skip_serializing_if = "Option::is_none")]
    pub gateway_direction: Option<String>,

    /// Default flow for exclusive gateway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// Condition expressions for gateway outgoing flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<std::collections::HashMap<String, String>>,

    /// IO mapping specification (for tasks)
    #[serde(rename = "ioSpec", skip_serializing_if = "Option::is_none")]
    pub io_spec: Option<IoSpec>,

    /// Retry configuration (for tasks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryConfig>,
}

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

/// IO mapping specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IoSpec {
    /// Input mapping (JMESPath expressions)
    #[serde(rename = "inputMapping", skip_serializing_if = "Option::is_none")]
    pub input_mapping: Option<std::collections::HashMap<String, String>>,

    /// Output mapping (JMESPath expressions)
    #[serde(rename = "outputMapping", skip_serializing_if = "Option::is_none")]
    pub output_mapping: Option<std::collections::HashMap<String, String>>,
}

/// Retry configuration for tasks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    #[serde(rename = "maxAttempts")]
    pub max_attempts: u32,

    /// Backoff configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff: Option<BackoffConfig>,
}

/// Backoff configuration for retries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackoffConfig {
    /// Base delay in milliseconds
    #[serde(rename = "baseMs")]
    pub base_ms: u64,

    /// Multiplier for exponential backoff
    pub factor: f64,

    /// Whether to add jitter
    #[serde(default)]
    pub jitter: bool,
}

/// Complete MDX file structure
#[derive(Debug, Clone)]
pub struct MdxFile {
    /// YAML frontmatter
    pub frontmatter: String,
    
    /// Markdown body content
    pub body: String,
}

impl MdxFile {
    /// Parse an MDX file from string content
    pub fn parse(content: &str) -> Result<Self, MdxParseError> {
        let content = content.trim();
        
        // Check for frontmatter delimiter
        if !content.starts_with("---") {
            return Err(MdxParseError::MissingFrontmatter);
        }
        
        // Find the closing delimiter
        let rest = &content[3..];
        let end_pos = rest.find("\n---")
            .ok_or(MdxParseError::UnclosedFrontmatter)?;
        
        let frontmatter = rest[..end_pos].trim().to_string();
        let body = rest[end_pos + 4..].trim().to_string();
        
        Ok(Self { frontmatter, body })
    }

    /// Serialize to MDX string
    pub fn to_string(&self) -> String {
        format!("---\n{}\n---\n\n{}\n", self.frontmatter, self.body)
    }

    /// Parse the frontmatter as a node
    pub fn parse_node_frontmatter(&self) -> Result<NodeFrontmatter, serde_yaml::Error> {
        serde_yaml::from_str(&self.frontmatter)
    }

    /// Parse the frontmatter as a flow
    pub fn parse_flow_frontmatter(&self) -> Result<FlowFrontmatter, serde_yaml::Error> {
        serde_yaml::from_str(&self.frontmatter)
    }

    /// Parse the frontmatter as a process
    pub fn parse_process_frontmatter(&self) -> Result<ProcessFrontmatter, serde_yaml::Error> {
        serde_yaml::from_str(&self.frontmatter)
    }
}

/// Errors that can occur when parsing MDX
#[derive(Debug, Clone, PartialEq)]
pub enum MdxParseError {
    MissingFrontmatter,
    UnclosedFrontmatter,
}

impl std::fmt::Display for MdxParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MdxParseError::MissingFrontmatter => write!(f, "MDX file must start with ---"),
            MdxParseError::UnclosedFrontmatter => write!(f, "Frontmatter must end with ---"),
        }
    }
}

impl std::error::Error for MdxParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mdx_file() {
        let content = r#"---
id: start_1
type: bpmn:startEvent
outgoing:
  - flow_1
---

# Start Event

This is the start of the process.
"#;

        let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
        assert!(mdx.frontmatter.contains("id: start_1"));
        assert!(mdx.body.contains("Start Event"));
    }

    #[test]
    fn test_parse_node_frontmatter() {
        let content = r#"---
id: task_1
type: bpmn:serviceTask
name: Gather Profile
incoming:
  - flow_1
outgoing:
  - flow_2
implementation: service
serviceRef: onboarding.gatherProfile
---

# Task
"#;

        let mdx = MdxFile::parse(content).expect("Failed to parse MDX");
        let node = mdx.parse_node_frontmatter().expect("Failed to parse frontmatter");
        
        assert_eq!(node.id, "task_1");
        assert_eq!(node.node_type, "bpmn:serviceTask");
        assert_eq!(node.name, Some("Gather Profile".to_string()));
        assert_eq!(node.incoming, vec!["flow_1"]);
        assert_eq!(node.outgoing, vec!["flow_2"]);
        assert_eq!(node.implementation, Some("service".to_string()));
        assert_eq!(node.service_ref, Some("onboarding.gatherProfile".to_string()));
    }

    #[test]
    fn test_roundtrip_frontmatter() {
        let node = NodeFrontmatter {
            id: "start_1".to_string(),
            node_type: "bpmn:startEvent".to_string(),
            name: None,
            incoming: vec![],
            outgoing: vec!["flow_1".to_string()],
            documentation: None,
            implementation: None,
            service_ref: None,
            script_format: None,
            script: None,
            gateway_direction: None,
            default: None,
            conditions: None,
            io_spec: None,
            retry: None,
        };

        let yaml = serde_yaml::to_string(&node).expect("Failed to serialize");
        let parsed: NodeFrontmatter = serde_yaml::from_str(&yaml).expect("Failed to parse");
        
        assert_eq!(node, parsed);
    }
}
