use axum::Router;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::{config::ServerConfig, error::ServerError};

/// Serves `router` until `shutdown` fires, then drains what is in flight.
///
/// # Errors
///
/// [`ServerError::Bind`] if the address is taken, [`ServerError::Serve`] if the
/// accept loop fails.
pub async fn serve(
    config: &ServerConfig,
    router: Router,
    shutdown: CancellationToken,
) -> Result<(), ServerError> {
    let addr = config.bind_addr;
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|source| ServerError::Bind { addr, source })?;

    tracing::info!(%addr, "listening");

    axum::serve(listener, router)
        .with_graceful_shutdown(async move { shutdown.cancelled().await })
        .await
        .map_err(ServerError::Serve)
}
