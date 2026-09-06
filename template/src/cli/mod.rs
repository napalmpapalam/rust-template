//! Argument parsing, and one module per subcommand.

mod config;
mod run;
mod version;

use std::{
    future::Future,
    path::{Path, PathBuf},
    process::ExitCode,
};

use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

use crate::{config::Config, logging};

pub use self::{config::ConfigCmd, run::RunCmd, version::VersionCmd};

/// {{description}}
#[derive(Debug, PartialEq, Parser)]
#[command(name = "{{project-name}}")]
pub struct Cli {
    /// Path to the YAML config file (default: ./config.yaml).
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Command to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands.
#[derive(Debug, PartialEq, Subcommand)]
pub enum Command {
    /// Print version information.
    Version(VersionCmd),

    /// Run the service.
    Run(RunCmd),

    /// Print the effective configuration (defaults, file, and env merged).
    Config(ConfigCmd),
}

/// Parses argv, dispatches, and maps the outcome to an exit code.
pub fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        // clap prints help and version to stdout, usage errors to stderr.
        Err(err) => {
            let _ = err.print();
            return if err.use_stderr() {
                ExitCode::from(2)
            } else {
                ExitCode::SUCCESS
            };
        }
    };

    // `{:?}` on an anyhow error renders the whole context chain, and the
    // backtrace when RUST_BACKTRACE is set.
    execute(cli).map_or_else(
        |err| {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        },
        |()| ExitCode::SUCCESS,
    )
}

/// Dispatches one parsed command.
pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Version(cmd) => cmd.execute(),
        Command::Config(cmd) => {
            let path = Config::resolve_path(cli.config.as_deref());
            let config = Config::load(cli.config.as_deref()).context("loading configuration")?;
            cmd.execute(&path, &config)
        }
        Command::Run(cmd) => {
            block_on_with_config(cli.config.as_deref(), |config| cmd.execute(config))
        }
    }
}

/// Loads the config, starts logging, then drives `run` to completion.
fn block_on_with_config<F, Fut>(config_path: Option<&Path>, run: F) -> Result<()>
where
    F: FnOnce(Config) -> Fut,
    Fut: Future<Output = Result<()>>,
{
    let config = Config::load(config_path).context("loading configuration")?;
    logging::init(&config.log)?;
    runtime()?.block_on(run(config))
}

/// Builds the multi-threaded tokio runtime the commands run on.
fn runtime() -> Result<Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("building async runtime")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{Cli, Command, ConfigCmd, PathBuf, RunCmd, VersionCmd};

    use clap::Parser as _;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("{{project-name}}").chain(args.iter().copied()))
    }

    #[test]
    fn parses_version_command() {
        assert_eq!(
            parse(&["version"]).unwrap().command,
            Command::Version(VersionCmd {})
        );
    }

    #[test]
    fn parses_config_command() {
        assert_eq!(
            parse(&["config"]).unwrap().command,
            Command::Config(ConfigCmd {})
        );
    }

    #[test]
    fn parses_run_with_a_config_path() {
        let cli = parse(&["run", "--config", "custom.yaml"]).unwrap();

        assert_eq!(cli.config, Some(PathBuf::from("custom.yaml")));
        assert_eq!(cli.command, Command::Run(RunCmd {}));
    }

    #[test]
    fn a_command_is_required() {
        assert!(parse(&[]).is_err());
    }

    #[test]
    fn rejects_a_subcommand_that_does_not_exist() {
        assert!(parse(&["frobnicate"]).is_err());
    }
}
