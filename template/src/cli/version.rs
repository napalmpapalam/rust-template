use anyhow::Result;
use clap::Args;

use crate::version;

/// `version` — print the build's version line.
#[derive(Debug, PartialEq, Args)]
pub struct VersionCmd {}

impl VersionCmd {
    /// Prints the version line.
    pub fn execute(self) -> Result<()> {
        println!("{}", version::version_string());
        Ok(())
    }
}
