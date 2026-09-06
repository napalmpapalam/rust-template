{% if server %}use anyhow::Result;
{% else %}use anyhow::{Context as _, Result};
{% endif %}use clap::Args;

{% if server %}use crate::{config::Config, server, version};
{% else %}use crate::{config::Config, version};
{% endif %}
/// `run` — the service itself.
#[derive(Debug, PartialEq, Args)]
pub struct RunCmd {}

impl RunCmd {
{% if server %}    /// Serves until the process is asked to stop.
    ///
    /// # Errors
    ///
    /// If the socket cannot be bound, or the accept loop fails.
    pub async fn execute(self, config: Config) -> Result<()> {
        tracing::info!(version = %version::version_string(), name = %config.name, "starting");

        server::serve(&config).await
    }
{% else %}    /// Runs until the process is asked to stop.
    ///
    /// # Errors
    ///
    /// If the signal handler cannot be installed.
    pub async fn execute(self, config: Config) -> Result<()> {
        tracing::info!(version = %version::version_string(), name = %config.name, "starting");

        // Replace this with the work.
        tokio::signal::ctrl_c()
            .await
            .context("waiting for Ctrl-C")?;

        tracing::info!("stopped");
        Ok(())
    }
{% endif %}}
