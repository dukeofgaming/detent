use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bounds {
    #[serde(rename = "@x", alias = "x")]
    pub x: String,
    #[serde(rename = "@y", alias = "y")]
    pub y: String,
    #[serde(rename = "@width", alias = "width")]
    pub width: String,
    #[serde(rename = "@height", alias = "height")]
    pub height: String,
}
