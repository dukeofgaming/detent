use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    #[serde(rename = "@x", alias = "x")]
    pub x: String,
    #[serde(rename = "@y", alias = "y")]
    pub y: String,
}
