//! BPMN Start Event type

use serde::{Deserialize, Serialize};

use crate::features::convert_bpmn_to_mdx::adapters::bpmn::Documentation;
use crate::features::convert_bpmn_to_mdx::adapters::bpmn::Validate;

/// BPMN Start Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "startEvent")]
pub struct StartEvent {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(
        rename = "@name",
        alias = "name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

impl Validate for StartEvent {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("StartEvent must have an id".to_string());
        }
        Ok(())
    }
}
