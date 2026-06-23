use serde::{Deserialize, Serialize};

use super::BPMNPlane;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BPMNDiagram {
    #[serde(rename = "@id", alias = "id", default)]
    pub id: String,
    #[serde(rename = "BPMNPlane")]
    pub plane: BPMNPlane,
}
