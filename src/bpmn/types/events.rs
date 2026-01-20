//! BPMN event types (StartEvent, EndEvent)

use serde::{Deserialize, Serialize};

use super::Documentation;

/// BPMN Start Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "startEvent")]
pub struct StartEvent {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(rename = "@name", alias = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// BPMN End Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "endEvent")]
pub struct EndEvent {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(rename = "@name", alias = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}
