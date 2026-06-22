//! MDX file parsing and serialization

use serde::de::DeserializeOwned;

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::{
    EndEvent, ExclusiveGateway, ManualTask, ParallelGateway, Process, ScriptTask, SequenceFlow,
    ServiceTask, StartEvent, Task, UserTask, Validate,
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
            mapping.remove(serde_yaml::Value::String("type".to_string()));
            mapping.remove(serde_yaml::Value::String("definitions".to_string()));
            mapping.remove(serde_yaml::Value::String("diagram".to_string()));
        }
        serde_yaml::from_value(value)
    }

    /// Validate frontmatter by dispatching on the `type` field.
    pub fn validate(&self) -> Result<(), String> {
        let value: serde_yaml::Value = serde_yaml::from_str(&self.frontmatter)
            .map_err(|e| format!("Invalid frontmatter YAML: {}", e))?;
        let bpmn_type = value
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Frontmatter must include a type field".to_string())?;

        match bpmn_type {
            "bpmn:process" => self
                .parse_process()
                .map_err(|e| format!("Invalid process frontmatter: {}", e))?
                .validate(),
            "bpmn:startEvent" => self
                .parse_start_event()
                .map_err(|e| format!("Invalid start event frontmatter: {}", e))?
                .validate(),
            "bpmn:endEvent" => self
                .parse_end_event()
                .map_err(|e| format!("Invalid end event frontmatter: {}", e))?
                .validate(),
            "bpmn:task" => self
                .parse_task()
                .map_err(|e| format!("Invalid task frontmatter: {}", e))?
                .validate(),
            "bpmn:manualTask" => self
                .parse_manual_task()
                .map_err(|e| format!("Invalid manual task frontmatter: {}", e))?
                .validate(),
            "bpmn:userTask" => self
                .parse_user_task()
                .map_err(|e| format!("Invalid user task frontmatter: {}", e))?
                .validate(),
            "bpmn:serviceTask" => self
                .parse_service_task()
                .map_err(|e| format!("Invalid service task frontmatter: {}", e))?
                .validate(),
            "bpmn:scriptTask" => self
                .parse_script_task()
                .map_err(|e| format!("Invalid script task frontmatter: {}", e))?
                .validate(),
            "bpmn:sequenceFlow" => self
                .parse_sequence_flow()
                .map_err(|e| format!("Invalid sequence flow frontmatter: {}", e))?
                .validate(),
            "bpmn:exclusiveGateway" => self
                .parse_exclusive_gateway()
                .map_err(|e| format!("Invalid exclusive gateway frontmatter: {}", e))?
                .validate(),
            "bpmn:parallelGateway" => self
                .parse_parallel_gateway()
                .map_err(|e| format!("Invalid parallel gateway frontmatter: {}", e))?
                .validate(),
            other => Err(format!("Unknown BPMN type: {other}")),
        }
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
