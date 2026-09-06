# rust-template

A `cargo generate` template for Rust binaries and services. Answer eight
questions, get a project that compiles, tests and lints on the first run —
workspace lints, layered config, tracing, CI, a Dockerfile and git hooks already
wired together.

![The wizard generating a project](assets/demo.gif)

## Prerequisites

- **Rust 1.90+** — [rustup.rs](https://rustup.rs)
- **cargo-generate** — `cargo install cargo-generate --locked`

## Usage

```sh
cargo generate napalmpapalam/rust-template template
```

> [!NOTE]
> The trailing `template` is the subfolder this repo keeps the skeleton in, so
> the repo root can hold its own README and CI. Leave it off and you get a
> project with a `template/` directory inside it.

Then:

```sh
cd <your-project>
cp config.example.yaml config.yaml
cargo run -- run
```

That first build writes `Cargo.lock` — commit it, since CI runs `--locked`.

### Verify

```sh
cargo run -- version   # <your-project> 0.1.0 (commit unknown)
cargo run -- config    # the merged configuration, as YAML
```

## The wizard

| Question | Choices | What it changes |
| --- | --- | --- |
| Project name | — | Package, binary, env-var prefix (`MY_SVC__…`), default service name |
| Description | — | `Cargo.toml`, the clap `--help` line, both READMEs |
| Layout | `workspace` / `single` | Whether `crates/core` ships alongside `src/` |
| HTTP server | yes / no | axum, `/health` and `/ready`, graceful shutdown on SIGTERM |
| License | MIT / Apache-2.0 / both / none | `LICENSE-*` and the `license` field |
| CI | GitHub / GitLab / none | `.github/workflows/` or `.gitlab-ci.yml` |
| Dockerfile | yes / no | Multi-stage `cargo-chef` build, `.dockerignore`, image push job |
| Git hooks | yes / no | `.githooks/pre-commit` — `fmt` + `clippy` |
| CLAUDE.md | yes / no | `CLAUDE.md`, `docs/design/`, `.claude/settings.json` |

## What you get

**Errors** — `anyhow` in the binary, `thiserror` in the crates. The binary wants
one error type and a context chain; a crate's caller wants variants it can match
on.

**Config** — serde defaults, then `./config.yaml`, then `MY_SVC__SECTION__FIELD`
env vars, in that order. `deny_unknown_fields` throughout, so a typo fails the
process at boot instead of silently doing nothing. `<binary> config` prints what
the three layers actually merged to.

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
red build still leaves the next one warm. Dependabot groups its updates into one
PR a week.

**Container** — a `cargo-chef` multi-stage build, so editing `src/` never
recompiles the dependency tree, on top of a non-root `debian:bookworm-slim`.

**Tracing** — `tracing-subscriber` with an `EnvFilter`, text for humans and
newline-delimited JSON for collectors, picked in config.

## Working on the template

```sh
# generate into a scratch directory and check it
cargo generate --path . template --name probe --silent --vcs none \
  -d description="Probe" -d layout=workspace -d server=true -d license=MIT \
  -d ci=github -d docker=true -d githooks=true -d claude=true

cd probe && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

Everything the generator needs lives in `template/`: `cargo-generate.toml` holds
the questions, `post-script.rhai` drops the files an answer turned off, and the
rest is the skeleton. Files under `.github/workflows/` are copied verbatim rather
than rendered — GitHub Actions expressions use `${{ … }}`, the same delimiters
liquid does.

Re-record the demo with `vhs assets/demo.tape`.

## License

MIT.
