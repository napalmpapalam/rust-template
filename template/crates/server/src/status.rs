use axum::{Router, http::StatusCode, routing::get};

/// Builds the probe routes: `GET /health` and `GET /ready`.
pub fn probes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
}

/// Answers `200` for as long as the process is up.
async fn health() -> StatusCode {
    StatusCode::OK
}

/// Answers `200` once the service can take traffic.
///
/// Check the dependencies that must be up — a pool, a queue — and answer `503`
/// until they are.
async fn ready() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::probes;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt as _;

    async fn get(path: &str) -> StatusCode {
        probes()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status()
    }

    #[tokio::test]
    async fn health_is_ok() {
        assert_eq!(get("/health").await, StatusCode::OK);
    }

    #[tokio::test]
    async fn ready_is_ok() {
        assert_eq!(get("/ready").await, StatusCode::OK);
    }

    #[tokio::test]
    async fn an_unknown_path_is_not_found() {
        assert_eq!(get("/nope").await, StatusCode::NOT_FOUND);
    }
}
