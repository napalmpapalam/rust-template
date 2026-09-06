//! Tracing setup.

use anyhow::{Context as _, Result};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::config::{LogConfig, LogFormat};

/// Installs the subscriber. Called once, before anything logs.
pub fn init(cfg: &LogConfig) -> Result<()> {
    let filter = EnvFilter::try_new(&cfg.filter).context("parsing log filter directive")?;
    let layer = match cfg.format {
        LogFormat::Json => fmt::layer().json().boxed(),
        LogFormat::Text => fmt::layer().boxed(),
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .try_init()
        .context("installing global tracing subscriber")
}
