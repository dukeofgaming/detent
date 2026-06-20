use serde::{Deserialize, Serialize};

use super::{Bounds, BPMNLabel};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BPMNShape {
    #[serde(rename = "@id", alias = "id", default)]
    pub id: String,
    #[serde(rename = "@bpmnElement", alias = "bpmnElement", default)]
    pub bpmn_element: String,
    #[serde(rename = "Bounds")]
    pub bounds: Bounds,
    #[serde(rename = "BPMNLabel", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<BPMNLabel>,
}
