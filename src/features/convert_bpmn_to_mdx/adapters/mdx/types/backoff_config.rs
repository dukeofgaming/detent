//! Backoff configuration for retries

use serde::{Deserialize, Serialize};

/// Backoff configuration for retries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackoffConfig {
    /// Base delay in milliseconds
    #[serde(rename = "baseMs")]
    pub base_ms: u64,

    /// Multiplier for exponential backoff
    pub factor: f64,

    /// Whether to add jitter
    #[serde(default)]
    pub jitter: bool,
}
