//! Binds the socket and serves until the process is asked to stop.

use anyhow::{Context as _, Result};
use tokio::net::TcpListener;

use crate::{config::Config, status};

/// Serves until Ctrl-C or SIGTERM, then drains what is in flight.
///
/// # Errors
///
/// If the address is already taken, or the accept loop fails.
pub async fn serve(config: &Config) -> Result<()> {
    let addr = config.server.bind_addr;
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("binding {addr}"))?;

    tracing::info!(%addr, "listening");

    axum::serve(listener, status::routes())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("serving")
}

/// Resolves on Ctrl-C, or on SIGTERM where the platform has one.
///
/// A listener that cannot be installed pends forever rather than resolving:
/// returning immediately would shut the process down the moment it started.
async fn shutdown_signal() {
    let interrupt = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!(%err, "cannot listen for Ctrl-C");
            std::future::pending::<()>().await;
        }
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};

        let Ok(mut sigterm) = signal(SignalKind::terminate())
            .inspect_err(|err| tracing::error!(%err, "cannot listen for SIGTERM"))
        else {
            return std::future::pending().await;
        };

        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = interrupt => {}
        () = terminate => {}
    }

    tracing::info!("shutdown signal received, draining");
}
