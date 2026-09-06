# {{project-name}}

{{description}}

## Prerequisites

- **Rust 1.90+** — [rustup.rs](https://rustup.rs)
{% if docker %}- **Docker** — for the container build only, [docs.docker.com](https://docs.docker.com/get-docker/)
{% endif %}
## Quick start

```sh
cargo run -- config     # the merged configuration, as YAML
cargo run -- run all    # every surface, until Ctrl-C or SIGTERM
```

Every setting has a default, so nothing needs configuring to boot. Copy
[config.example.yaml](config.example.yaml) to `config.yaml` to change one.
{% if server %}
### Verify

```sh
curl localhost:8080/health   # 200
```
{% endif %}{% if githooks %}
### Hooks

Turn them on once, so `fmt` and `clippy` run before every commit:

```sh
git config core.hooksPath .githooks
```
{% endif %}
## Commands

| Command | Does |
| --- | --- |
{% if server %}| `{{project-name}} run api` | Serves the HTTP API and nothing else. |
{% endif %}| `{{project-name}} run workers` | Runs the background workers and nothing else. |
| `{{project-name}} run all` | Runs everything in one process. |
| `{{project-name}} config` | Prints the effective configuration as YAML. |
| `{{project-name}} version` | Prints version and git commit. |

`--config <path>` overrides where the config file is read from.

`run` on its own is not a command: a pod that meant `workers` and got everything
would quietly run a second copy of every other surface. Whichever comes first — a
signal, or the first surface to return — stops the rest, so the process never
lingers half-alive.
{% if server %}
## Routes

| Route | Answers |
| --- | --- |
| `GET /health` | `200` for as long as the process is up. |
| `GET /ready` | `200` once the service can take traffic. |
{% endif %}
## Configuration

Three layers, each overriding the one before: **serde defaults**, then
**`./config.yaml`**, then **environment variables**.

Env vars carry the prefix `{{crate_name | upcase}}__`, with `__` as the nesting
separator:

```sh
{{crate_name | upcase}}__LOG__FORMAT=json
{% if server %}{{crate_name | upcase}}__SERVER__BIND_ADDR=0.0.0.0:9000
{% endif %}{{crate_name | upcase}}__WORKER__INTERVAL=5m
```

`{{crate_name | upcase}}_CONFIG` names the file when `--config` is absent.
Unknown keys are refused at boot, so a typo fails loudly instead of doing
nothing.

## Layout

| Path | Role |
| --- | --- |
{% if server %}| `crates/server` | The HTTP surface: listener, probes, `ServerConfig`, shutdown. |
{% endif %}| `src/main.rs` | Thin entry point; everything testable lives in the library. |
| `src/cli/` | Argument parsing and one module per subcommand. `run/serve.rs` decides which surfaces a shape starts. |
| `src/config/` | One file per section. Loading order and env-var mapping in `load.rs`. |
| `src/worker.rs` | An example background loop. Replace the body; keep the shape. |
| `src/signal.rs` | Ctrl-C and SIGTERM, the only things that end the process. |

A section lives in the crate that owns what it configures{% if server %} — `ServerConfig` in
`crates/server`{% endif %}, so it never drifts from the code it configures.

## Development

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
```
{% if ci != "none" %}
Those four are exactly what CI runs, in parallel. It also runs `cargo audit`
against the dependency tree, weekly as well as on every lockfile change.
{% endif %}{% if docker %}
## Container

```sh
docker build --build-arg GIT_COMMIT="$(git rev-parse --short HEAD)" -t {{project-name}} .
docker run --rm{% if server %} -p 8080:8080{% endif %} {{project-name}}
```
{% endif %}{% if license != "none" %}
## License

{{license}}.
{% endif %}
