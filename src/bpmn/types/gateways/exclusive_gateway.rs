//! BPMN Exclusive Gateway type

use serde::{Deserialize, Serialize};

use crate::bpmn::Documentation;

/// BPMN Exclusive Gateway (XOR)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "exclusiveGateway")]
pub struct ExclusiveGateway {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(
        rename = "@name",
        alias = "name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(
        rename = "@gatewayDirection",
        alias = "gatewayDirection",
        skip_serializing_if = "Option::is_none"
    )]
    pub gateway_direction: Option<String>,

    #[serde(
        rename = "@default",
        alias = "default",
        skip_serializing_if = "Option::is_none"
    )]
    pub default: Option<String>,

    #[serde(rename = "incoming", default)]
    pub incoming: Vec<String>,

    #[serde(rename = "outgoing", default)]
    pub outgoing: Vec<String>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}
