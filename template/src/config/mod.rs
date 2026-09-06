//! The effective configuration: serde defaults, then a YAML file, then env vars.

mod load;
mod log;
mod worker;

use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
{% if server %}
use {{crate_name}}_server::ServerConfig;
{% endif %}
pub use self::log::{LogConfig, LogFormat};
pub use self::worker::WorkerConfig;

/// Env var naming the config file, when the `--config` flag is absent.
const CONFIG_PATH_ENV: &str = "{{crate_name | upcase}}_CONFIG";

/// Prefix every override carries: `{{crate_name | upcase}}__LOG__FORMAT=json`.
const ENV_PREFIX: &str = "{{crate_name | upcase}}";

/// Every configurable knob, merged from all sources.
///
/// A section lives in the crate that owns what it configures; this only names
/// them.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields, default)]
pub struct Config {
    /// Logging section.
    pub log: LogConfig,
{% if server %}    /// Where the HTTP server binds.
    pub server: ServerConfig,
{% endif %}    /// How the background workers are paced.
    pub worker: WorkerConfig,
}

impl Config {
    /// Loads the configuration from the path [`Self::resolve_path`] picks.
    ///
    /// # Errors
    ///
    /// If the file exists but is malformed, or a value fails to deserialize.
    pub fn load(cli_path: Option<&Path>) -> Result<Self> {
        load::load(Some(&Self::resolve_path(cli_path)), ENV_PREFIX)
    }

    /// Returns the config path: the `--config` flag, else the env var, else
    /// `./config.yaml`.
    #[must_use]
    pub fn resolve_path(cli_path: Option<&Path>) -> PathBuf {
        cli_path
            .map(Path::to_path_buf)
            .or_else(|| std::env::var_os(CONFIG_PATH_ENV).map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("config.yaml"))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{Config, ENV_PREFIX, load};

    use std::collections::HashMap;

    /// Loads a `Config` from env vars alone, with no file in the way.
    fn from_env(vars: &[(&str, &str)]) -> anyhow::Result<Config> {
        let vars: HashMap<String, String> = vars
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect();

        load::load_with_env(None, ENV_PREFIX, Some(vars))
    }

    #[test]
    fn the_shipped_defaults_load() {
        let config = from_env(&[]).unwrap();

        assert_eq!(config.log.filter, Config::default().log.filter);
    }

    #[test]
    fn env_overrides_a_nested_field() {
        let config = from_env(&[("{{crate_name | upcase}}__LOG__FILTER", "debug")]).unwrap();

        assert_eq!(config.log.filter, "debug");
    }

    #[test]
    fn a_duration_reads_as_a_human_writes_it() {
        let config = from_env(&[("{{crate_name | upcase}}__WORKER__INTERVAL", "90s")]).unwrap();

        assert_eq!(config.worker.interval.as_secs(), 90);
    }

    #[test]
    fn an_unknown_field_is_refused() {
        // `deny_unknown_fields` is what turns a typo into a boot failure
        // instead of a setting that silently does nothing.
        assert!(from_env(&[("{{crate_name | upcase}}__LGO__FILTER", "debug")]).is_err());
    }
}
