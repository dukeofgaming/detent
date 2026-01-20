//! MDX file parsing and serialization

use super::{FlowFrontmatter, NodeFrontmatter, ProcessFrontmatter};

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
