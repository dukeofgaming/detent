//! BPMN End Event type

use serde::{Deserialize, Serialize};

use crate::features::bpmn_mdx_transpiler::adapters::bpmn::Documentation;
use crate::features::bpmn_mdx_transpiler::adapters::bpmn::Validate;

/// BPMN End Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "endEvent")]
pub struct EndEvent {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(
        rename = "@name",
        alias = "name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

impl Validate for EndEvent {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("EndEvent must have an id".to_string());
        }
        Ok(())
    }
}
