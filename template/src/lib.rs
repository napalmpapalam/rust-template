//! {{project-name}} — {{description}}

use std::process::ExitCode;

mod cli;
mod config;
mod logging;
{% if server %}mod server;
mod status;
{% endif %}mod version;

/// Parses argv, dispatches, and maps the outcome to an exit code.
#[must_use]
pub fn run() -> ExitCode {
    cli::main()
}
