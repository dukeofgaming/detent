use serde::{Deserialize, Serialize};

use super::{BPMNEdge, BPMNShape};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BPMNPlane {
    #[serde(rename = "@id", alias = "id", default)]
    pub id: String,
    #[serde(rename = "@bpmnElement", alias = "bpmnElement", default)]
    pub bpmn_element: String,
    #[serde(rename = "BPMNShape", default)]
    pub shapes: Vec<BPMNShape>,
    #[serde(rename = "BPMNEdge", default)]
    pub edges: Vec<BPMNEdge>,
}
