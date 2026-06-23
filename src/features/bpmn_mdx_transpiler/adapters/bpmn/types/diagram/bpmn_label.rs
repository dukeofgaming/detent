use serde::{Deserialize, Serialize};

use super::Bounds;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BPMNLabel {
    #[serde(rename = "Bounds")]
    pub bounds: Bounds,
}
