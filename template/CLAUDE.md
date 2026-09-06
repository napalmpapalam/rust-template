# {{project-name}}

{{description}} See [README.md](README.md) for what it does and how to run it.

## Working here

- **Run things directly** — `cargo run -- run all`, `cargo run -- config`. Never
  `cargo build` then execute the artifact; the run already compiles.
- **Before pushing:** `cargo fmt --all`, then
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then
  `cargo test --workspace --all-features`.{% if ci != "none" %} That is exactly what CI runs, and
  what `.githooks/pre-commit` runs locally. CI adds `cargo doc` under
  `-D warnings` and `cargo audit`.
- **CI runs `--locked`**, so `Cargo.lock` is committed and a dependency bump is
  a deliberate edit, never a silent one.{% endif %}

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
- **A surface that returns takes the process with it.** `run/serve.rs` cancels
  the shared token on the first exit, so nothing is left running alone.

## Layout

Domain lives in `crates/`, composition in `src/`.

| Path | Role |
| --- | --- |
{% if server %}| `crates/server` | The HTTP surface: listener, probes, `ServerConfig`, shutdown. |
{% endif %}| `src/main.rs` | Thin entry point; everything testable lives in the library. |
| `src/cli/` | Argument parsing and one module per subcommand. |
| `src/cli/run/serve.rs` | The composition root — which surfaces a shape starts, and what stops them. |
| `src/config/` | One file per section. Loading order and env-var mapping in `load.rs`. |
| `src/worker.rs` | An example background loop. Replace the body; keep the shape. |
| `src/version.rs` | Build-time metadata from `build.rs`. |

Every crate's dependencies come from `[workspace.dependencies]` in the root
`Cargo.toml` with `{ workspace = true }`, and its lints from
`[lints] workspace = true`. A crate owns its own config section; `src/config/mod.rs`
only names it.

Which surface a shape runs is fixed in `cli/run/serve.rs`, never in config —
config controls how each surface behaves, not which ones exist.
