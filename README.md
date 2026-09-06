# rust-template

[![CI](https://github.com/napalmpapalam/rust-template/actions/workflows/ci.yml/badge.svg)](https://github.com/napalmpapalam/rust-template/actions/workflows/ci.yml)
[![cargo-generate](https://img.shields.io/badge/cargo--generate-template-000?logo=rust)](https://cargo-generate.github.io/cargo-generate/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A `cargo generate` template for Rust binaries and services. Answer seven
questions, get a project that compiles, tests and lints on the first run —
workspace lints, layered config, tracing, a supervised runtime, CI, a Dockerfile
and git hooks already wired together.

![The wizard generating a project](assets/demo.gif)

## Prerequisites

- **Rust 1.90+** — [rustup.rs](https://rustup.rs)
- **cargo-generate** — `cargo install cargo-generate --locked`

## Usage

```sh
cargo generate --git https://github.com/napalmpapalam/rust-template
```

> [!NOTE]
> The `napalmpapalam/rust-template` shorthand works too, but it is looked up as
> a favorite first and warns when it finds none. `--git` skips that.

Then:

```sh
cd <your-project>
cargo run -- run all
```

Every setting has a default, so nothing needs configuring to boot. That first
build writes `Cargo.lock` — commit it, since CI runs `--locked`.

### Verify

```sh
cargo run -- version   # <your-project> 0.1.0 (commit unknown)
cargo run -- config    # the merged configuration, as YAML
curl localhost:8080/health
```

## The wizard

| Question | Choices | What it changes |
| --- | --- | --- |
| Project name | — | Package, binary, env-var prefix (`MY_SVC__…`) |
| Description | — | `Cargo.toml`, the clap `--help` line, both READMEs |
| HTTP server | yes / no | `crates/server`, the `run api` shape, `/health` and `/ready` |
| License | MIT / Apache-2.0 / both / none | `LICENSE-*` and the `license` field |
| CI | GitHub / GitLab / none | `.github/workflows/` or `.gitlab-ci.yml` |
| Dockerfile | yes / no | Multi-stage `cargo-chef` build, `.dockerignore`, image push job |
| Git hooks | yes / no | `.githooks/pre-commit` — `fmt` + `clippy` |
| CLAUDE.md | yes / no | `CLAUDE.md` and `.claude/settings.json` |

## What you get

**A runtime that supervises itself.** `run api`, `run workers`, `run all` — one
variant per deployment shape, so a pod names exactly what it is. `run` on its own
is refused: a pod that meant `workers` and got everything would quietly run a
second copy of every other surface. One `CancellationToken` reaches every
surface, and whichever comes first — a signal or the first surface to return —
cancels the rest, so the process never lingers half-alive.

**Domain in crates, composition in `src/`.** `crates/server` owns the listener,
the probes and its own `ServerConfig`; the binary only names that type in its
global config, so a section and the code it configures never drift apart.

**Errors** — `anyhow` in the binary, `thiserror` in the crates. The binary wants
one error type and a context chain; a crate's caller wants variants it can match
on.

**Config** — serde defaults, then `./config.yaml`, then `MY_SVC__SECTION__FIELD`
env vars, in that order. Every field has a default, so `config.example.yaml`
ships fully commented out — a file that repeats the defaults hides which settings
a deployment actually depends on. `deny_unknown_fields` throughout, so a typo
fails the process at boot instead of doing nothing, and `<binary> config` prints
what the three layers merged to.

**Lints** — deny-level `unwrap_used`, `expect_used`, `panic`,
`indexing_slicing`, `cast_possible_truncation`, `wildcard_imports` and friends,
declared once in `[workspace.lints]` and inherited by every crate. `missing_docs`
warns and CI runs rustdoc under `-D warnings`, so every public item carries a doc
comment.

**CI** — `fmt`, `clippy`, `doc` and `test` run concurrently, so wall-clock is the
slowest single check rather than their sum. `cargo audit` runs on every lockfile
change and weekly besides, because an advisory lands against code that has not
changed. Dependency caches are keyed on the toolchain rather than the lockfile
and are saved even when a job fails — content-addressed entries stay valid, so a
red build still leaves the next one warm.

**Container** — a `cargo-chef` multi-stage build, so editing `src/` never
recompiles the dependency tree, on top of a non-root `debian:bookworm-slim`.

## Working on the template

```sh
# generate into a scratch directory and check it
cargo generate --path . --name probe --silent --vcs none \
  -d description="Probe" -d server=true -d license=MIT \
  -d ci=github -d docker=true -d githooks=true -d claude=true

cd probe && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

Everything the generator needs lives in `template/`: `cargo-generate.toml` holds
the questions, `post-script.rhai` drops the files an answer turned off, and the
rest is the skeleton. Keeping the repo root out of it is what lets this README
and the CI below live here — cargo-generate finds `template/` on its own,
because it is the only folder holding a `cargo-generate.toml`. Files under `.github/workflows/` are copied verbatim rather
than rendered — GitHub Actions expressions use `${{ … }}`, the same delimiters
liquid does.

CI generates both shapes the wizard can produce and runs on each exactly what
that project's own CI would, plus a `docker build`.

Re-record the demo with `vhs assets/demo.tape`.

## License

MIT.
