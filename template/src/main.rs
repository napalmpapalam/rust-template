//! Thin entry point — everything testable lives in the library.

use std::process::ExitCode;

fn main() -> ExitCode {
    {{crate_name}}::run()
}
