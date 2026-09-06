# {{project-name}}

{{description}}

## Quick start

```sh
cargo run -- run all
```

Every setting has a default, so no config file is needed to boot. Copy
[config.example.yaml](config.example.yaml) to `config.yaml` when you need to
change one.
{% if githooks %}
Turn the hooks on once, so `fmt` and `clippy` run before every commit:

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

`run` on its own is not a command: a pod that meant `workers` and got everything
would quietly run a second copy of every other surface. `--config <path>`
overrides where the config file is read from.

One `CancellationToken` stops every surface a shape started. Whichever comes
first — a signal, or the first surface to return — cancels the rest, so the
process never lingers half-alive.
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

Each section belongs to whichever crate owns the thing it configures{% if server %} — `ServerConfig` lives in `crates/server` and the binary only names it{% endif %}, so a
section and its code never drift apart.

## Layout

| Path | Role |
| --- | --- |
{% if server %}| `crates/server` | The HTTP surface: listener, probes, `ServerConfig`, shutdown. |
{% endif %}| `src/main.rs` | Thin entry point; everything testable lives in the library. |
| `src/cli/` | Argument parsing and one module per subcommand. `run/serve.rs` is the composition root — it decides which surfaces a shape starts. |
| `src/config/` | One file per section. Loading order and env-var mapping in `load.rs`. |
| `src/worker.rs` | An example background loop. Replace the body; keep the shape. |
| `src/signal.rs` | Ctrl-C and SIGTERM, the only things that end the process. |

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
