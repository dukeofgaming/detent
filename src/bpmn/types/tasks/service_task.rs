//! BPMN Service Task type

use serde::{Deserialize, Serialize};

use crate::bpmn::Documentation;

/// BPMN Service Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "serviceTask")]
pub struct ServiceTask {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(
        rename = "@name",
        alias = "name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(
        rename = "@implementation",
        alias = "implementation",
        skip_serializing_if = "Option::is_none"
    )]
    pub implementation: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}
