//! BPMN sequence flow type

use serde::{Deserialize, Serialize};

use super::{ConditionExpression, Documentation};
use crate::transpiler::bpmn::Validate;

/// BPMN Sequence Flow (edge between nodes)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sequenceFlow")]
pub struct SequenceFlow {
    #[serde(rename = "@id", alias = "id")]
    pub id: String,

    #[serde(
        rename = "@name",
        alias = "name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(rename = "@sourceRef", alias = "sourceRef")]
    pub source_ref: String,

    #[serde(rename = "@targetRef", alias = "targetRef")]
    pub target_ref: String,

    #[serde(
        rename = "conditionExpression",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition_expression: Option<ConditionExpression>,

    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

impl Validate for SequenceFlow {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("SequenceFlow must have an id".to_string());
        }
        if self.source_ref.is_empty() {
            return Err("SequenceFlow must have a sourceRef".to_string());
        }
        if self.target_ref.is_empty() {
            return Err("SequenceFlow must have a targetRef".to_string());
        }
        Ok(())
    }
}
