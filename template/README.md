# {{project-name}}

{{description}}

## Quick start

```sh
cp config.example.yaml config.yaml
cargo run -- run
```
{% if githooks %}
Turn the hooks on once, so `fmt` and `clippy` run before every commit:

```sh
git config core.hooksPath .githooks
```
{% endif %}
## Commands

| Command | Does |
| --- | --- |
| `{{project-name}} run` | {% if server %}Serves until Ctrl-C or SIGTERM, then drains.{% else %}Runs until Ctrl-C or SIGTERM.{% endif %} |
| `{{project-name}} config` | Prints the effective configuration as YAML. |
| `{{project-name}} version` | Prints version and git commit. |

`--config <path>` overrides where the config file is read from.
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
{% endif %}```

`{{crate_name | upcase}}_CONFIG` names the file when `--config` is absent.
Unknown keys are refused at boot, so a typo fails loudly instead of doing
nothing. See [config.example.yaml](config.example.yaml) for every knob.

## Development

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
```

Those four are exactly what CI runs, in parallel. It also runs `cargo audit`
against the dependency tree, weekly as well as on every lockfile change.
{% if docker %}
## Container

```sh
docker build --build-arg GIT_COMMIT="$(git rev-parse --short HEAD)" -t {{project-name}} .
docker run --rm{% if server %} -p 8080:8080{% endif %} {{project-name}}
```
{% endif %}{% if license != "none" %}
## License

{{license}}.
{% endif %}
