mod serve;

use anyhow::Result;
use clap::Subcommand;

use crate::config::Config;

/// What this process runs — one variant per deployment shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub enum RunCmd {
{% if server %}    /// Serve the HTTP API and nothing else.
    Api,

{% endif %}    /// Run the background workers and nothing else.
    Workers,

    /// Run everything in one process.
    All,
}

impl RunCmd {
{% if server %}    /// Whether this shape serves HTTP.
    pub(super) fn serves_api(self) -> bool {
        matches!(self, Self::Api | Self::All)
    }

{% endif %}    /// Whether this shape runs the background workers.
    pub(super) fn runs_workers(self) -> bool {
        matches!(self, Self::Workers | Self::All)
    }

    /// Starts what this shape names and waits for it to stop.
    ///
    /// # Errors
    ///
    /// The first failure any started surface reports.
    pub async fn execute(self, config: Config) -> Result<()> {
        serve::run(self, config).await
    }
}
