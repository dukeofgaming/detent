//! BPMN task types (Task, ServiceTask, ScriptTask)

use serde::{Deserialize, Serialize};

use super::Documentation;

/// Generic BPMN Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "task")]
pub struct Task {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN Service Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "serviceTask")]
pub struct ServiceTask {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@implementation", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN Script Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "scriptTask")]
pub struct ScriptTask {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@scriptFormat", skip_serializing_if = "Option::is_none")]
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
