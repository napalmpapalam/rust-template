# syntax=docker/dockerfile:1.7
ARG RUST_VERSION=1.90

# --- planner: capture the dependency recipe ---------------------------------
FROM rust:${RUST_VERSION}-slim-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- builder: cook deps, then build the binary ------------------------------
FROM chef AS builder
# The build context has no .git, so the commit is passed in instead of read.
ARG GIT_COMMIT=unknown
ENV GIT_COMMIT=${GIT_COMMIT}
COPY --from=planner /app/recipe.json recipe.json
# Dependencies are their own layer: an edit to src/ never recompiles them.
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --bin {{project-name}}

# --- runtime: the binary and nothing else -----------------------------------
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home appuser
COPY --from=builder /app/target/release/{{project-name}} /usr/local/bin/{{project-name}}
USER appuser
{% if server %}EXPOSE 8080
{% endif %}ENTRYPOINT ["/usr/local/bin/{{project-name}}"]
CMD ["run"]
