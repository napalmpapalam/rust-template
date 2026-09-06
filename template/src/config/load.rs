//! Layered loading: serde defaults, then the YAML file, then env vars.
//!
//! Env vars use a double underscore both after the prefix and as the nesting
//! separator, so `{{crate_name | upcase}}__LOG__FORMAT` overrides `log.format`.

use std::{collections::HashMap, path::Path};

use anyhow::{Context as _, Result};
use serde::de::DeserializeOwned;

/// Loads `T` from an optional YAML file and the prefixed env vars.
///
/// # Errors
///
/// A missing file is fine; a malformed one, or a value that fails to
/// deserialize, is not.
pub(super) fn load<T: DeserializeOwned>(path: Option<&Path>, env_prefix: &str) -> Result<T> {
    load_with_env(path, env_prefix, None)
}

/// Same as [`load`], with an injectable env source so tests stay hermetic.
pub(super) fn load_with_env<T: DeserializeOwned>(
    path: Option<&Path>,
    env_prefix: &str,
    env_override: Option<HashMap<String, String>>,
) -> Result<T> {
    let env = config::Environment::with_prefix(env_prefix)
        .prefix_separator("__")
        .separator("__")
        .try_parsing(true)
        .source(env_override);

    let mut builder = config::Config::builder();
    if let Some(path) = path {
        let file = config::File::from(path)
            .format(config::FileFormat::Yaml)
            .required(false);
        builder = builder.add_source(file);
    }

    builder
        .add_source(env)
        .build()
        .context("merging configuration sources")?
        .try_deserialize()
        .context("deserializing merged configuration")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{HashMap, Path, load_with_env};

    use std::io::Write as _;

    use serde::Deserialize;

    #[derive(Debug, Default, Deserialize, PartialEq)]
    #[serde(rename_all = "snake_case", deny_unknown_fields, default)]
    struct TestConfig {
        server: ServerSection,
        name: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "snake_case", deny_unknown_fields, default)]
    struct ServerSection {
        host: String,
        port: u16,
    }

    impl Default for ServerSection {
        fn default() -> Self {
            Self {
                host: "0.0.0.0".into(),
                port: 8080,
            }
        }
    }

    fn env(vars: &[(&str, &str)]) -> Option<HashMap<String, String>> {
        Some(
            vars.iter()
                .map(|(key, value)| ((*key).into(), (*value).into()))
                .collect(),
        )
    }

    #[test]
    fn defaults_apply_without_a_file_or_env() {
        let cfg: TestConfig = load_with_env(None, "TEST", env(&[])).unwrap();

        assert_eq!(cfg, TestConfig::default());
    }

    #[test]
    fn a_missing_file_is_not_an_error() {
        let path = Path::new("/nonexistent/config-load-test.yaml");
        let cfg: TestConfig = load_with_env(Some(path), "TEST", env(&[])).unwrap();

        assert_eq!(cfg, TestConfig::default());
    }

    #[test]
    fn env_overrides_a_nested_field() {
        let vars = env(&[("TEST__SERVER__PORT", "9999"), ("TEST__NAME", "from-env")]);
        let cfg: TestConfig = load_with_env(None, "TEST", vars).unwrap();

        assert_eq!(cfg.server.port, 9999);
        assert_eq!(cfg.name, "from-env");
        assert_eq!(cfg.server.host, "0.0.0.0");
    }

    #[test]
    fn the_file_applies_and_env_wins_over_it() {
        let path = std::env::temp_dir().join("config-load-precedence-test.yaml");
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "name: from-file\nserver:\n  port: 1111").unwrap();

        let vars = env(&[("TEST__SERVER__PORT", "2222")]);
        let cfg: TestConfig = load_with_env(Some(&path), "TEST", vars).unwrap();
        std::fs::remove_file(&path).ok();

        assert_eq!(cfg.name, "from-file");
        assert_eq!(cfg.server.port, 2222);
    }

    #[test]
    fn a_value_of_the_wrong_type_is_an_error() {
        let vars = env(&[("TEST__SERVER__PORT", "not-a-port")]);

        assert!(load_with_env::<TestConfig>(None, "TEST", vars).is_err());
    }

    #[test]
    fn an_unknown_field_in_the_file_is_an_error() {
        let path = std::env::temp_dir().join("config-load-unknown-field-test.yaml");
        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(file, "typo_field: oops").unwrap();

        let result = load_with_env::<TestConfig>(Some(&path), "TEST", env(&[]));
        std::fs::remove_file(&path).ok();

        assert!(result.is_err());
    }
}
