//! Logging configuration section.

use serde::{Deserialize, Serialize};

/// Logging configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields, default)]
pub struct LogConfig {
    /// Filter directives, e.g. `info` or `info,{{crate_name}}=debug`.
    pub filter: String,
    /// Output format.
    pub format: LogFormat,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            filter: "info".into(),
            format: LogFormat::default(),
        }
    }
}

/// Log output format.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// Human-readable text (default).
    #[default]
    Text,
    /// Newline-delimited JSON, for log collectors.
    Json,
}
