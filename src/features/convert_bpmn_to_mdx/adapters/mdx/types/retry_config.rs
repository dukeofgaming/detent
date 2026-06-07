//! Retry configuration for tasks

use serde::{Deserialize, Serialize};

use super::BackoffConfig;

/// Retry configuration for tasks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    #[serde(rename = "maxAttempts")]
    pub max_attempts: u32,

    /// Backoff configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff: Option<BackoffConfig>,
}
