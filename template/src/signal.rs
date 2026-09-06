//! Process lifetime: the signals that mean stop.

/// Resolves on Ctrl-C, or on SIGTERM where the platform has one.
///
/// A listener that cannot be installed pends forever — resolving would stop the
/// process the moment it started.
pub async fn shutdown() {
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
}
