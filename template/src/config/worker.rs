//! Background worker configuration section.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Wait between ticks when the config says nothing.
const DEFAULT_INTERVAL: Duration = Duration::from_secs(30);

/// How the background workers are paced.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields, default)]
pub struct WorkerConfig {
    /// Wait between ticks, written the way a human would: `30s`, `5m`.
    #[serde(with = "humantime_serde")]
    pub interval: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            interval: DEFAULT_INTERVAL,
        }
    }
}
