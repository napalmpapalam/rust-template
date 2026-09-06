//! Composition root: which surfaces a shape starts, and what stops them.

use anyhow::{Context as _, Result};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
{% if server %}
use {{crate_name}}_server::{probes, serve as serve_http};
{% endif %}
use crate::{cli::RunCmd, config::Config, signal, version, worker};

/// Starts the surfaces `cmd` names, then waits for a signal or the first exit.
pub(super) async fn run(cmd: RunCmd, config: Config) -> Result<()> {
    tracing::info!(version = %version::version_string(), ?cmd, "starting");

    let shutdown = CancellationToken::new();
    let mut tasks = JoinSet::new();
{% if server %}
    if cmd.serves_api() {
        let (server_config, token) = (config.server, shutdown.clone());
        tasks.spawn(async move {
            serve_http(&server_config, probes(), token)
                .await
                .context("http server")
        });
    }
{% endif %}
    if cmd.runs_workers() {
        let (worker_config, token) = (config.worker, shutdown.clone());
        tasks.spawn(async move { worker::run(&worker_config, token).await.context("worker") });
    }

    let outcome = supervise(&mut tasks, &shutdown).await;
    drain(tasks).await;
    outcome
}

/// Waits for a signal, or for the first task to return — whichever comes first.
///
/// Either way the token fires, so every other surface stops with it: a process
/// that lost half of what it was started for is not half-useful, it is a pod
/// that reports healthy while doing nothing.
async fn supervise(tasks: &mut JoinSet<Result<()>>, shutdown: &CancellationToken) -> Result<()> {
    let first = tokio::select! {
        () = signal::shutdown() => {
            tracing::info!("shutdown signal received, draining");
            None
        }
        joined = tasks.join_next() => joined,
    };

    shutdown.cancel();

    match first {
        Some(Ok(result)) => result,
        Some(Err(err)) => Err(err).context("a task panicked"),
        None => Ok(()),
    }
}

/// Gives the rest of the tasks their chance to finish after cancellation.
///
/// Failures here are logged, not returned: the process is already on its way
/// down, and the first one to fall over is the one worth reporting.
async fn drain(mut tasks: JoinSet<Result<()>>) {
    while let Some(joined) = tasks.join_next().await {
        match joined {
            Ok(Ok(())) => {}
            Ok(Err(err)) => {
                tracing::error!(error = %format_args!("{err:#}"), "task failed on the way out");
            }
            Err(err) => tracing::error!(%err, "task panicked on the way out"),
        }
    }
}
