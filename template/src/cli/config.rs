use std::path::Path;

use anyhow::{Context as _, Result};
use clap::Args;

use crate::config::Config;

/// `config` — print the merged configuration as YAML.
#[derive(Debug, PartialEq, Args)]
pub struct ConfigCmd {}

impl ConfigCmd {
    /// Prints where the file was looked for, then the effective values.
    pub fn execute(self, path: &Path, config: &Config) -> Result<()> {
        if path.exists() {
            println!("# config file: {}", path.display());
        } else {
            println!(
                "# config file: {} (not found — defaults + env only)",
                path.display()
            );
        }

        let yaml = serde_yaml::to_string(config).context("serializing configuration to YAML")?;
        print!("{yaml}");

        Ok(())
    }
}
