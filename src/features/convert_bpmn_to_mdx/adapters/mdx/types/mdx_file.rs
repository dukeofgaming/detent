//! MDX file parsing and serialization

use serde::de::DeserializeOwned;

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{
    EndEvent, ExclusiveGateway, ManualTask, ParallelGateway, Process, ScriptTask, SequenceFlow,
    ServiceTask, StartEvent, Task, UserTask,
};

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
        let end_pos = rest
            .find("\n---")
            .ok_or(MdxParseError::UnclosedFrontmatter)?;

        let frontmatter = rest[..end_pos].trim().to_string();
        let body = rest[end_pos + 4..].trim().to_string();

        Ok(Self { frontmatter, body })
    }

    /// Serialize to MDX string
    pub fn to_mdx_string(&self) -> String {
        format!("---\n{}\n---\n\n{}\n", self.frontmatter, self.body)
    }

    /// Generic method to parse frontmatter as any deserializable BPMN type
    ///
    /// # Example
    /// ```text
    /// use detent::features::convert_bpmn_to_mdx::adapters::mdx::{MdxFile, StartEvent};
    ///
    /// let mdx = MdxFile::parse("---\nid: start_1\noutgoing:\n- flow_1\n---\n").unwrap();
    /// let start_event: StartEvent = mdx.parse_as().unwrap();
    ///
    /// assert_eq!(start_event.id, "start_1");
    /// ```
    pub fn parse_as<T: DeserializeOwned>(&self) -> Result<T, serde_yaml::Error> {
        serde_yaml::from_str(&self.frontmatter)
    }

    // Convenience methods for common types (delegate to parse_as)

    /// Parse the frontmatter as a StartEvent
    pub fn parse_start_event(&self) -> Result<StartEvent, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as an EndEvent
    pub fn parse_end_event(&self) -> Result<EndEvent, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a Task
    pub fn parse_task(&self) -> Result<Task, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a ManualTask
    pub fn parse_manual_task(&self) -> Result<ManualTask, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a UserTask
    pub fn parse_user_task(&self) -> Result<UserTask, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a ServiceTask
    pub fn parse_service_task(&self) -> Result<ServiceTask, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a ScriptTask
    pub fn parse_script_task(&self) -> Result<ScriptTask, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a SequenceFlow
    pub fn parse_sequence_flow(&self) -> Result<SequenceFlow, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as an ExclusiveGateway
    pub fn parse_exclusive_gateway(&self) -> Result<ExclusiveGateway, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a ParallelGateway
    pub fn parse_parallel_gateway(&self) -> Result<ParallelGateway, serde_yaml::Error> {
        self.parse_as()
    }

    /// Parse the frontmatter as a Process
    pub fn parse_process(&self) -> Result<Process, serde_yaml::Error> {
        let mut value: serde_yaml::Value = serde_yaml::from_str(&self.frontmatter)?;
        if let serde_yaml::Value::Mapping(ref mut mapping) = value {
            mapping.remove(&serde_yaml::Value::String("type".to_string()));
            mapping.remove(&serde_yaml::Value::String("definitions".to_string()));
            mapping.remove(&serde_yaml::Value::String("diagram".to_string()));
        }
        serde_yaml::from_value(value)
    }

    /// Get raw frontmatter for generic parsing
    pub fn frontmatter(&self) -> &str {
        &self.frontmatter
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
