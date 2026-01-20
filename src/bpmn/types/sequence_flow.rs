//! BPMN sequence flow type

use serde::{Deserialize, Serialize};

use super::{ConditionExpression, Documentation};

/// BPMN Sequence Flow (edge between nodes)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sequenceFlow")]
pub struct SequenceFlow {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@sourceRef")]
    pub source_ref: String,

    #[serde(rename = "@targetRef")]
    pub target_ref: String,

    #[serde(rename = "conditionExpression", skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<ConditionExpression>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}
