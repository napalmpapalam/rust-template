//! An example background loop. Replace the body; keep the shape.

use anyhow::Result;
use tokio_util::sync::CancellationToken;

use crate::config::WorkerConfig;

/// Ticks until `shutdown` fires.
///
/// Returning takes the process down, so only an unrecoverable failure is `Err`.
pub async fn run(config: &WorkerConfig, shutdown: CancellationToken) -> Result<()> {
    let mut ticker = tokio::time::interval(config.interval);

    loop {
        tokio::select! {
            () = shutdown.cancelled() => break,
            _ = ticker.tick() => tracing::debug!("tick"),
        }
    }

    tracing::info!("worker stopped");
    Ok(())
}
