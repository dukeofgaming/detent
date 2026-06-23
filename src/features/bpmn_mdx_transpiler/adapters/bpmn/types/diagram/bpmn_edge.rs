use serde::{Deserialize, Serialize};

use super::{BPMNLabel, Waypoint};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BPMNEdge {
    #[serde(rename = "@id", alias = "id", default)]
    pub id: String,
    #[serde(rename = "@bpmnElement", alias = "bpmnElement", default)]
    pub bpmn_element: String,
    #[serde(rename = "waypoint", default)]
    pub waypoints: Vec<Waypoint>,
    #[serde(rename = "BPMNLabel", default, skip_serializing_if = "Option::is_none")]
    pub label: Option<BPMNLabel>,
}
