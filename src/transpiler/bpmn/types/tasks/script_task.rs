//! BPMN Script Task type

use serde::{Deserialize, Serialize};

use crate::compiler::bpmn::Documentation;
use crate::compiler::bpmn::Validate;

/// BPMN Script Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "scriptTask")]
pub struct ScriptTask {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(
        rename = "@name",
        alias = "name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(
        rename = "@scriptFormat",
        alias = "scriptFormat",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_format: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

impl Validate for ScriptTask {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("ScriptTask must have an id".to_string());
        }
        Ok(())
    }
}
