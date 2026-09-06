# {{project-name}}

{{description}} See [README.md](README.md) for what it does and how to run it.

## Working here

- **Run things directly** — `cargo run -- run`, `cargo run -- config`. Never
  `cargo build` then execute the artifact; the run already compiles.
- **Before pushing:** `cargo fmt --all`, then
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then
  `cargo test --workspace --all-features`. That is exactly what CI runs, and
  what `.githooks/pre-commit` runs locally. CI adds `cargo doc` under
  `-D warnings` and `cargo audit`.
- **CI runs `--locked`**, so `Cargo.lock` is committed and a dependency bump is
  a deliberate edit, never a silent one.
- **Read [docs/design/README.md](docs/design/README.md) before changing a
  domain** — how each part works and why, one file per domain.

## Constraints that bite

- **Lints are deny-level**: `unwrap_used`, `expect_used`, `panic`,
  `indexing_slicing`, `cast_possible_truncation`, `cast_sign_loss`,
  `wildcard_imports`. `missing_docs` warns, and CI runs rustdoc with
  `-D warnings` — so every public item needs a doc comment. Test modules opt out
  with `#[allow(clippy::unwrap_used, clippy::expect_used)]`.
- **Release builds are `panic = "abort"`** — a panic takes the process down
  rather than unwinding one task. Another reason errors must be `Result`.
- **`anyhow` in the binary, `thiserror` in the crates.** The binary wants one
  error type and a context chain; a crate's caller wants variants it can match
  on. Never the other way round.
- **Config is `deny_unknown_fields`** — a typo in a key fails the process at
  boot instead of silently doing nothing.

## Layout

{% if layout == "workspace" %}Domain lives in `crates/`, composition in `src/`.

| Path | Role |
| --- | --- |
| `crates/core` | The vocabulary — validated types, no I/O. Rename it to the domain, add siblings. |
| `src/main.rs` | Thin entry point; everything testable lives in the library. |
| `src/cli/` | Argument parsing and one module per subcommand. |
| `src/config/` | One file per section. Loading order and env-var mapping in `load.rs`. |
{% if server %}| `src/server.rs` | Binds the socket, serves until the shutdown signal fires. |
| `src/status.rs` | Health and readiness probes. |
{% endif %}| `src/version.rs` | Build-time metadata from `build.rs`. |

Every crate's dependencies come from `[workspace.dependencies]` in the root
`Cargo.toml` with `{ workspace = true }`, and its lints from `[lints] workspace = true`.
{% else %}| Path | Role |
| --- | --- |
| `src/main.rs` | Thin entry point; everything testable lives in the library. |
| `src/cli/` | Argument parsing and one module per subcommand. |
| `src/config/` | One file per section. Loading order and env-var mapping in `load.rs`. |
{% if server %}| `src/server.rs` | Binds the socket, serves until the shutdown signal fires. |
| `src/status.rs` | Health and readiness probes. |
{% endif %}| `src/version.rs` | Build-time metadata from `build.rs`. |

The root `Cargo.toml` is a workspace of one: versions live in
`[workspace.dependencies]` and lints in `[workspace.lints]`, so adding a crate
under `crates/` later costs one line.
{% endif %}
